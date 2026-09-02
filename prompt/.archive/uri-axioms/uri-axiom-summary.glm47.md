# URI Axioms of Web Architecture - Summary

This document outlines Tim Berners-Lee's axioms of Web architecture, particularly focusing on Universal Resource Identifiers (URIs). These principles form the foundation of how the Web operates and ensure interoperability across platforms and protocols.

## Core Concept: URIs as Universal Identifiers

URIs (Universal Resource Identifiers) are the addresses/names that identify objects in the Web's information space. An object is "on the Web" if it has a URI. These objects are sometimes called "First Class Objects" (FCOs). The Web works best when any information object of value has a URI, allowing it to be referenced and linked to.

## Axiom 0: Universality

**Any resource anywhere can be given a URI.**

**Any resource of significance should be given a URI.**

The Web is designed as a universal information space capable of containing every bit of information accessible by networks. It was built to incorporate existing systems (like FTP) and remain extendable for future protocols. URIs can identify various types of information objects, including services, connection endpoints, and most importantly, generic documents.

The Semantic Web extends this by allowing URIs to identify anything - real objects, abstract concepts - by combining document identifiers with fragment identifiers using "#".

## Axiom 1: Global Scope

**A URI has the same meaning regardless of whom or where you specify it to.**

URIs have no scope requirements. Simply stating something is "on the Web" is sufficient for anyone to follow a hypertext link or look up that URI. While HTTP URIs (starting with "http:") are common, many other URI schemes exist, making the universal space of identifiers compatible with pre-existing protocols (SMTP, NNTP), Web-specific protocols (HTTP), and future protocols yet to be invented.

### Independent Design Test

Any new identifier or address space can be represented as a subset of URI space. Since URIs have no length limit, any new naming system can be encoded as a printable string with a standard scheme prefix (e.g., ISBN:, X500:).

## Axiom 2a: Sameness

**A URI will repeatably refer to "the same" thing.**

The same identifier string should, from one day to the next, point to the same object "in some sense." This leaves open the complex question of identity and what "sameness" means.

## Axiom 2b: Identity

**The significance of identity for a given URI is determined by the owner of the URI.**

The person or agent who created the URI and first determined what it points to defines its identity. While different schemes have varying requirements, the owner operates within scheme-imposed constraints to define identity.

This leads to an important design principle: "The technology should define mechanisms wherever possible without defining policy." Many URI properties are social rather than technical, requiring protocols for exchanging guarantees about URI properties.

Consequences of this axiom:

- Pointers can reference documents that never change
- Pointers can reference documents that change over time
- Pointers can reference documents available in multiple formats
- Pointers can reference documents that vary by requester

The Web architecture leaves it to the URI owner to determine these properties, though they can be described in machine- or human-readable formats.

### Identity Abuse Warning

Web servers often have context about requests (who is asking, what document they came from). Using this information to dramatically change content without clear indication undermines the concept of identity and reference. Unless clearly indicated on the page or via protocol, returning different information for the same URI is a form of deception and disrupts caching. HTTP 1.1's "Vary" header allows proper indication of this behavior.

## URI Equivalence and Canonicalization

**Two URIs are the same if and only if they are identical character for character.**

Different URIs may still be equivalent if they refer to the same thing and produce identical results. Some equivalence can be deduced from web standards:

- HTTP domain names are case-insensitive
- Some sites may have case-invariant paths
- Some sites may treat "/" and "/index.html" as equivalent

However, **no definitive canonicalization algorithm exists for URIs**. Generic URI handling should treat them as case-sensitive character strings. Encryption and signature algorithms should NOT attempt to canonicalize URIs before signing due to the arbitrariness of any canonicalization attempt.

The only canonicalization recommended is that defined by URI specification algorithms (generating absolute URIs from relative ones, hex-encoding/decoding non-reserved characters).

## Axiom 3: Non-Unique Universal Spaces

**URI space does not have to be the only universal space.**

The fact that URIs form a universal space doesn't prevent others from creating their own universal spaces. These spaces could potentially envelop the URI space as a subset. This allows different systems to interoperate even if they were independently invented with different design decisions.

