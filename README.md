# Spoty

**Your Spotify library, in a calmer and more considered desktop app.**

Spoty brings Spotify's familiar discovery and playback flow together with a
quiet, carefully shaped interface. The goal is to make it feel at home for
everyday listening and detailed library management, without borrowing another
music app's visual identity.

## Project status

Spoty is being built from the Spotifast codebase. Its name, app identity and
visual language are changing first; the listening experience will continue to
be reshaped around Spoty's own product direction. There are no Spoty release
downloads or update feed yet.

Spotify currently limits Spoty's Developer Mode app to five allowlisted
accounts and requires Premium for the app owner. Wider access depends on
Spotify's quota approval.

## What it does today

- Browse and search Spotify's catalogue and your library.
- Play Spotify music, manage playlists and control the queue.
- Use Spotify Connect and local playback through librespot. Local playback
  requires Spotify Premium.
- Choose light or dark appearance, change playback settings and use the
  keyboard to control the app.

Spoty is a native desktop client. It uses Spotify's Web API and librespot for
the capabilities they provide. It does not use a hosted Spoty backend or an
alternate music catalogue.

## Build from source

You need the Rust toolchain in `rust-toolchain.toml` and the platform
development libraries described in [CONTRIBUTING.md](CONTRIBUTING.md).

```sh
cargo run --no-default-features
```

To open the app with sample content and without signing in:

```sh
cargo run --no-default-features --features demo -- --demo
```

MilkDrop remains available as an optional feature with `--features milkdrop`.

## Local data

Spoty uses its own settings and credential-store identity. Existing Spotifast
settings and Spotify credentials are left untouched and are not imported; sign
in to Spotify again in Spoty.

## Product and design direction

- [Product brief](PRODUCT.md)
- [Design principles](DESIGN.md)
- [Contributing guide](CONTRIBUTING.md)

Spoty is an independent project and is not affiliated with Spotify. Spotify is
a trademark of Spotify AB.
