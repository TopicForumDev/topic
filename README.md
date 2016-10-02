# Topic
## An anonymous forum in Rust
Topic is an anonymous forum written in the Rust programming language. It is a blend of anonymous imageboards and traditional forums, focused on discussion and information exchange rather than image sharing.
The application server (this codebase) provides the base framework for the anonymous forum, but features like (timed) thread culling and "early" bans such as rangebans and spam bans, as well as ban expiry, are expected to be handled by some other mechanism.

The concept considered for thread culling is to delete all but a select few threads every 24 hours, based on bump order. This allows very popular threads to go on without being culled too quickly, thus causing threads on popular topics to be spammed on a given board (as happens in the traditional imageboard format). The threads have pages so that someone who opens a large thread doesn't see their machine grind to a near-halt.
Images are not hosted and are inserted via markdown (handled by hoedown). This cuts down on file hosting costs and legality issues that hosting files might entail.

## Features
This is a quicklist, possibly non-comprehensive, of things that work so far:
- Posting
- Deleting posts
- Bumping
- Reply linking
- Reporting
- Hoedown-based markdown
- Bump-ordered threads
- Boards, board categories
- Board rules
- Hiding specific posts, threads, board categories, and the board rules

Things that should work but haven't been thoroughly tested (and may need manual operations on the database) include:
- Stickying threads
- Turning a post into an admin-post
- Banning users at the application server-level

## Limitations and known issues
This is a pre-alpha release of the codebase because I do not have enough time or interest to reliably work on it. The base concept is complete, except page layout which should be just sufficient to showcase the site.
The following features are considered important or core, but are missing as of this version:
- Challenges. Captchas, automatically generated board-specific questions, or other less invasive, spam-preventing, outsider-gating methods.
- Admin pages. There are untested admin pages available, but they rely on a database architecture that might not match the rest of the site, and they may not even work right.
- Autoreload. The application server is old-style, and the pages being served do not know how to request limited updates to the server, while the server does not serve data over a suitable API.

Less important, planned features include:
- Memory, for things like hidden posts/threads/boards.
- Ban appeals.

## How to run
- Copy `src/config_template.toml` to `src/config.toml` and fill out the fields
- Populate the database
- `cargo build`
- `cargo run`
