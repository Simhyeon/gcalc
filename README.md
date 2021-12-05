# Gcalc

Gcalc is a game probability (mostly gachas) calculator.

### Why not use a simple formula?

Well because, real life examples are not clear cut demonstrated geometric
sequences. Sometimes there is bonus for a specific gacha stage and there is
also so-called confirmed gacha system. Therefore single formula cannot fit into
diverse game gacha environments.

## Demo usage

```bash
# Print records of probabilty 0.2(20%) with budget of 100 and cost of 20 for each iteration.
# Target probabilty is 0.6
gcalc cond 0.2 --budget 100 --cost 20 -f console -p 2 -T percentage -t 0.6
```
which prints the table with the help of [prettytable-rs](https://github.com/phsym/prettytable-rs)
```
+-------+------------+------+
| count | probabilty | cost |
+-------+------------+------+
| 1     | 20.00%     | 0    |
+-------+------------+------+
| 2     | 36.00%     | 20   |
+-------+------------+------+
| 3     | 48.80%     | 40   |
+-------+------------+------+
| 4     | 59.04%     | 60   |
+-------+------------+------+
| 5     | 67.23%     | 80   |
+-------+------------+------+

```

## Goal

- Easily usable binary and library for probability check and calculations
- Easy automation with csv files
- Multi format: cross-platform binary + wasm binary + c-compatible library
- Integration with jupyter notebooks
