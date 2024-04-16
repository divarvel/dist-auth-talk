---
title: "Auth & systèmes distribués : ne jetons pas le bébé avec l’auth du bain"
#display-notes: true
#light: true
#ratio43: true
overlay: "@clementd / @gcouprie"
author:
  - name: Clément Delafargue
    desc:
      - Software developer at <a href="https://outscale.com">3DS Outscale</a>
      - <a href="https://framapiaf.org/clementd">@clementd@framapiaf.org</a>
  - name: Geoffroy Couprie
    desc:
      - Apollo GraphQL
      - <a href="https://twitter.com/gcouprie">@gcouprie</a>
      - <a href="https://mastodon.social/@geal">@geal@mastodon.social</a>
---

# What is auth?

::: notes
| overloaded term. gathers two concepts
:::

---

<div style="display: flex;">
  <div style="flex-grow: 1; text-align: center;"><h2>Authentication</h2></div>
  <div style="flex-grow: 1; text-align: center;"><h2>Authorization</h2></div>
</div>

---

<div style="display: flex;">
  <div style="flex-grow: 1; text-align: center;"><h2>Authn</h2></div>
  <div style="flex-grow: 1; text-align: center;"><h2>Authz</h2></div>
</div>

::: notes
| authentication: who are you?
| authorization: what can you do?
| usually authentication is used to determine authorization, but
| that is not always the case
:::

---

# Distributed systems

::: notes
| a distributed system is about running different bits of software
| on different pieces of hardware.
:::

---

:::bigimage
![](./assets/but-why.gif)
:::

---

# [compartimentalize]{.jumbo}

::: notes
| The goal is to create boundaries
| to compartimentalize stuff (hardware failures, resource use,
| teams management, blast radius of a bug / vuln)
:::

---

# Tradeoffs

::: notes
| after a few years of μservices being trendy, we're starting to know tradeoffs
| well.
:::

---

# Latency

::: notes
| in the context of auth, the biggest tradeoff is latency in data updates. you
| usually want a single source of truth with auth, and in a distributed system
| you don't have a single data store, so propagation takes time
:::

---

# Monolith, [[fewer problems]{}]{.incremental}

::: notes
| in terms of solely auth, a monolith is unequivocally better. centralized
| auth is simpler
:::

---

:::bigimage
![](./assets/2001.gif)
:::

::: notes
| if you can afford a monolith (tolerate hardware failure,
| team coupling, etc), then all sorts of problems disappear
:::

---

# kthxbai

::: notes
| for the rest of the talk we will assume that we are in the context
| where a distributed system is required (for whatever reason)
:::

---

# Auth in a distributed system

::: notes
| authorization is a cross-cutting concern. in most cases each entity
| will have to perform authorization on incoming requests.
| it's also often something that you somehow want to manage centrally.
| chasing access rules across a distributed architecture is a nightmare
:::

---

# [Don't panic]{.jumbo}

---

:::bigimage
![](./assets/dsotm.jpeg)
:::


::: notes
| the good thing is that you can choose where you apply decentralization
| what's important is staying aware of the dependencies between services
| and the actual call graph triggered by an incoming request. it will
| give you a good idea of possible failure modes and of the actual
| cost of tradeoffs
:::

---

<h1 style="z-index: 2; background-color: rgba(255,255,255, 0.3);">Centralized</h1>

:::bigimage
![](./assets/panopticon.jpg)
:::

::: notes
| you can still keep auth completely centralized in a distributed system
| every node calls out to the auth service.
| conceptually simple, but the auth service is a SPOF
| some guarantees are still lost
| reliance on operational guarantees from the auth service: see zanzibar
| all in all, it's an okay architecture, even though the auth service
| is a spof: you can limit the scope of the auth service to make it
| more robust and accept this failure mode
:::

---

<h1 style="z-index: 2; background-color: rgba(255,255,255, 0.3);">Decentralized</h1>

:::bigimage
![](./assets/darkest-timeline.webp)
:::

::: notes
| if you want truly autonomous nodes and an auth service spof is
| not possible, then each service decides on its own.
| but how?
:::

---

# challenges

::: notes
more frightening than centralized auth, even though it brings super interesting
properties
:::

---

# Bearer tokens (boring)

- JWT (super common, can be tricky)
- PASETO (JWT without the footguns)
- Roll your own tokens

::: notes
| Since services cannot call out to a central auth service, services need a way
| to trust information. A common way to do that is signed tokens
| JWT / PASETO / custom tokens: payload + signature. It's up to the services
| to actually interpret it. Conventions in the JWT world with well-known claims
:::

