use axum::{extract::Query, response::Html, routing::get, Extension, Router};

mod auth;

use base64::Engine;
use biscuit_auth::{builder::ToAnyParam, macros::authorizer, Biscuit, PublicKey};
use rand::Rng;
use serde::Deserialize;
use std::{env, fmt::Display, fs::DirEntry, time::SystemTime};
use tower_http::services::{ServeDir, ServeFile};

use auth::{error_body, run_auth, ParseBiscuit};

#[tokio::main]
async fn main() {
    let public_key = PublicKey::from_bytes_hex(&env::var("BISCUIT_PUBLIC_KEY").unwrap()).unwrap();
    let protected = Router::new()
        .route("/dog", get(dog_handler))
        .route_layer(ParseBiscuit::new(public_key));
    let app = Router::new()
        .nest_service("/", ServeFile::new("assets/public/index.html"))
        .nest_service("/index.js", ServeFile::new("assets/public/index.js"))
        .nest_service("/assets", ServeDir::new("assets/public"))
        .nest("/protected", protected);

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Doggo {
    Puna,
    Nix,
}

impl Display for Doggo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Doggo::Puna => write!(f, "puna"),
            Doggo::Nix => write!(f, "nix"),
        }
    }
}

impl ToAnyParam for Doggo {
    fn to_any_param(&self) -> biscuit_auth::builder::AnyParam {
        biscuit_auth::builder::AnyParam::Term(biscuit_auth::builder::Term::Str(self.to_string()))
    }
}

#[derive(Deserialize)]
pub struct GetQuery {
    dog: Doggo,
}

fn get_pictures(dog: Doggo) -> Result<Vec<u8>, std::io::Error> {
    let dir_path = match dog {
        Doggo::Puna => "assets/images/puna",
        Doggo::Nix => "assets/images/nix",
    };

    let entries: Vec<Result<DirEntry, _>> = std::fs::read_dir(dir_path)?.collect();
    let entries: Vec<DirEntry> = entries.into_iter().collect::<Result<_, _>>()?;

    let path = entries
        .get(rand::thread_rng().gen_range(0..entries.len()))
        .unwrap()
        .path();

    println!("Reading {path:?}");
    std::fs::read(path)
}

async fn dog_handler(
    Extension(biscuit): Extension<Biscuit>,
    Query(query): Query<GetQuery>,
) -> Result<Html<String>, Html<String>> {
    let snapshot = run_auth(
        &biscuit,
        authorizer!(
            r#"
    dog({dog});
    time({now});
    allow if user("clementd");
    allow if right({dog}, "read");
        "#,
            now = SystemTime::now(),
            dog = query.dog
        ),
    )?;

    let bytes = get_pictures(query.dog).map_err(|_| error_body(&snapshot))?;

    Ok(Html(format!(
        r#"<img src="data:image/jpeg;base64,{}">
        <code><pre>{snapshot}</pre><code>"#,
        base64::prelude::BASE64_STANDARD.encode(bytes)
    )))
}
