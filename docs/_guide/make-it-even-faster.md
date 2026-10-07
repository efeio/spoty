---
title: Make It Even Faster
description: "Reduce loading delays with your own Spotify connection."
nav_order: 6
---

## Spotify API access

Spoty uses its own Spotify Developer app, currently in Development Mode.
The app owner needs Premium, and each user must be on the app's allowlist.
Spotify allows up to five users per Development Mode app. Some API endpoints
are restricted in this mode, so a few Spotify features may not be available.
See Spotify's [quota modes](https://developer.spotify.com/documentation/web-api/concepts/quota-modes).

You can reduce those delays by creating a **personal Spotify app**. This is
a connection registered to your account on Spotify's developer website.
You do not need to write code or install another player.

Spoty offers this setup once after you sign in with Premium. Choose
**Set up personal app** to open Settings, or **Keep shared app** to continue
as you are. You can set it up later in Settings.

A personal app can give supported requests another developer account's
allowance. Spotify counts Development Mode quota per developer account, so
multiple app IDs on the same account share one allowance. Creating an app is
free and takes a few minutes, and the app owner needs Premium. If you create
several personal apps, they share your account's allowance, following Spotify's
[July 2026 quota update](https://developer.spotify.com/blog/2026-07-23-web-api-quota-updates).
Some features still use Spoty's app, and a personal connection has the same
Development Mode user and endpoint limits.

## Requests use both apps

Your personal connection handles supported searches, library reads, and
playback control. Spoty's connection remains available for requests tied to
the default app. Search results appear as each request is ready.

Development Mode allows ten search results at a time for each type.

Setting up playback on this computer also helps playlists load faster.
Spoty can load some playlists through its music connection instead.
[How it connects](/how-it-connects/) explains which connection each feature uses.

## Make a Spotify app

1. Open the [Spotify developer dashboard](https://developer.spotify.com/dashboard)
   and sign in with your Spotify account. Spotify asks that it be a
   Premium account.
2. Click **Create app**. Choose a name and description you recognize; Spotify
   shows them during sign-in.
3. Under **Redirect URIs**, add exactly:

   ```
   http://127.0.0.1:8989/login
   ```

4. Tick **Web API**, accept the terms, and save.
5. The app's page shows its **Client ID**. Copy it.

![Settings, with a personal Spotify app in use](/assets/images/make-it-even-faster.png)

## Use it in Spoty

1. Open **Settings**, find **Personal Spotify app**, and paste the
   Client ID.
2. Click **Authorize**. Your browser opens Spotify's sign-in for your app.
   Spoty verifies that it belongs to the same Spotify account, then shows
   **Personal app ready**.

Your playback setup stays the same. Select **Remove** to stop using your
personal connection and return to shared access.