---

# Bearer tokens (fancy)

- Macaroons
- Biscuits

::: notes
| Macaroons: caveat system for describing constraints
| Biscuits: embedded logic language for describing constraints & access rules
:::

---

# Why biscuits?

::: incremental
- Public key cryptography
- Offline attenuation
- Structured logic language
:::

::: notes
| Macaroons were deployed at clever cloud early on because we had a use-case for
| offline attenuation, but their reliance on a shared secret prevented a wider-scale use
| also the lack of structure for authorization logic required duplicated logic in
| different tech stacks
:::

---

:::bigimage
![](./assets/bearer-tokens.jpg)
:::

---


# Distributed trust

::: notes
| in this setup, there is no central auth system that you can query
| for fresh information. you have to rely on bearer tokens
:::

---

# Incomplete & Dynamic data

::: notes
| in this setup, a service may carry enough information to make
| auth decisions, so sometimes a cache of external context is required
| this cache can become stale
| bearer tokens themselves can become stale. need for revocation
:::

---

# Mitigating issues

::: notes
| the main issue with distributed auth and bearer tokens is that once
| a token has been created, there is no direct way to invalidate it.
| the only way you can do it is to tell everyone "stop accepting this
| token in particular"
:::

---

# [revocation]{.jumbo}

::: notes
| talking about token revocation could easily fill a 1h slot so we'll
| have to summarize
:::

---

# [do it yesterday[]{.make-alternate}]{.jumbo}

::: notes
| have a basic revocation infra available from day 1, even if it's
| dumb. You'll thank yourself once you have to revoke a token.
| make sure every token is uniquely identifiable so it can be revoked
| without affecting other holders.
:::

---

# Make tokens unique

::: notes
| make sure every token is uniquely identifiable so it can be revoked
| without affecting other holders.
:::

---

# Track tokens

::: notes
| if possible, keep a list of all the tokens you generate, so you can
| revoke them (you might know that a token has leaked without having
| access to the token itself). if tokens have unique ids, you can
| store them instead of the tokens, this way it's not sensitive info
:::

---

# Expiration date

::: notes
| revocation and emission lists can only grow, so they need to be
| pruned at some point. the simplest way to do that is to give every
| token an expiration date, and then store this date in the emission
| and revocation lists. this way, pruning is easy.
:::

---

# [do it yesterday[]{.make-alternate}]{.jumbo}

---

# Access token / refresh token

<pre style="margin-left: auto; margin-right: auto;">
┌───────┐ auth data         ┌────────────────┐
│  user ├──────────────────▲│ token delivery │
└─────┬─┘▼──────────────────┤    service     │
   ▲  │    refresh token    └─────────┬──────┘
   │  │     + access token        ▲   │
   │  │                           │   │
   │  │   refresh token           │   │
   │  └───────────────────────────┘   │
   │                                  │
   └──────────────────────────────────┘
          access token
           + refresh token
</pre>

::: notes
| a good way to reduce tokens lifetime is to separate access tokens
| from refresh tokens. access tokens are bearer tokens with a
| predefinite validity period. refresh tokens are stateful and can
| be exchanged for access tokens. The access token delivery service
| is still a spof, but is not on the hot path for every request, so
| failure is more tolerable.
| adjusting the access token validity period lets you chose a
| compromise between freshness and resilience
:::

---

# Key rotation

::: notes
| change keys regularly, accept both old and new keys during a limited period
:::

---

# [do it yesterday[]{.make-alternate}]{.jumbo}

::: notes
| same as for revocation, this has to be planned from day 1 because
| once you need it, you have to be fast.
:::

---

# Perform regular rotations

::: notes
| the best way to be prepared is to rotate keys regularly and make
| sure nothing breaks.
| rotating keys mandates that every token has an expiration date, so
| that you can retire keys without breaking anything
:::

---

# [do it yesterday[]{.make-alternate}]{.jumbo}

---


# Principle of least privilege

::: notes
| even with revocation in place, there is still latency. so reducing
| the blast radius during that vuln window is always a good thing.
:::

---

# Dedicated execution roles (good)

::: notes
| of course you want people to have access to what they need, so 
| identity-based rules tend to be static.
| a good example is AWS execution roles you can define for lambdas
| where each lambda gets dedicated credentials that are only valid
| for the execution duration
:::

---

# Single purpose tokens on demand (better)