For example, international telephone numbers could be mapped to URI space with a "phone:" scheme. Conversely, URIs could be mapped into international phone numbers by allocating special numbers. Multiple universal spaces can coexist, allowing you to dial any Web object from a telephone or point to any phone from a hypertext document.

This axiom is crucial for the Web's acceptance as a non-domineering technology and our trust in its future evolvability.

## Identity, State, and HTTP GET

Since URIs repeatably refer to "the same" thing, dereferencing a URI often returns exactly the same result, especially over short periods. This enables caching and supports the concept of the Web as an information space rather than a computing program.

### GET Axioms

**In HTTP, GET must not have side effects.**

**In HTTP, anything which does not have side-effects should use GET.**

The HTTP GET operation is expected to repeatably return the same result, making it idempotent. Consequently:

- Software can use previously fetched copies (caching)
- HTTP GET should have no side effects
- GET should never be used to initiate state-changing operations
- Users cannot be held accountable for doing a GET

Side effects mean any significant communication between parties. Server logging doesn't count as communication between the two parties.

Violating this rule (using GET for state changes, adding users to mailing lists, committing to purchases) introduces security problems.

### Correct Form Usage

- Information-requesting forms should use GET (favored over POST because results have a URI and can be bookmarked)
- Operation-executing forms (that change the Web or user relationships) must use POST or other methods

Adhering to these rules ensures interoperability with caches and agents that exploit the repeatability of HTTP GET.

### Generic URIs

URIs don't always need to point to exactly the same bits - they can be "generic" (see discussion of generic URIs in related documentation).

## The Opacity Axiom

**The only thing you can use an identifier for is to refer to an object. When not dereferencing, you should not look at the contents of the URI string to gain other information.**

This principle of **opacity** is fundamental. URIs are typically passed around without examining their internal contents. Software should treat URIs as generally as possible to support reuse of existing and future schemes.

### Examples of Proper Opacity

In HTTP URIs, the client only looks at the first part to determine which server to contact. The rest is opaque to the client - the client cannot deduce information from those characters.

Breaking this rule leads to architectural problems. For instance, clients inferring document type from ".html" at the end of a URI breaks generality.

### Benefits of Opacity

- Opens the door to new URI schemes
- Enables exciting new interpretations of HTTP URI spaces
- Allows servers to use opaque strings for parameters and new topologies

### Metadata in HTTP Protocol

Since clients can't infer metadata from URI strings (due to opacity), the HTTP protocol must provide this information through headers.

### Gateway Applications

When servers act as gateways to existing information spaces, opaque URI strings can encode complex locations from legacy systems. Names from databases might coincidentally end with ".html" without implying an HTML document - it's just the encoding's result.

## Query Strings

The question mark in HTML forms separates the base URI from parameters appended to perform operations. The string following "?" is the query string.

### Evaluation Function

For a URL like `http://foo/bar?baz#frag`:

```
select(get("foo", query("bar", "baz")), "frag")
```

Where:

- `query(resource, querystring)` is evaluated by resource "bar" (opaque to all except server "foo")
- `get(server, restofuri)` is executed by the client (understands "foo", not "bar")
- `select(fragmentid, resource)` is evaluated by client code handling the resource

Query strings are not opaque to the client but should be opaque to proxies.

### Usage Patterns

Query strings are used for searches and HTML form GET actions. The complete URI (including query string) should follow the "slow change" axiom - results should be repeatable.

Common misassumption: results of operations with query strings are more transient. This breaks the opacity rule and is often incorrect. Query strings can indicate parameters like personalized subspaces that produce stable, cacheable results.

Unfortunately, some clients and caches don't cache URIs with question marks, leading to deliberate insertion of "?" to defeat caching. This non-standard, axiom-breaking convention can damage systems using question marks for legitimate, cacheable documents.

## Hierarchies and Relative URIs

### Universal Syntax for Hierarchical Schemes

The URI syntax uses slashes to indicate hierarchical structure, but strings between slashes remain opaque. The double slash "//" and single slash "/" serve as common universal syntax for hierarchical boundaries, which is important because:

1. Hierarchical schemes are common
2. Relative naming within hierarchical spaces has many advantages
3. It's essential for scalability of Web content creation

