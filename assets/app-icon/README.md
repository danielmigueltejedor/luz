# Luz app icon

**Figure:** Louis Daguerre. A pioneer of photography, the craft a RAW developer serves.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Louis Daguerre 2.jpg" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Louis_Daguerre_2.jpg) (public domain). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

## Files

| File | What |
|---|---|
| `lightcraft.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `lightcraft-small.svg` | lighter 512 px version; also the scalable Linux icon and the web favicon where there is one |
| `lightcraft-1024.png` | 1024 px render (store listings, docs) |
| `lightcraft-macos-512.png` | runtime window/Dock icon on macOS (embedded by `apps/lightcraft/src/main.rs`) |
| `lightcraft.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `lightcraft.ico` | Windows icon, 16-256 px, embedded in `lightcraft.exe` by `apps/lightcraft/build.rs` |
| `hicolor/<n>x<n>/apps/ai.storyteller.lightcraft.png` | Linux icon theme, 16-512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.lightcraft.svg` | Linux scalable icon (the small SVG) |

The app id is `ai.storyteller.lightcraft`: the Wayland app id, the `.desktop` file
(`packaging/linux/ai.storyteller.lightcraft.desktop`, `Icon=ai.storyteller.lightcraft`) and the hicolor icon
name. To install on Linux, copy `hicolor/` into `/usr/share/icons/hicolor/` (or `~/.local/share/icons/hicolor/`)
and the `.desktop` file into `applications/`.

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `lightcraft.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