::: notes
| however, if you can deliver tokens crafted specially for the
| operations you're trying to carry out, you avoid a lot of issues
:::

---

# Perf / latency tradeoff

::: notes
| it's not always feasible though, especially with central auth
| systems where creating new roles takes time (eg AWS)
| KMS has a dedicated feature, just for that: creating restricted access tokens
| without bothering with IAM
:::

---

# [✨offline attenuation ✨]{.jumbo}

::: notes
| what if you could generate single-use token on demand, in a hot path, with
| no perf cost?
|
| that's exactly what offline attenuation does
:::

---

# Biscuit

::: incremental
- spec
- implementations
:::

::: notes
| biscuit is a spec (token format, datalog variant syntax & semantics)
| there are various implementations, biscuit-rust is the reference implementation
| several implementations are available (rust + wasm derivatives, haskell, java, go, etc)
:::

---

# Biscuit spec

::: incremental
- token format (payload serialization)
- token format (crypto)
- authorization rules (denotational & operational semantics)
:::

::: notes
| biscuit is a spec, it defines both a token format (crypto and payload), as
| well as a dedicated language used to describe authorization rules.
:::

---

# Biscuit

::: incremental
- public key crypto (default ed25519)  
  (cryptographic agility WIP)
- offline attenuation
- authorization rules
- snapshots
:::

::: notes
| the token itself relies on ed25519 signatures, so it works with keypairs
| a token contains an authority block, signed by the token emitter, but it can
| also contain attenuation blocks, that can be freely added to a token
| (ie without needing the signing key). such blocks can only restrict a token scope
| the token can contain both data and authorization logic
| snapshots can persist the whole context for after-the-fact auditing
:::

---

# Timeline

- init 2018 (offline attenuation & datalog)
- v1 in 2019 (faster crypto & better datalog)
- v2 2022 (boring-er crypto & scoping)
- v3 2023 (third-party blocks & snapshots)
- v4 2024? (more datalog features)

::: notes
| created at clever-cloud, following a successful use of macaroons in a limited context
| geoffroy, since left clever cloud, and clément joined biscuit,
| after having gained significant operational experience with macaroons
:::

---

# April 2023 -> April 2024

