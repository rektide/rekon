---
type: Practice
title: Consumer accounts — let users of a library speak at its home
description: Keep project-grounded accounts of library use, case studies, friction and improvement hopes in the library's own documentation.
resource: /praxis/consumer-accounts/README.md
status: draft
generated: { by: "model:openai/gpt-6-astra#medium", at: "2026-10-22" }
tags: [praxis, consumers, libraries, case-studies, upstream]
sources:
  - resource: "opencode://ses_e9a40d003ffeEeLMvkRjLaMwVK"
    title: Human asks to document and recommend the library-side consumer documentation pattern for libraries we use under ~/src
  - resource: file:///home/rektide/src/kubeless/doc/consumer/index.md
    title: Kubeless groups native consumer accounts, implemented adoption and prospective opportunities
  - resource: file:///home/rektide/src/kubeless/doc/consumer/twigd.md
    title: Twigd account joins current adoption, native responsibilities, revision-specific evidence and upstream hopes
  - resource: /evidence/README.md#evidence-case-stories
    title: Case stories connect expected help, actual conduct, results and reflection
  - resource: /praxis/README.md
    title: Praxis connects action and learning with maintained domain homes
---

# Consumer accounts

**Give the projects using a library a voice in that library's own documentation.**
For libraries we use under `~/src`, recommend a `doc/consumer/` home containing
accounts from the consuming projects: how they use the library, what happened
in practice, what is awkward or missing, and what they hope it could make possible.
Follow an existing repository's documentation conventions when choosing the path.

## Terms

- **Library / provider:** the shared project supplying a capability; this practice
  also applies to reusable tools and services.
- **Consumer:** a project using or considering that capability.
- **Consumer account:** a maintained, attributed account of that project's use
  and aspirations, hosted where the provider's maintainers can find it.

## Where it lives

The concrete precedent is
[Kubeless's consumer index](file:///home/rektide/src/kubeless/doc/consumer/index.md).
Its library-side collection lets readers compare actual adoption and prospective
uses across projects. [Twigd's account](file:///home/rektide/src/kubeless/doc/consumer/twigd.md)
illustrates the combination: product intent, implemented use, native/shared
responsibilities, compatibility evidence, and desired improvements.

Start with `doc/consumer/<project>.md` in the library repository. Add an
`index.md` as the collection grows, with a short description and current standing
for each account. Link it from the library's documentation entrance. Link back
from the consuming project's integration or architecture documentation, so either
side can lead a reader to the account.

The consumer remains the authority on its needs and experience; the library
maintainers own the library's contracts and priorities. Attribute who is speaking
and link to the consumer's native code, design, evidence and work items. Preserve
those owner records rather than copying their entire history into the library.

## What to capture

A useful account can be short. Give the reader enough to understand:

1. **Purpose and standing.** What is the consumer trying to offer? Is this
   existing use, an experiment, or prospective adoption?
2. **Current use.** Which library capabilities does it use, how are they composed,
   and what remains the consumer's responsibility? Link concrete entry points.
3. **Case studies.** What help was expected, what was tried, what happened, and
   what was learned? Include friction and failed attempts alongside successes.
   Identify versions or revisions when they matter to the result.
4. **Hopes and improvement asks.** What experience would become easier or newly
   possible? Explain the payoff, current workaround, and relevant constraints.
   Distinguish the underlying need from a suggested API or implementation.
5. **Next connections.** Link relevant library discussions, designs and existing
   tickets, plus the consumer-side source of the request. Keep open questions visible.

This is a place for ambitious hopes as well as observed use. Mark their standing:
an improvement request is consumer intent, and an accepted library direction is
a separate decision. A library feature landing and a consumer adopting it are
also separate events; record each when it happens.

## When to write and refresh

Recommend this pattern when adopting a local library, investigating a meaningful
integration, or learning something consequential from its use. Look for an
existing account before adding another. Update it when adoption changes, a case
produces evidence, a workaround disappears, or the desired direction develops.
Retain links to earlier investigations and revision-bound results.

When working on the library, read relevant consumer accounts before shaping shared
interfaces. Compare recurring needs across accounts: they can reveal a useful
shared capability or show why a responsibility belongs with a particular consumer.
Return implementation outcomes and remaining gaps to the accounts so the next
library or consumer session can start from what actually changed.

## Cross-references

- [Evidence and case stories](/evidence/README.md#evidence-case-stories) supplies
  the expected-help → actual-result → learning structure for the cases within an account.
- [Post-build possibility gathering](/praxis/post-build/README.md) explores
  shared offerings made possible by a completed integration; consumer accounts
  give that exploration a persistent source of needs and subsequent results.
- [Fronts and horizons](/praxis/fronts/README.md) helps compare the payoff of
  improvement directions surfaced by several consumers.
- [Praxis](/praxis/README.md) connects this feedback practice to acting and learning.
