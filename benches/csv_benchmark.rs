#![feature(try_blocks)]
#![feature(coerce_unsized)]
#![allow(unused_braces)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::InputStream;

// Include the generated CSV parser modules
#[path = "../tests/gen/csvlexer.rs"]
mod csvlexer;
#[path = "../tests/gen/csvlistener.rs"]
mod csvlistener;
#[path = "../tests/gen/csvparser.rs"]
mod csvparser;
#[path = "../tests/gen/csvvisitor.rs"]
mod csvvisitor;

use csvlexer::CSVLexer;
use csvparser::CSVParser;

fn small_csv() -> String { "h1,h2,h3\nd1,d2,d3\ne1,e2,e3\n".to_string() }

fn medium_csv() -> String {
    let mut csv = String::from("Header1,Header2,Header3,Header4,Header5\n");
    for i in 0..100 {
        csv.push_str(&format!(
            "data{},value{},item{},field{},column{}\n",
            i, i, i, i, i
        ));
    }
    csv
}

fn large_csv() -> String {
    let mut csv = String::from("ID,Name,Email,Age,City,Country,Score,Status\n");
    for i in 0..1000 {
        csv.push_str(&format!(
            "{},User{},user{}@example.com,{},City{},Country{},{},Active\n",
            i,
            i,
            i,
            20 + (i % 50),
            i % 10,
            i % 5,
            i % 100
        ));
    }
    csv
}

fn quoted_csv() -> String {
    r#"Name,Description,Value
"Simple","A simple value","123"
"With, comma","Contains a comma","456"
"With ""quotes""","Has escaped quotes","789"
"Multi
Line","Multiline content","999"
"#
    .to_string()
}

fn benchmark_lexer_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_lexer");

    let small = small_csv();
    group.throughput(Throughput::Bytes(small.len() as u64));
    group.bench_with_input(BenchmarkId::new("small", small.len()), &small, |b, data| {
        b.iter(|| {
            let data = black_box(data.as_str());
            let tf = ArenaCommonFactory::default();
            let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
            let token_stream = CommonTokenStream::new(lexer);
            black_box(token_stream.size())
        });
    });

    let medium = medium_csv();
    group.throughput(Throughput::Bytes(medium.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("medium", medium.len()),
        &medium,
        |b, data| {
            b.iter(|| {
                let data = black_box(data.as_str());
                let tf = ArenaCommonFactory::default();
                let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
                let token_stream = CommonTokenStream::new(lexer);
                black_box(token_stream.size())
            });
        },
    );

    let large = large_csv();
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_with_input(BenchmarkId::new("large", large.len()), &large, |b, data| {
        b.iter(|| {
            let data = black_box(data.as_str());
            let tf = ArenaCommonFactory::default();
            let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
            let token_stream = CommonTokenStream::new(lexer);
            black_box(token_stream.size())
        });
    });

    group.finish();
}

fn benchmark_full_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_parser");

    let small = small_csv();
    group.throughput(Throughput::Bytes(small.len() as u64));
    group.bench_with_input(BenchmarkId::new("small", small.len()), &small, |b, data| {
        b.iter(|| {
            let data = black_box(data.as_str());
            let tf = ArenaCommonFactory::default();
            let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
            let token_stream = CommonTokenStream::new(lexer);
            let mut parser = CSVParser::new(token_stream);
            let result = parser.csvFile();
            result.is_ok()
        });
    });

    let medium = medium_csv();
    group.throughput(Throughput::Bytes(medium.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("medium", medium.len()),
        &medium,
        |b, data| {
            b.iter(|| {
                let data = black_box(data.as_str());
                let tf = ArenaCommonFactory::default();
                let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
                let token_stream = CommonTokenStream::new(lexer);
                let mut parser = CSVParser::new(token_stream);
                let result = parser.csvFile();
                result.is_ok()
            });
        },
    );

    let large = large_csv();
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_with_input(BenchmarkId::new("large", large.len()), &large, |b, data| {
        b.iter(|| {
            let data = black_box(data.as_str());
            let tf = ArenaCommonFactory::default();
            let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
            let token_stream = CommonTokenStream::new(lexer);
            let mut parser = CSVParser::new(token_stream);
            let result = parser.csvFile();
            result.is_ok()
        });
    });

    group.finish();
}

fn benchmark_quoted_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_quoted_strings");

    let quoted = quoted_csv();
    group.throughput(Throughput::Bytes(quoted.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("quoted_fields", quoted.len()),
        &quoted,
        |b, data| {
            b.iter(|| {
                let data = black_box(data.as_str());
                let tf = ArenaCommonFactory::default();
                let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
                let token_stream = CommonTokenStream::new(lexer);
                let mut parser = CSVParser::new(token_stream);
                let result = parser.csvFile();
                result.is_ok()
            });
        },
    );

    group.finish();
}

fn benchmark_varying_row_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_varying_rows");

    for num_cols in [2, 5, 10, 20].iter() {
        let mut csv = String::new();
        // Header
        for i in 0..*num_cols {
            if i > 0 {
                csv.push(',');
            }
            csv.push_str(&format!("Col{}", i));
        }
        csv.push('\n');

        // 50 rows
        for _ in 0..50 {
            for i in 0..*num_cols {
                if i > 0 {
                    csv.push(',');
                }
                csv.push_str(&format!("data{}", i));
            }
            csv.push('\n');
        }

        group.throughput(Throughput::Bytes(csv.len() as u64));
        group.bench_with_input(BenchmarkId::new("columns", num_cols), &csv, |b, data| {
            b.iter(|| {
                let data = black_box(data.as_str());
                let tf = ArenaCommonFactory::default();
                let lexer = CSVLexer::new_with_token_factory(InputStream::new(data), &tf);
                let token_stream = CommonTokenStream::new(lexer);
                let mut parser = CSVParser::new(token_stream);
                let result = parser.csvFile();
                result.is_ok()
            });
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_plots();
    targets = benchmark_lexer_only, benchmark_full_parser, benchmark_quoted_strings, benchmark_varying_row_sizes
);
criterion_main!(benches);
