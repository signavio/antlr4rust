# Quick Start Guide: CSV Parser Benchmarks

This guide will help you quickly run and understand the CSV parser benchmarks.

## Prerequisites

- Rust toolchain installed
- Navigate to the `antlr4rust` directory

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench --bench csv_benchmark
```

This will run all benchmark suites and generate HTML reports.

### Run Specific Benchmark Groups

```bash
# Lexer-only benchmarks (tokenization)
cargo bench --bench csv_benchmark csv_lexer

# Full parser benchmarks (lexer + parser + tree)
cargo bench --bench csv_benchmark csv_parser

# Quoted string handling
cargo bench --bench csv_benchmark csv_quoted_strings

# Varying column count tests
cargo bench --bench csv_benchmark csv_varying_rows
```

### Run a Single Benchmark

```bash
# Run just the small CSV lexer test
cargo bench --bench csv_benchmark csv_lexer/small
```

## Viewing Results

### Terminal Output

Benchmark results appear immediately in the terminal:

```
csv_parser/small/27     time:   [3.0686 µs 3.0740 µs 3.0798 µs]
                        thrpt:  [8.3607 MiB/s 8.3764 MiB/s 8.3912 MiB/s]
```

This shows:
- **time**: Mean execution time with confidence interval [min mean max]
- **thrpt**: Throughput (bytes/second) with confidence interval

### HTML Reports

View detailed interactive reports:

```bash
# Use the convenience script
./benches/view_report.sh

# Or open manually
open target/criterion/report/index.html
```

## Understanding the Benchmarks

### csv_lexer
Tests **tokenization only** - how fast the lexer converts text into tokens.
- Small: 3 rows, 3 columns (27 bytes)
- Medium: 101 rows, 5 columns (~4 KB)
- Large: 1001 rows, 8 columns (~60 KB)

### csv_parser
Tests the **complete parsing pipeline** - lexer + parser + tree construction.
Uses the same input sizes as csv_lexer.

### csv_quoted_strings
Tests parsing CSV files with **special cases**:
- Simple quoted strings
- Commas inside quotes
- Escaped quotes (`""`)
- Multi-line fields

### csv_varying_rows
Tests impact of **column count** on performance:
- 2, 5, 10, and 20 columns
- 50 rows each

## Comparing Runs

Criterion automatically compares each run against the previous baseline:

```
change: [-2.0% +0.5% +3.0%]  (p = 0.23 > 0.05)
No change in performance detected.
```

### Save a Baseline

```bash
cargo bench --bench csv_benchmark -- --save-baseline my-optimization
```

### Compare Against Baseline

```bash
cargo bench --bench csv_benchmark -- --baseline my-optimization
```

## Tips

1. **Close other applications** before benchmarking for accurate results
2. **Run on battery power** if on a laptop (some CPUs throttle differently when plugged in)
3. **Run multiple times** - Criterion will track trends over time
4. **Check the HTML reports** - they provide much more detail than terminal output

## Example Workflow

```bash
# 1. Save baseline before making changes
cargo bench --bench csv_benchmark -- --save-baseline before-opt

# 2. Make your code changes
# ... edit code ...

# 3. Run benchmarks and compare
cargo bench --bench csv_benchmark -- --baseline before-opt

# 4. View detailed results
./benches/view_report.sh
```

## Getting Help

- See `README.md` for detailed documentation
- See `EXAMPLE_OUTPUT.md` for sample results and analysis
- Check Criterion documentation: https://bheisler.github.io/criterion.rs/

## Troubleshooting

### "gnuplot not found" Warning
This is normal. Criterion will use the plotters backend instead, which works fine.

### Benchmarks Take Too Long
Criterion collects statistical samples to ensure accuracy. This is normal and ensures reliable results.

### Different Results Each Run
Some variation is normal. Look at the confidence intervals and the "change" percentage to understand if differences are significant.