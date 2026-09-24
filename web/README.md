---
type: Guide
title: web — guides for building for the web
description: Platform-first guidance for web work: reach for HTML, CSS, and browser features before toolchains.
resource: /web/README.md
tags: [web, html, css, performance]
status: draft
generated: { by: "model:glm-5.3", at: 2026-09-28 }
stale_after: 2027-03-28
sources:
  - { resource: https://modernmotherfuckingwebsite.dreamstation.systems/, title: Modern Motherfucking Website, author: "Robin Reel", last_modified: 2026-09-24 }
---

<a id="web"></a>
# web

Guides for web work. Default posture: the platform already shipped the
feature — type the tags. Reach for a toolchain only when the problem outgrows
what a file of HTML can express.

- [Modern Motherfucking Website](#modern-motherfucking-website) — every
  "modern feature" is just tags: semantics, SEO, social cards, performance,
  images.

<a id="modern-motherfucking-website"></a>
## Modern Motherfucking Website

Source: [Modern Motherfucking Website](https://modernmotherfuckingwebsite.dreamstation.systems/)
by Robin Reel — a CC0 sequel to motherfuckingwebsite.com. Thesis: the things
teams justify frameworks for (accessibility, SEO, social cards, performance,
responsive images, dark mode) are all plain HTML, CSS, or one script tag of
JSON. "Start with HTML. Add what you need. Stop when you're done."

**Semantics first.** Landmarks and document outline come from element names,
not ARIA libraries. Disclosure widgets are `<details>`, modals are
`<dialog>`, and native form validation/date inputs need no JS:

```html
<header>…</header>
<nav>…</nav>
<main>
  <article>
    <h1>One h1 per page</h1>
    <time datetime="2026-09-24">today</time>
  </article>
</main>
<footer>…</footer>
```

**Structured data is a script tag of JSON-LD** — no SEO plugin. Pick a type
from schema.org, fill in the blanks:

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "…",
  "datePublished": "2026-09-24",
  "author": { "@type": "Person", "name": "…" }
}
</script>
```

**Social cards are seven meta tags:**

```html
<meta property="og:type"  content="article">
<meta property="og:title" content="…">
<meta property="og:description" content="…">
<meta property="og:url"   content="https://…">
<meta property="og:image" content="https://…/www.webp">
<meta property="og:image:alt" content="…">
<meta name="twitter:card" content="summary_large_image">
```

Crawlers see these instantly; a client-rendered SPA has to bolt on SSR to
get back what writing HTML gave for free.

**Performance is link tags and attributes**, not a framework: `preconnect`
/ `dns-prefetch` to open connections early, `preload` what the parser can't
find, `fetchpriority="high"` for the hero image, `loading="lazy"` for the
rest:

```html
<link rel="preconnect" href="https://cdn.example.com" crossorigin>
<link rel="preload" href="/fonts/body.woff2" as="font" type="font/woff2" crossorigin>
<img src="hero.avif" fetchpriority="high" width="1200" height="600" alt="…">
<img src="footer-junk.avif" loading="lazy" decoding="async" width="400" height="300" alt="…">
```

Instant navigations without a client-side router — speculation rules (a
script tag of JSON) plus view transitions in CSS:

```html
<script type="speculationrules">
{ "prerender": [{ "where": { "href_matches": "/*" }, "eagerness": "moderate" }] }
</script>
```

```css
@view-transition { navigation: auto; }
```

If you preload everything you've prioritized nothing — hint actual content.

**Images: `<picture>` picks the format and size; the browser decides.**
`width`/`height` reserve layout space (CLS fixed with two attributes), and
encoding is one command when you add the image, not a build system:

```html
<picture>
  <source type="image/avif" srcset="hero-800.avif 800w, hero-1600.avif 1600w"
          sizes="(max-width: 40rem) 100vw, 40rem">
  <source type="image/webp" srcset="hero-800.webp 800w, hero-1600.webp 1600w"
          sizes="(max-width: 40rem) 100vw, 40rem">
  <img src="hero-800.jpg" width="1600" height="800" alt="…">
</picture>
```

```sh
avifenc hero.png hero.avif
cwebp -q 80 hero.png -o hero.webp
```

**The rest is one-liners**: dark mode is a single `prefers-color-scheme`
media query; responsive layout is `max-width`. Modern CSS (nesting,
variables, grid, container queries, `:has()`, `clamp()`) removed the reasons
for preprocessors.

**When a toolchain *is* warranted** (the source concedes): hundreds of
pages with shared chrome → static site generator or SSI — generate HTML,
serve HTML. Genuine applications (editors, spreadsheets) → a real app
framework. Everything else is a document, and the web is extremely good at
documents.

## Cross-references

- [constitution](/constitution/README.md) — doc lifecycle and promotion;
  this guide may promote material from `design/` inquiries as web work
  accumulates.
