# Universal Resource Identifiers -- Axioms of Web architecture

**Tim Berners-Lee**

_December 19, 1996_

## Summary

The Web is a universal information space where any resource of significance should be given a URI (Universal Resource Identifier). This foundational document outlines core axioms that define how the Web operates and ensures interoperability across different systems and protocols.

## Key Axioms

### Universality

- **Axiom 0**: Any resource anywhere can be given a URI - the Web can contain in principle every bit of information accessible by networks
- **Axiom 0a**: Any resource of significance should be given a URI - objects without URIs can't be referenced, limiting Web's power
- **Theorem**: Any new space of identifiers or address space can be represented as a subset of URI space

### Global Scope & Identity

- **Axiom 1**: Global scope - It doesn't matter to whom or where you specify that URI, it will have the same meaning
- **Axiom 2a**: Sameness - A URI will repeatably refer to "the same" thing (though "sameness" can vary by context)
- **Axiom 2b**: The significance of identity for a given URI is determined by the person who owns the URI

### URI Opacity

- **Opacity Axiom**: URIs should be treated as opaque strings - The only thing you can use an identifier for is to refer to an object. When not dereferencing, you should not look at URI contents for other information
- **Canonicalization**: Two URIs are the same if they are the same character for character. No definitive canonicalization algorithm exists for URIs

### HTTP GET Semantics

- **GET must not have side effects** - This allows caching and makes GET operations idempotent
- **Anything without side effects should use GET** - Enables bookmarking, caching, and proper Web architecture

### Universal Space Flexibility

- **Axiom 3**: URI space does not have to be the only universal space - multiple universal spaces can co-exist and interoperate
- **Passing the Test of Independent Invention**: The Web should meet the test that if a similar system were independently invented elsewhere, the two could be made to interoperate

## Important Consequences

### First Class Objects (FCOs)

An information object is "on the web" if it has a URI. Objects with URIs are First Class Objects. The Web works best when any information object of value and identity is a first class object.

### Identity Abuse Warning

The document warns against breaking identity concepts by returning dramatically different content based on request context (user, referrer, etc.) without clear indication, as this undermines reference and caching.

### Hierarchical Structure

The slash-based hierarchical syntax in URIs enables relative naming, which is essential for scalability. Relative URIs allow documents close in a tree to refer to each other without knowing absolute positions.

### Different URI Schemes

Different URI schemes (http:, ftp:, mailto:, etc.) have different identity properties regarding:

- What the URI corresponds to
- Reuse policies
- Persistence guarantees
- Dereference capabilities

### Anti-Patterns

- **Don't split "name" and "location"** - The Web way is one URI per reference, with alternative identifiers embedded as metadata
- **Don't break opacity** - Looking at URI internals leads to fragile assumptions

## Legacy & Impact

This document established foundational principles that continue to guide Web architecture:

- Cool URIs don't change (persistence matters)
- Opacity enables future extensibility
- GET semantics enable the Web as an information space vs. a computing program
- Universal design allows independent systems to interoperate

---

_Document originally published at https://www.w3.org/DesignIssues/Axioms.html_
_This markdown version is a Key Concepts (k2) summary for easier reference_
