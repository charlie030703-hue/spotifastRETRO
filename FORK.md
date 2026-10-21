# SpotifastRETRO

SpotifastRETRO is a hard fork of [crmne/spotifast](https://github.com/crmne/spotifast)
(MIT, see `LICENSE`). It keeps Spotifast's native Rust backend (librespot
playback, sign-in, the Web API, the image cache, the audio device, the Winamp
skin engine and MilkDrop) and replaces the interface with a new one built
from the Dell 1996 design language in `Design Reference/DESIGN-dell-1996.md`.

## What changes from upstream

- **The interface.** A black page frame, flat catalogue-tinted ribbon cards,
  Arial Black display titles, Times body copy, hard bevels and stickers.
  Album artwork leads every page; each card's tint is the artwork's dominant
  colour snapped to one of the eight catalogue tints.
- **No upstream merges.** Upstream's commits are not merged routinely; fixes
  worth having are ported by hand.
- **Policy checks removed.** Upstream's CI checks for maintainer-approved
  settings, dependency sources and review evidence do not apply to this fork.

## What stays

The product boundaries in `AGENTS.md` still hold: no browser engine, no
telemetry, no hosted backend, no alternate audio sources, no DRM work. The
architecture rules still hold too: views emit `Action`s, nothing blocks the
UI thread, disk writes go through `fastframe_file::Writer`, the interface is
optimistic, and visualisers show the signal post-EQ and pre-volume.

## Design rules for the new interface

- `{rounded.none}` everywhere; only seals are round.
- No soft shadows, gradients or opacity fades. Depth is a 1 px hairline, the
  8 px frame, or a hard 1 px bevel.
- Dell red appears twice per screen at most: the page's single play CTA and
  the time readout in the top banner.
- No colour outside the closed palette, except the artwork itself.
