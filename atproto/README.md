# ATProto lookup skill

Reference material and worked recipes for AT Protocol lookups — resolving handles, finding
records that link to a record (likes, reposts, replies, quotes), and checking whether a record
exists. Extend as more lookup flows get exercised.

For client login and session checks, see [ATProto CLI sessions](/atproto/cli.md).

## Handle → DID

AppView endpoint, no auth:

```sh
curl -s 'https://public.api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle=jauntywunderkind.bsky.social'
# {"did":"did:plc:zjbq26wybii5ojoypkso2mso"}
```

Also available: `https://plc.directory/<did>` for the DID document (PDS, signing keys, rotation
history).

## Finding likes of a post with Constellation

[Constellation](https://constellation.microcosm.blue/) (by [microcosm](https://microcosm.blue/))
is a community-run threaded graph API over the firehose: it indexes every link between records
(~18B linking records, ~28M DIDs as of 2026-09) and lets you query them by source/target at-uri.

### The recipe

```sh
# 1. resolve the actor
curl -s 'https://public.api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle=<handle>'

# 2. query likes pointing at the post, narrowed to one actor via source=
curl -s 'https://constellation.microcosm.blue/links?collection=app.bsky.feed.like&path=subject.uri&source=<did>&target=at://<repo>/app.bsky.feed.post/<rkey>'

# 3. cross-check the post itself on the AppView
curl -s 'https://public.api.bsky.app/xrpc/app.bsky.feed.getPosts?uris=at://<repo>/app.bsky.feed.post/<rkey>'
```

### Query modes

`GET /links` takes two shapes:

1. **source + target**: both must be *full* at-uris including record path
   (`at://did:plc:.../app.bsky.feed.like/3mxyz...`).
2. **collection + path + target**: `target` is the at-uri being pointed at; `collection` is the
   type of the linking record; `path` names which field of that record holds the link.

`source=` is optional in mode 2 and narrows results to one actor. Omitting `path` in mode 2
fails with `Failed to deserialize query string: missing field 'path'`.

Handy `path` values:

| collection | path | finds |
| --- | --- | --- |
| `app.bsky.feed.like` | `subject.uri` | likes of a post |
| `app.bsky.feed.repost` | `subject.uri` | reposts of a post |
| `app.bsky.feed.post` | `replyParent` | direct replies |
| `app.bsky.feed.post` | `embed.record.uri` | quote posts |

### Response shape

```json
{ "total": 0, "linking_records": [], "cursor": null }
```

`linking_records` entries carry `source` (DID), `path`, and the record URI of the linking
record.

### Caveats

- **A `total: 0` is not proof of absence.** Constellation is eventually-consistent firehose
  coverage. Cross-check before concluding (see below).
- `getPosts` returning an entry with `null` `cid`/`likeCount` fields means the AppView has no
  live record for that URI — deleted, or not (yet) indexed.
- There is also a browser console for exploratory queries: <https://console.microcosm.blue/>.

### Cross-checks when Constellation comes up empty

- Query without `source=` — if *nobody's* likes on the post are indexed, coverage is the
  suspect, not your like.
- **Own PDS repo** (authoritative, always works): the local checkout
  `~/archive/jauntywunderkind.bsky.social/` holds a copy of the user's repo; or fetch live via
  `com.atproto.repo.listRecords` with `collection=app.bsky.feed.like` against the user's PDS.
- [pdsls.dev](https://pdsls.dev) — browse any repo/record in the browser
  (see [`bsky-at-links`](~/src/bsky-at-links) for a webextension that surfaces `at://` links to
  feed it).

## Reference material

### Local (check `~/archive/` first — `dl <url>` lands new things here)

Archive checkouts, org by org (machine wikis under `~/wiki/<org>/<repo>` where present):

| Org | Project | Notes |
| --- | --- | --- |
| [`bluesky-social`](https://github.com/bluesky-social) | [`atproto`](https://github.com/bluesky-social/atproto) | TypeScript reference SDK |
| | [`bsky`](https://github.com/bluesky-social/bsky) | AppView |
| | [`pds`](https://github.com/bluesky-social/pds) | reference PDS |
| | [`indigo`](https://github.com/bluesky-social/indigo) | Go implementation |
| | [`ozone`](https://github.com/bluesky-social/ozone) | moderation |
| | [`feed-generator`](https://github.com/bluesky-social/feed-generator) | |
| | [`social-app`](https://github.com/bluesky-social/social-app) | bsky.app web client |
| | [`goat`](https://github.com/bluesky-social/goat) | atproto CLI |
| | [`jetstream`](https://github.com/bluesky-social/jetstream) | firehose streaming |
| [`microcosm.blue`](https://github.com/microcosm-blue) | [`hubble`](https://github.com/microcosm-blue/hubble) | firehose consumer |
| | [`lightrail`](https://github.com/microcosm-blue/lightrail) | |
| | [`microcosm-rs`](https://github.com/microcosm-blue/microcosm-rs) | Rust |
| | `RecordPath` | |
| | `repo-stream` | |
| | `pds-debug` | |
| | `compare-hoses` | |
| [`at-microcosm`](https://github.com/at-microcosm) | `lexicons` | |
| | `micro-ops` | |
| | `ufos-app` | |
| [`ATProtocol-Community`](https://github.com/ATProtocol-Community) | `atmosphereconf` | conference materials |

No constellation checkout yet — `dl` it when needed.

Account repos (PDS checkouts via `dl`): `~/archive/jauntywunderkind.bsky.social/`,
`~/archive/blaine.bsky.social/`, `~/archive/why.bsky.team/`, ...

First-party:

- [`~/src/bsky-at-links`](~/src/bsky-at-links) — webextension adding `at://` links to bsky.app
- [`~/src/search-bluesky-likes-recursion`](~/src/search-bluesky-likes-recursion) — client-side
  bluesky likes search

### Online

- Constellation API: <https://constellation.microcosm.blue/> · web console:
  <https://console.microcosm.blue/>
- AppView XRPC: <https://public.api.bsky.app/xrpc/> (`app.bsky.*`, no auth for reads)
- DID directory: <https://plc.directory>
- Specs & docs: <https://atproto.com/specs/overview> ·
  [Bluesky API docs](https://docs.bluesky.xyz/)
- Record browser: <https://pdsls.dev>
