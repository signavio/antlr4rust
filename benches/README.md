# CSV Parser Benchmarks

This directory contains benchmarks for the ANTLR4 Rust CSV parser implementation using [Criterion.rs](https://github.com/bheisler/criterion.rs).

## Running the Benchmarks

To run all benchmarks:

```bash
cargo bench
```

To run a specific benchmark group:

```bash
# Lexer only benchmarks
cargo bench csv_lexer

# Full parser benchmarks
cargo bench csv_parser

# Quoted strings benchmarks
cargo bench csv_quoted_strings

# Varying row sizes benchmarks
cargo bench csv_varying_rows
```

## Benchmark Groups

### `csv_lexer`
Tests the lexer performance in isolation (tokenization only):
- **small**: 3 rows with 3 columns
- **medium**: 101 rows with 5 columns (~100 data rows)
- **large**: 1001 rows with 8 columns (~1000 data rows)

### `csv_parser`
Tests the full parsing pipeline (lexer + parser + tree construction):
- **small**: 3 rows with 3 columns
- **medium**: 101 rows with 5 columns
- **large**: 1001 rows with 8 columns

### `csv_quoted_strings`
Tests parsing of CSV files with quoted fields, including:
- Simple quoted strings
- Quoted strings containing commas
- Quoted strings with escaped quotes
- Multi-line quoted fields

### `csv_varying_rows`
Tests performance with varying numbers of columns per row:
- 2, 5, 10, and 20 columns
- 50 rows for each configuration

## HTML Reports

Criterion generates detailed HTML reports with plots and statistics. After running the benchmarks, you can view the reports at:

```
target/criterion/report/index.html
```

You can also use the convenience script to open the report:

```bash
./benches/view_report.sh
```

The HTML reports include:
- Performance comparisons between runs
- Throughput measurements (bytes processed per second)
- Statistical analysis of results with confidence intervals
- Regression detection
- Interactive plots and charts

## Interpreting Results

Each benchmark reports:
- **Time**: Mean execution time with confidence intervals
- **Throughput**: Bytes processed per second
- **Change**: Performance change compared to the previous run

Lower execution times and higher throughput indicate better performance.

## Comparing Performance

Criterion automatically compares your current run against the previous baseline. To save a new baseline:

```bash
cargo bench -- --save-baseline my-baseline
```

To compare against a specific baseline:

```bash
cargo bench -- --baseline my-baseline
```

## Profiling

For detailed profiling, you can use cargo-flamegraph:

```bash
cargo install flamegraph
cargo flamegraph --bench csv_benchmark -- --bench
```

Note: On macOS, you may need to run with `sudo`.

## Notes

- The benchmarks use `black_box()` to prevent compiler optimizations from eliminating the actual work
- Each benchmark uses `ArenaCommonFactory` for efficient memory allocation
- Throughput is measured in bytes of input CSV data processed per second