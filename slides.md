---
title: "Auth & systèmes distribués : ne jetons pas le bébé avec l’eau du bain"
#light: true
#ratio43: true
overlay: "@clementd / @gcouprie"
author:
  - name: Clément Delafargue
    desc:
      - Software developer at <a href="https://outscale.com">3DS Outscale</a>
      - <a href="https://framapiaf.org/clementd">@clementd</a>
  - name: Geoffroy Couprie
    desc:
      - Apollo GraphQL
      - "@geal@mastodon.social"
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

# Distributed systems [[but why?]{}]{.incremental}

:::incremental
- tolerate hardware failures
- increase capacity
- decouple lifecycles
- compartimentalize trust
:::

::: notes
| a distributed system is about running different bits of software
| on different pieces of hardware. The goal is to create boundaries
| to compartimentalize stuff (hardware failures, resource use,
| teams management, blast radius of a bug / vuln)
:::

---

# Distributed systems: tradeoffs

::: incremental
- new failure modes
- latency (network calls)
- latency (stale data)
- blind spots (your data is in another castle)
:::

::: notes
| the biggest traedoff is that turning local calls into network
| calls creates tons of new failure modes, + performance considerations
| when aggregating data,  that you have to account for.
| compartimentalization is also a drawback: a central data model is very
| convenient. moving away from that makes things harder:
| either the data is replicated from somewhere and possibly out of date
| or it is just not available locally
:::

---

# Monolith, [[fewer problems]{}]{.incremental}

::: notes
| if you can afford a monolith (tolerate hardware failure,
| team coupling, etc), then all sorts of problems disappear
:::

---

# Now you can leave, bye!

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

# It's a spectrum

::: notes
| the good thing is that you can choose where you apply decentralization
| what's important is staying aware of the dependencies between services
| and the actual call graph triggered by an incoming request. it will
| give you a good idea of possible failure modes and of the actual
| cost of tradeoffs
:::

---

# Centralized auth in a distributed system

::: notes
| every node calls out to the auth service.
| conceptually simple, but the auth service is a SPOF
| some guarantees are still lost
| reliance on operational guarantees from the auth service: see zanzibar
| all in all, it's an okay architecture, even though the auth service
| is a spof: you can limit the scope of the auth service to make it
| more robust and accept this failure mode
:::

---

# Distributed auth in a distributed system

::: notes
| if you want truly autonomous nodes and an auth service spof is
| not possible.
:::

---

# Distributed auth in a distributed system:<br>challenges

:::incremental
- distributed trust
- stale auth info
- incomplete data available locally
- local caches of external information
:::

::: notes
| in this setup, there is no central auth system that you can query
| for fresh information. you have to rely on bearer tokens, which
| can become stale. You need to set up revocation to be able to 
| stop accepting a token once it has been emitted.
| in this setup, a service may carry enough information to make
| auth decisions, so sometimes a cache of external context is required
| this cache can become stale
:::

---

# Bearer tokens

::: notes
| Since services cannot call out to a central auth service, services need a way
| to trust information. A common way to do that is signed tokens
:::

---

# Bearer tokens

:::incremental
- JWT (super common, can be tricky)
- PASETO (JWT without the footguns)
- Roll your own tokens
- Macaroons
- Biscuits
:::

::: notes
| JWT / PASETO / custom tokens: payload + signature. It's up to the services
| to actually interpret it. Conventions in the JWT world with well-known claims
| Macaroons: caveat system for describing constraints
| Biscuits: embedded logic language for describing constraints & access rules
:::

---

# TODO biscuit examples

---

# Mitigating issues

::: notes
| the main issue with distributed auth and bearer tokens is that once
| a token has been created, there is no direct way to invalidate it.
| the only way you can do it is to tell everyone "stop accepting this
| token in particular"
:::

---

# Token revocation

::: notes
| talking about token revocation could easily fill a 1h slot so we'll
| have to summarize
:::

---

# Token revocation

::: incremental
- start with static lists from day 1
- make sure tokens are unique / identifiable
- keep track of emitted tokens
- all tokens should have an expiration date
- prune token list regularly
:::

::: notes
| talking about token revocation could easily fill a 1h slot so we'll
| have to summarize.
| have a basic revocation infra available from day 1, even if it's
| dumb. You'll thank yourself once you have to revoke a token.
| make sure every token is uniquely identifiable so it can be revoked
| without affecting other holders.
| if possible, keep a list of all the tokens you generate, so you can
| revoke them (you might know that a token has leaked without having
| access to the token itself). if tokens have unique ids, you can
| store them instead of the tokens, this way it's not sensitive info
| revocation and emission lists can only grow, so they need to be
| pruned at some point. the simplest way to do that is to give every
| token an expiration date, and then store this date in the emission
| and revocation lists. this way, pruning is easy.
:::

---

# Access token / refresh token

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

# Distributed auth patterns

- auth gateway
- chained calls with service-level restrictions
- token delivery service + capability-based tokens
- offline attenuation

---

# Plan

- comment choisir entre un système d'auth centralisé et un système distribué
- un tour d'horizon des solutions possibles pour les jetons au porteur;
- les différentes architectures d'auth possibles (passerelle d'auth, intégration directe, …);
- les éléments indispensables à mettre en place dans un tel système (rafraichissement des tokens, révocation, rotation des clés, …);
- la plateforme biscuit, construite autours de ces use cases.

---

# Centered title

---

# Centered title even when it's long and spans multiple lines

---

# Centered [[incremental]{}]{.incremental} title

---

# Top title

- With
- content

---

# Top title

::: incremental
- a
- b
- c
:::

---

# [Jumbo text]{.jumbo}

---

::: jumbogroup

## [a group of]{.jumbo}
## [big jumbo text]{.jumbo}
## [because it's fun]{.jumbo}

::::::::::

---

```haskell
test :: Test
test = do
  traverse (`xor` b) [test]
  
```

---

# Title that should be centered but is not because of notes

::: notes
| notes that are displayed because the right flag is added to the pandoc
| invocation. The leading `|` character allows to preserve line breaks,
| that's convenient in notes
:::

---

:::bigimage
![](./assets/puna.jpg)
:::
