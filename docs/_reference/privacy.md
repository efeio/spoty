---
title: Privacy
description: What Spoty stores on your computer, what it sends and to whom, and what it never collects.
nav_order: 4
---

Spoty is a desktop app that runs entirely on your computer. It has no
account of its own, no server, no telemetry, no analytics, and no advertising.
Its author receives nothing about you or how you use it.

Spoty stores Spotify grants in the system credential store and keeps ordinary
settings and history in its own local application directories. It does not
import Spoty settings or credentials.

## What stays on your computer

- **Spotify sign-ins.** The grants Spotify issues when you sign in, and the
  reusable playback credential, are kept in the system credential store:
  Credential Manager on Windows, Keychain on macOS, and Secret Service on
  Linux. Your Spotify password never passes through Spoty; you sign in
  on Spotify's own pages. A proxy password, if you set one, uses the same
  store.
- **Settings and history.** Settings, window positions, recent plays, the
  last session, skins, themes and MilkDrop presets live in the config
  directory.
- **Caches.** Downloaded audio, artwork, lyrics and library metadata live in
  the cache directory and can be deleted at any time.
- **Log.** `spoty.log` records errors and diagnostics. It stays on your
  computer and never contains credentials; share it only if you choose to
  attach it to a bug report.

[Settings & Files](/settings-and-files/) lists every location and what is safe
to delete. **Sign out** in Settings removes the stored credentials.

## What is sent, and to whom

Spoty connects only to the services below.

- **Spotify.** Sign-in, your library, search, playlists, playback and Spotify
  Connect all go to Spotify, under your account. Spotify's own
  [privacy policy](https://www.spotify.com/legal/privacy-policy/) applies to
  that data.
- **LRCLIB.** When the lyrics panel is open and Spotify has no lyrics for the
  song, Spoty sends its artist, title, album and length to
  [lrclib.net](https://lrclib.net). Nothing identifying you is included.
- **GitHub.** Spoty checks its own GitHub repository for releases. There are
  no Spoty release downloads yet. The first opening of MilkDrop also fetches
  its preset packs from GitHub. No Spotify data is sent.
- **Your local network.** Spoty looks for Spotify Connect speakers over
  mDNS and talks to the ones you choose.

Links you open from the app, such as the Winamp Skin Museum or this website,
open in your browser.

## Website

Spoty does not have a separate website yet. The project and its documentation
are in the [GitHub repository](https://github.com/efeio/spoty).

## Questions

Ask on [GitHub](https://github.com/efeio/spoty/issues).
