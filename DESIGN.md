# Spoty Design System

## Theme

The interface serves desktop listeners choosing music at a desk, often with
the app open for long stretches beside other work. Keep the screen composed
and easy to scan in both bright and dim rooms. Respect the listener's light or
dark theme preference.

Use a restrained palette: warm, low-chroma neutral surfaces with a muted rose
accent reserved for primary actions, selected states, and playback status.
Keep active navigation and the primary playback control on the rose accent;
inactive browsing controls stay quiet. Artwork may add color to the content,
but should not tint every surface or compete with text.

## Color roles

- **Window and panel:** warm near-black in dark mode, soft ivory in light mode.
- **Surface:** a slightly lifted neutral for controls and interactive rows.
- **Text:** warm near-white / deep plum-black with subdued secondary text.
- **Accent:** rose, used sparingly for active playback and primary actions.
- **Semantic colors:** retain distinct danger and warning roles.

## Typography

Use the existing Inter family and its real weights. Keep headings clear and
compact, with regular or medium weight for supporting labels. Preserve the
current platform-aware font sizing and text direction behavior.

## Layout and components

- Keep the native three-part shell and familiar library, search, and player
  controls while individual screens are reshaped.
- Show home shortcuts in one horizontal strip so recent listening and discovery
  sit closer to the first viewport without losing fast library access.
- Use artwork as the focal point for music shelves and collection pages.
- Give section headings and content enough separation to make browsing
  scannable without adding decorative containers.
- Preserve the existing shared buttons, rows, cards, loading, and error states
  unless a screen-specific redesign requires a clear improvement.

## Interaction

Keep playback feedback immediate. Motion should explain a state change and
remain restrained. All primary actions must remain usable by keyboard and
clearly distinguishable in both themes.