Relative naming allows closely located documents to reference each other without knowing their absolute position in the tree. This pattern works for file systems, HTTP URLs, and should apply to any hierarchical scheme.

### Example: Telephone Number Mapping

International telephone numbers are hierarchical:

- Country code
- Area code (if any)
- Telephone number

Mapping to URI syntax (using // instead of +):

```
phone://1/617/253-5708
```

Using relative URI rules within a base context:

- Local number: `861-5000`
- Country-relative number: `/800/123-4567`
- International number: `phone://41/22/767-6111`

This demonstrates the power of relative naming in hierarchical systems.

### Myth Dispelled

**Myth:** "The // must only be used to introduce a fully qualified domain name."

**Reality:** The double slash syntax can be extended. If needed, triple slashes could introduce higher-level hierarchies (e.g., interplanetary codes), just as double slashes were introduced when extending Unix file systems to networked file systems.

### RFC1630 Algorithm

RFC1630 defines an algorithm for parsing relative URIs with any arbitrary number of consecutive slashes. While not perfect in handling "." and "..", it applies to any number of slash levels.

Key rules:

- If schemes differ, full absolute URI required
- Otherwise, scheme is omitted
- Matching consecutive slashes from context prepended to partial URI
- Otherwise, last path element removed, partial URI appended
- Recursively remove "xxx/../" and "/." patterns

## Matrix Spaces and Semicolons

Many web sites have documents that vary along multiple dimensions, naturally arranged on a matrix rather than a tree. Example URI for a map:

```
//moremaps.com/map/color;lat=50;long=20;scale=32000
```

Special forms of relative URIs for matrix spaces were proposed but are not a standard Web feature as of 2001.

## Properties of Different URI Schemes

The URI concept doesn't define specific identity properties between URI and resource - that's left to the URI owner. However, different schemes have varying implementations and restrictions.

### Comparison of Common URI Schemes

| Scheme      | Identity Relationship                                                                     | Reuse                                    | Persistence       |
| ----------- | ----------------------------------------------------------------------------------------- | ---------------------------------------- | ----------------- |
| **http**    | Generic document as defined by publisher. Generic URIs possible with content negotiation. | Publisher-defined                        | Publisher-defined |
| **ftp:**    | Sequence of bits                                                                          | Publisher-defined                        | Publisher-defined |
| **uuid:**   | Uniqueness upheld by publisher                                                            | Publisher-defined                        | No dereference    |
| **sha1:**   | Sequence of bits                                                                          | Mathematically extremely unlikely        | No dereference    |
| **mid:**    | Email message (1:1 modulo recoding, header changes)                                       | Possible after 2 years (not recommended) | No dereference    |
| **mailto:** | Mailbox as used in email protocols                                                        | Socially unacceptable                    | No dereference    |
| **telnet:** | Connection endpoint for interactive login                                                 | Publisher-defined                        | No dereference    |

## Anti-Pattern: How Not to Do It

**URI Abuse:** Using separate URI spaces for "name" and "location" in document formats.

Examples:

```html
<a href="uri1" urn="foo"></a>
```

SGML references to "public identifier" and "system identifier"

**The Web Way:** Reference one URI. Include equivalent identifiers as metadata embedded in the document. This expresses exact relationships without ambiguity, with more power and generality, and consistency across applications.

## Key Takeaways

1. **Universality:** URIs can identify any resource globally
2. **Global Scope:** URIs work everywhere without context
3. **Sameness:** URIs repeatably refer to the same thing
4. **Owner-Defined Identity:** The URI owner determines identity semantics
5. **No Side Effects:** HTTP GET must be idempotent and side-effect-free
6. **Opacity:** Don't parse URI strings for information - treat as opaque identifiers
7. **Hierarchical Support:** Slashes enable relative references within hierarchical spaces
8. **Multiple Universal Spaces:** URI space doesn't preclude other universal naming systems
9. **Scheme Flexibility:** Different URI schemes have varying properties but follow core axioms
10. **Metadata over Dual URIs:** Use metadata instead of multiple identifier types

These axioms ensure the Web's interoperability, extensibility, and continued evolution as a universal information space.
