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
| authentication: who are you?
| authorization: what can you do?
| often authentication then authorization, but not necessarily
:::

---

# Distributed systems

::: notes
| several nodes, network communications: failure happens
| goals: resiliency, scaling out
:::

---

# Monolith, [[fewer problems]{}]{.incremental}

::: notes
| everything happens in a single place, auth checks happen locally
| the main issue becomes expressing auth rules
:::

---

# Centralized auth in a distributed system

::: notes
| every node calls out to the auth service. conceptually simple, but SPOF
| some guarantees are still lost
| reliance on operational guarantees from the auth service: see zanzibar
:::

---

# Distributed auth in a distributed system

::: notes
| avoid spofs by making nodes autonomous
:::

---

# Distributed auth in a distributed system:<br>challenges

:::incremental
- stale info (revocation)
- distributed trust
- incomplete data available locally
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

# Distributed auth patterns

- auth gateway
- chained calls with service-level restrictions
- token delivery service + capability-based tokens
- offline attenuation

---

# Mitigating issues

- stale tokens & revocation
- key rotation

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
