<div align="center">
  <a href="https://getnodalia.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-white.svg">
      <img src="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-black.svg" width="150" alt="Nodalia">
    </picture>
  </a>
  <br><br>
  <img src="https://raw.githubusercontent.com/danielmigueltejedor/luz/main/assets/app-icon/hicolor/512x512/apps/ai.storyteller.lightcraft.png" width="176" height="176" alt="Luz">
  <h1>Luz</h1>
  <p><strong>A RAW developer and photo library for the Nodalia suite, built on the LightCraft engine.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/status-early%20stage-orange" alt="Early stage">
    <img src="https://img.shields.io/badge/engine-LightCraft-F59300" alt="LightCraft engine">
    <img src="https://img.shields.io/badge/Rust-native-dea584?logo=rust&logoColor=black" alt="Native Rust">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea44f" alt="MIT OR Apache-2.0">
  </p>

  <p>
    <a href="#overview">Overview</a> ·
    <a href="#what-you-get-today">Today</a> ·
    <a href="#build-and-run">Build</a> ·
    <a href="#project-status">Status</a> ·
    <a href="#keeping-up-with-upstream">Upstream</a> ·
    <a href="#the-nodalia-suite">Suite</a> ·
    <a href="#license">License</a>
  </p>
</div>

---

## Overview

**Luz** is the RAW developer and photo library of [Nodalia](https://getnodalia.com). It is built on the engine of
[LightCraft](https://github.com/storytold/lightcraft), which lives in this repository and is kept up to
date with upstream.

> [!IMPORTANT]
> **Luz is at an early stage.** What runs today is the upstream application under the Luz name.
> Nodalia's own interface is not wired in yet and there are no published builds.

## What you get today

- The LightCraft engine.
- A desktop binary called `luz`.
- The engine's own feature description, kept as [`docs/upstream-readme.md`](docs/upstream-readme.md).

## Build and run

You need a recent stable Rust toolchain.

```bash
git clone https://github.com/danielmigueltejedor/luz.git
cd luz
cargo run --release -p lightcraft
```

The desktop binary is called `luz`.

## Project status

| Area | State |
|---|---|
| Engine | Imported from `storytold/lightcraft` and tracked in [`docs/upstream.md`](docs/upstream.md) |
| Product name | Done: the desktop binary is called `luz` |
| App icon | Done: portrait of Louis Daguerre |
| Nodalia interface | Not started |
| Shared interface resources | Planned: the Nodalia apps will share interface components and styles |
| Installable packages | None published |

## Keeping up with upstream

[`docs/upstream.md`](docs/upstream.md) records which upstream commit is integrated. Updates are
merges of upstream's `main`, so Luz's own changes are kept and conflicts are resolved in the open.

## App icon

Nodalia app icons are black-and-white portraits of historical figures over the app colour.
Luz's is Louis Daguerre, a pioneer of photography. Provenance and licence are in [`assets/app-icon/`](assets/app-icon/).

## The Nodalia suite

Luz is one of the applications of [Nodalia](https://getnodalia.com), an independent software
project developed in public by [Daniel Miguel Tejedor](https://danielmigueltejedor.com).

## License

`MIT OR Apache-2.0`, as upstream. The licence files and `NOTICE` of LightCraft are kept.
See [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).

## Credits

Luz would not exist without [LightCraft](https://github.com/storytold/lightcraft) and its contributors.
