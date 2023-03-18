import { biscuit, authorizer, KeyPair, Biscuit } from "@biscuit-auth/biscuit-wasm";
import { koa } from "./biscuit.js";
import { webcrypto } from "node:crypto";
globalThis.crypto = webcrypto;
import {readdir} from "node:fs/promises";
import path from "node:path";
import Koa from "koa"
import Router from '@koa/router'
import sendfile from "koa-sendfile"
import { Buffer } from "node:buffer"

const app = new Koa();
const router = new Router();

const keypair = new KeyPair();
const p = koa(keypair.getPublicKey());

const buf = Buffer.from([1,2,3,4]);
const b = biscuit`
    scope("puna", "read", ${new Date()}, ${new Uint8Array(buf)});
  `.build(keypair.getPrivateKey());
console.log("This token will grant you read access to /protected/puna");
console.log(b.toBase64());

router.get('/protected/:dog',
    p(ctx => authorizer`allow if scope(${ctx.params.dog}, "read");`),
    async ctx => {
      if(ctx.params.dog === 'puna') {
        const files = await readdir("./assets/puna");
        const picName = files[Math.floor((Math.random()*files.length))];
        await sendfile(ctx, `assets/puna/${picName}`);
      } else {
        ctx.body = `${ctx.params.dog}!`;
      }
});

app
  .use(router.routes())
  .use(router.allowedMethods());

app.listen(3000);