- new documentation (<https://doc.biscuitsec.org>)
- more tooling (VScode plugin, LSP)
- CLI features (snapshots, queries, timing info, JSON output)
- biscuit-python
- biscuit-rust 4.0

::: notes
| special thanks to outscale for dedicating time to biscuit
| (not only my time, but also some of my colleagues)
:::

---

# Implementations

- rust (biscuit v3 ref implementation)
- JS (biscuit v3, rust via WebAssembly)
- Haskell (biscuit v3)
- Java (biscuit v2, v3 in progress)
- Go (biscuit v2)
- python (in progress)
- .Net (in progress)

---

# Tooling

- CLI
- web components
- editor support (vscode, helix, tree-sitter, lsp)

---

# Used in prod

- Clever Cloud
- SpaceAndTime
- nixbuild.net

---

# Biscuit: token

```{=html}
<bc-datalog-editor>
<code>
// datalog payload: data
user("01GXHB9ZFJEAWF9QN7WEDX6E1E");
// datalog payload: auth rules
check if time($time), $time < 2023-03-31T00:10:46Z;
</code>
</bc-datalog-editor>
```

::: notes
| here is an example of a token. it contains a regular payload, but also
| auth rules. the token itself can carry auth logic, and it's crucial in some
| architectures. auth logic in a token is purely restrictive: it can only deny
| access by failing, but is not enough in itself
| the token itself is serialized as a binary blob through protobuf and is usually
| base64 encoded when sent over the wire
:::

---

# Biscuit: token

---

```{=html}
<bc-token-generator privateKey="7acc936a92ceaeaefb1e21483877ba0b603f2b9bcdcf6422a19e9f488cc16a75">
<code class="block">
// datalog payload: data
user("01GXHB9ZFJEAWF9QN7WEDX6E1E");
// datalog payload: auth rules
check if time($time), $time < 2023-03-31T00:10:46Z;
</code>
</bc-token-generator>
```

---

# Biscuit: auth rules

---

```{=html}
<bc-datalog-editor>
<code>
// context provided by the server
time(2023-03-31T00:00:00Z);
resource("file1");
operation("read");

// rules provided by the server
right("01GXHB9ZFJEAWF9QN7WEDX6E1E", "file1", "read");
allow if user($user),
         resource($r), 
         operation($op),
         right($user, $r, $op);
</code>
</bc-datalog-editor>
```

::: notes
| the token itself can carry rules, but ultimately it's up to the receiving
| agent to decide whether or not to authorize. That is done with `allow if`
| usually the receiving agent will provide context (what is the operation
| that is currently attempted, on which resource, etc), more static rules
| like ACLs or capabilities, and finally a condition for allowing requests.
| the biscuit library then combines the token with the server policies, and
| computes the result of authorization policies
:::

---

# Biscuit: offline attenuation

---

```{=html}
<bc-datalog-playground showBlocks="true">
<pre><code class="block">
admin(true);
</code></pre>
<pre><code class="authorizer">
allow if admin(true);
</code></pre>
</bc-datalog-playground>
```

::: notes
| here we have a token granting admin rights to its holder
| (as defined in the authorizer block)
| the request is authorized
:::

---

```{=html}
<bc-datalog-playground showBlocks="true">
<pre><code class="block">
admin(true);
</code></pre>
<pre><code class="block">
check if source_ip("127.0.0.1");
</code></pre>
<pre><code class="authorizer">
allow if admin(true);
</code></pre>
</bc-datalog-playground>
```

::: notes
| the biscuit crypto model allows a token holder to craft a new token by
| appending a block to an existing one. once added, blocks can not be removed,
| altered or reordered. The evaluation model guarantees that a block can only
| restrict a token's scope
| here the token can only be used with a local IP address
:::

---

```{=html}
<bc-datalog-playground showBlocks="true">
<pre><code class="block">
admin(true);
</code></pre>
<pre><code class="block">
check if source_ip("127.0.0.1");
</code></pre>
<pre><code class="authorizer">
source_ip("127.0.0.1");
allow if admin(true);
</code></pre>
</bc-datalog-playground>
```

---

# Biscuit: snapshots

---

```{=html}
<bc-snapshot-printer snapshot="CgkI6AcQZBjAhD0Q3yYauwEIBBIFZmlsZTISBDEyMzQSBWZpbGUxIkQQAxoJCgcIChIDGIEIGg0KCwgEEgMYgggSAhgAKiYKJAoCCBsSBggFEgIIBRoWCgQKAggFCggKBiCAxZihBgoEGgIIACoQEAMaDAoKCAUSBiDbsZWhBjIVChEKAggbEgsIBBIDGIAIEgIYABAAOh4KAhAAEgkKBwgKEgMYgQgSDQoLCAQSAxiCCBICGAA6EgoCCgASDAoKCAUSBiDbsZWhBkAA" class="centered">
</bc-snapshot-printer>  
```

---

# Demo

<https://github.com/divarvel/dist-auth-talk/tree/main/demos>

---

# Distributed auth patterns

::: notes
| now for some patterns. there are various ways to implement
| distributed auth. often you'll still require a bit of
| centralization. the question is how / where you do that and
| how much it affects other services
:::

---

# Auth gateway

<pre style="margin-left: auto; margin-right: auto;">
            trusted network
           ┌────────────────────────────────────┐
           │                                    │
           │                     services       │
           │                      ┌───┐         │
         ┌─┴┐      trusted        │   │  trusted│
         │  ├─────────────────────► 1 ├─────┐   │
         │g │                     └───┘     │   │
         │a │                               │   │
untrusted│t │                     ┌───┐     │   │
────────►│e │      trusted        │   │     │   │
         │w ├─────────────────────► 2 │◄────┘   │
         │a │                     └───┘         │
         │y │                                   │
         │  │                     ┌───┐         │
         │  │      trusted        │   │         │
         │  ├─────────────────────► 3 │         │
         └─┬┘                     └───┘         │
           │                                    │
           └────────────────────────────────────┘
</pre>

::: notes
| you usually already have an ingress where all incoming traffic
| is directed. so if you already have a spof, you can make it
| handle auth concerns without creating a new one.
| it is a very interesting pattern where a single incoming request
| can trigger several internal cross-service requests: the auth
| exposed to the outside can be completely separate from what's
| used between services internally. eg a stateful session exposed
| to users, and different mechanisms internally (eg trusted network,
| or bearer tokens)
:::

---

# Challenges

::: notes
| quite common and convenient. not perfect though
:::

---

# Complexity @ ingress

::: notes
| makes the ingress logic more complicated, so it can affect its
| robustness.
| just half the story: how are internal calls authorized?
:::

---

# Coarse-grained model

::: notes
| does the
| gateway need to know about every service internals so it can
| perform authorization. in practice this nudges into very coarse
| grained authz rules: rules that the gateway can check itself
| without too much knowledge of service internals.
| it's still a great solution for authentication if you let 
| services themselves perform authorization
:::

---

# Confused deputy attacks

::: notes
| services can still be tricked into performing operations that
| they can do in theory but should be forbidden in a given
| context.
| especially in multi-tenant system, the service-level restrictions
| don't prevent a tenant from accessing data from another tenant
:::

---

# Internal calls with request-level restrictions

<pre style="margin-left: auto; margin-right: auto;">
              incoming credentials
             ┌─────────────┐
             │             │
             │   services  │
incoming     │   ┌────┐    │
─────────────┴──►│    │    │
request          │ 1  ├────▼┐
                 └────┘     │
                            │ authorized
                 ┌────┐     │ as incoming
                 │    │     │    request
                 │ 2  │◄────┘
                 └────┘
</pre>

::: notes
| the first service requires a token provided by the auth gateway
| and then passes it to subsequent calls. each service uses this
| token to perform auth.
| we get back request scoping and tenant isolation
:::

---

# Internal calls with offline attenuation

<pre style="margin-left: auto; margin-right: auto;">
              incoming credentials
             ┌─────────────┐
             │             │
             │   services  │
incoming     │   ┌────┐    │
─────────────┴──►│    │  ┌─┴───┐
request          │ 1  ├──┤magic│
                 └────┘  └─┬───┘
                           │ authorized as incoming
                 ┌────┐    │ request
                 │    │    │ (+ request-specific
                 │ 2  │◄───┘    restrictions)
                 └────┘
</pre>

::: notes
| same as before, but each service attenuates the token before
| passing it further. this way, even if a service is compromised,
| only dedicated tokens are leaked, so the blast radius is limited
| only feasible with offline attenuation, since it doesn't require
| crafting new credentials / roles. this can be done at a
| per-request/per-service level.
:::


---

# Mixing trust domains

<pre style="margin-left: auto; margin-right: auto;">
                                  services
                                   ┌───┐
          ┌──┐                     │   │
          │  ├─────────────────────► 1 │
          │g │  ▲                  └───┘
          │a ├──┘inject rights
access    │t │                     ┌───┐
 ────────►│e │                     │   │
token     │w ├─────────────────────► 2 │
          │a │  ▲                  └───┘
          │y ├──┘
          │  │  inject rights      ┌───┐
          │  │                     │   │
          │  ├─────────────────────► 3 │
          └─┬┘  ▲                  └───┘
            └───┘
                inject rights
</pre>

---

# Token delivery service

<pre style="margin-left: auto; margin-right: auto;">
┌────────┐ auth data    ┌────────┐
│        ├─────────────►│token   │
│  user  │              │delivery│
│        │◄─────────────┤service │
└─┬────┬─┘  token(s)    └────────┘
  │    │
  │    │
  │    │                  ┌───────┐
  │    └─────────────────►│service│
  │                       │  1    │
  │                       └───────┘
  │
  │                       ┌───────┐
  └──────────────────────►│service│
                          │  2    │
                          └───────┘
</pre>

::: notes
| this one is interesting if you can directly expose services
| the user logs in at the token service, and gets access tokens
| they can use to access services directly. i used it at fretlink
| and it was super robust.
:::

---

# Out of the hot path

::: notes
the auth service is called once and then services are accessed directly
:::

---

# Challenges

::: notes
| the only truly decentralized solution
:::

---

# Internals are more exposed

::: notes
| stuff that is usually hidden behind the gateway is now exposed, so it can
| be more brittle, and there is a bigger attack surface
:::

---

# Recap

::: incremental
- centralized auth is simpler…
- but SPOF-y
- decentralized auth is doable…
- but plan for REVOCATION
- Least Privilege Principle
- (offline attenuation is great)
- Eat biscuits
:::

---

# Biscuit next steps<br><small>(contributors welcome)</small>
  
- vscode support (LSP integration)
- more powerful datalog features (laziness, nested structures)
- ECDSA support
- webauthn support

---

<div style="height: 100vh; display: flex; flex-direction: column; justify-content: space-evenly; align-items: center;">
## [biscuitsec.org](https://biscuitsec.org)
## [doc.biscuitsec.org](https://doc.biscuitsec.org)
## [github.com/biscuit-auth](https://github.com/biscuit-auth)
## [#biscuit-auth:matrix.org](https://matrix.to/#/#biscuit-auth:matrix.org)
</div>
