use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lang::{Context, Func, Program};

fn criterion_benchmark(c: &mut Criterion) {
    let benchmarks = [
        ("or", "true || false"),
        ("and", "true && false"),
        ("add", "1 + 2"),
        ("sub", "1 - 2"),
        ("mul", "1 * 2"),
        ("div", "1 / 2"),
        ("floor", "1 // 2"),
        ("mod", "1 % 2"),
        ("pow", "1 ** 2"),
        ("eq", "1 == 2"),
        ("neq", "1 != 2"),
        ("gt", "1 > 2"),
        ("lt", "1 < 2"),
        ("gte", "1 >= 2"),
        ("lte", "1 <= 2"),
        ("not", "!true"),
        ("neg", "-(1)"),
        ("list", "[1, 2, 3]"),
        ("index", "[1, 2, 3][1]"),
        ("ident", "test"),
        ("attr", "1.to_string"),
        ("func", "func()"),
        ("int", "1"),
        ("float", "1.62"),
        ("decimal", "d1.5"),
        ("string", "\"test\""),
        ("bool", "true"),
    ];

    c.bench_function("string_test", |b| b.iter(|| "test".to_string()));
    c.bench_function("format_str", |b| b.iter(|| format!("test {}", "test")));
    c.bench_function("format_string", |b| b.iter(|| format!("test {}", "test")));

    for (name, source) in black_box(&benchmarks) {
        c.bench_function(format!("compile {}", name).as_str(), |b| {
            b.iter(|| Program::compile(source))
        });
        c.bench_function(name, |b| {
            let program = Program::compile(source).unwrap();
            let ctx = Context::default();
            ctx.insert_value("test".into(), 1.into());
            ctx.insert_value("func".into(), Func::new(|_, _| Ok(1.into())).into());
            b.iter(|| program.execute(&ctx))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
