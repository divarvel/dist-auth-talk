use axum::{
    extract::Request,
    response::{Html, IntoResponse, Response},
};
use biscuit_auth::{error, Authorizer, Biscuit, PublicKey};
use futures_util::future::BoxFuture;
use http::{
    header::{ToStrError, AUTHORIZATION},
    StatusCode,
};
use std::{
    collections::HashMap,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct ParseBiscuit {
    public_key: PublicKey,
}

impl ParseBiscuit {
    pub fn new(public_key: PublicKey) -> ParseBiscuit {
        ParseBiscuit { public_key }
    }
}

impl<S> Layer<S> for ParseBiscuit {
    type Service = ParseBiscuitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ParseBiscuitService {
            inner,
            public_key: self.public_key,
        }
    }
}

#[derive(Clone)]
pub struct ParseBiscuitService<S> {
    inner: S,
    public_key: PublicKey,
}

impl<S> Service<Request> for ParseBiscuitService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let token = parse_token(&request, self.public_key);
        match token {
            Ok(t) => {
                request.extensions_mut().insert(t);
                let future = self.inner.call(request);
                Box::pin(async move {
                    let response: Response = future.await?;
                    Ok(response)
                })
            }
            Err(AuthError::NoAuthorizationHeader) => {
                println!("Missing auth header");
                Box::pin(async move { Ok(StatusCode::UNAUTHORIZED.into_response()) })
            }
            Err(AuthError::InvalidAuthHeader(e)) => {
                println!("Invalid auth header: {e}");
                Box::pin(async move { Ok(StatusCode::FORBIDDEN.into_response()) })
            }
            Err(AuthError::NoBearerToken) => Box::pin(async move {
                println!("Missing bearer token");
                Ok(StatusCode::UNAUTHORIZED.into_response())
            }),
            Err(AuthError::BiscuitError(e)) => Box::pin(async move {
                println!("Biscuit parsing error: {e}");
                Ok(StatusCode::UNAUTHORIZED.into_response())
            }),
        }
    }
}

enum AuthError {
    NoAuthorizationHeader,
    InvalidAuthHeader(ToStrError),
    NoBearerToken,
    BiscuitError(error::Token),
}

fn extract_token_from_headers(request: &Request) -> Result<String, AuthError> {
    Ok(request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AuthError::NoAuthorizationHeader)?
        .to_str()
        .map_err(AuthError::InvalidAuthHeader)?
        .strip_prefix("Bearer ")
        .ok_or(AuthError::NoBearerToken)?
        .to_owned())
}

fn extract_token_from_query_string(request: &Request) -> Result<String, AuthError> {
    let query = request.uri().query().unwrap_or_default();
    let params: HashMap<String, String> = serde_urlencoded::from_str(query).unwrap();
    Ok(params
        .get("token")
        .ok_or(AuthError::NoBearerToken)?
        .to_owned())
}

fn parse_token(request: &Request, public_key: PublicKey) -> Result<Biscuit, AuthError> {
    let token_string = extract_token_from_query_string(request)?;
    Biscuit::from_base64(token_string, public_key).map_err(AuthError::BiscuitError)
}

pub const ERROR_BODY: Html<&'static str> =
    Html(r#"<img src="/assets/poutou.webp" class="error"><p>Forbidden</p>"#);

pub fn run_auth(biscuit: &Biscuit, mut authorizer: Authorizer) -> Result<(), Html<&'static str>> {
    authorizer.add_token(biscuit).map_err(|_| ERROR_BODY)?;
    let result = authorizer.authorize();
    println!(
        "{}",
        authorizer.to_base64_snapshot().map_err(|e| {
            println!("Snapshot error: {e}");
            ERROR_BODY
        })?
    );

    match result {
        Ok(policy_id) => {
            println!("Auth success, policy n°{policy_id}");
            Ok(())
        }
        Err(e) => {
            println!("Auth error, {e}");
            Err(ERROR_BODY)
        }
    }
}
