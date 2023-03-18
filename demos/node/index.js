import express from "express";
import { biscuit, authorizer, KeyPair, Biscuit } from "@biscuit-auth/biscuit-wasm";
import { middleware } from "./biscuit.js";
import { webcrypto } from "node:crypto";
globalThis.crypto = webcrypto;
import {readdir} from "node:fs/promises";
import path from "node:path";

const app = express();
const port = 3000;

const keypair = new KeyPair();
const p = middleware(keypair.getPublicKey());

app.get(
  "/protected/:dog",
  p((req) => authorizer`allow if scope(${req.params.dog}, "read");`),
  (req, res) => {
    if(req.params.dog === 'puna') {
      readdir("./assets/puna").then(files => {
        const picName = files[Math.floor((Math.random()*files.length))];
        res.sendFile(`${picName}`, {
          root: path.resolve("assets/puna")
        });
      }).catch((e) => {
        console.error(e);
        res.send(`${req.params.dog}!`);
      });
    } else {
      res.send(`${req.params.dog}!`);
    }
  }
);

app.listen(port, () => {
  const b = biscuit`
    scope("puna", "read");
  `.build(keypair.getPrivateKey());
  console.log("This token will grant you read access to /protected/puna");
  console.log(b.toBase64());
});
