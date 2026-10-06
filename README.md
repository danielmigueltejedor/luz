# Luz

Revelado RAW y biblioteca fotográfica de Nodalix.

El motor de este repositorio es el código de [storytold/lightcraft](https://github.com/storytold/lightcraft) en `265248c86fa3267709aea45633d0d135331e180c`.
La licencia de ese código se conserva, igual que `NOTICE` y `docs/upstream-readme.md`.
Luz es el nombre del producto. El motor puede cambiar sin esperar a upstream.
Un sync posterior compara commits. No se sustituye este árbol con un reset.

## Compilar

```bash
cargo run -p lightcraft
```

El binario de escritorio se llama `luz`.

## Estado

La shell que entra en este import es la de upstream. La interfaz GTK 4 / libadwaita de Nodalix no está cableada todavía. El motor sí está en este repositorio, con sus tests.
