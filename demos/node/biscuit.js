import  {Biscuit, Authorizer} from '@biscuit-auth/biscuit-wasm';

export function middleware(pubkey) {
  return function(mkAuthorizer) {
    return function(req, res, next) {
      if(req.query.token) {
        let parsed;
        try {
          let authorizer;
          if(typeof mkAuthorizer === "function") {
            authorizer = mkAuthorizer(req);
          } else {
            authorizer = mkAuthorizer;
          }
          parsed = Biscuit.fromBase64(req.query.token, pubkey);
          authorizer.addToken(parsed);
          const result = authorizer.authorize();
          next();
        } catch(e) {
          console.log(JSON.stringify(e));
          res.status(403).json(e);
        }
      } else {
        res.status(401).send();
      }
    }
  }
}

export function koa(pubkey) {
  return function(mkAuthorizer) {
    return function(ctx, next) {
      if(ctx.query.token) {
        let parsed;
        try {
          let authorizer;
          if(typeof mkAuthorizer === "function") {
            authorizer = mkAuthorizer(ctx);
          } else {
            authorizer = mkAuthorizer;
          }
          parsed = Biscuit.fromBase64(ctx.query.token, pubkey);
          authorizer.addToken(parsed);
          const result = authorizer.authorize();
          next();
        } catch(e) {
          console.log(JSON.stringify(e));
          ctx.status = 403;
          ctx.body = e;
        }
      } else {
        ctx.status = 401;
      }
    }
  }
}
