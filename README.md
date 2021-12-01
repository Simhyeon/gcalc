# Gcalc

Gcalc is a game probability (mostly gachas) calculator.

### Why not use a simple formula?

Well because, real life examples are not clear cut demonstrated geometric
sequences. Sometimes there is bonus for a specific gacha stage and there is
also so-called confirmed gacha system. Therefore single formula cannot fit into
diverse game gacha simulations.

## Binary usage

```bash
# Print up to 10 rows of calculations which has 50% probability each
gcalc 50%
# Read csv value and count 10 rows
gcalc -f values.csv -c 10
```

## Goal

- Easily usable binary and library for probabilty check and calculations
- Easy automation with csv files
- Multi format: cross-platform binary + wasm binary + c-compatible library
