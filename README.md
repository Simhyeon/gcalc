# Gcalc

Gcalc is a game probability (mostly gachas) calculator.

### Why not use a simple formula?

Well because, real life examples are not clear cut demonstrated geometric
sequences. Sometimes there is bonus for a specific gacha stage and there is
also so-called confirmed gacha system. Therefore single formula cannot fit into
diverse game gacha environments.

## Demo

```bash
# Print records of probabilty 0.2(20%) with budget of 100 and cost of 20 for each iteration.
# Target probabilty is 0.6
gcalc cond 0.2 --budget 100 --cost 20 -f console -p 2 -T percentage -t 0.6
```
which prints a table with the help from
[tabled](https://crates.io/crates/tabled) crate.
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

## Usage

```bash
# Basic
gcalc <SUBCOMMAND>

# SUBCOMMANDS:
#   cond         Conditional calculation
#   qual         Conditional calculation but only prints result
#   range        Prints range of calculations
#   reference    Create reference file

# Print from 0 to 10 as github markdown formatted table, which has a precision of
# 2 digits. Each try has cost of 1000.
cargo run --features binary -- range 0.2 --count 10 --format gfm --precision 2 --cost 1000

# Print probability changes illustrated as csv formatted table, which has a
# precision of 2 digits. Target probability is 0.8.
cargo run --features binary -- cond 0.2 --format csv --precision 2 --target 0.8
```

Results of prior usages are,
```
# cargo run --features binary -- range 0.2 --count 10 --format gfm --precision 2 --cost 1000
| count | probability | cost |
|-------+-------------+------|
|   1   |    0.20     | 1000 |
|   2   |    0.36     | 2000 |
|   3   |    0.49     | 3000 |
|   4   |    0.59     | 4000 |
|   5   |    0.67     | 5000 |
|   6   |    0.74     | 6000 |
|   7   |    0.79     | 7000 |
|   8   |    0.83     | 8000 |

# cargo run --features binary -- cond 0.2 --format csv --precision 2 --target 0.8
count,probability,cost
1,0.20,0.0
2,0.36,0.0
3,0.49,0.0
4,0.59,0.0
5,0.67,0.0
6,0.74,0.0
7,0.79,0.0
8,0.83,0.0
```

## Goal

- Easily usable binary and library for probability check and calculations
- Easy automation with csv files
- Multi format: cross-platform binary + wasm binary
- Integration with jupyter notebooks
