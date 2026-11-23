use std::env;
use std::fs;

/// Count tokens using the real Rust lexer (proc_macro2 / rustc_lexer).
/// Excludes comments and whitespace. Mirrors the TypeScript Compiler API approach.
fn count_tokens(source: &str) -> usize {
    let tokens = match source.parse::<proc_macro2::TokenStream>() {
        Ok(t) => t,
        Err(_) => {
            // Incomplete snippets: wrap so the lexer still sees the body
            let wrapped = format!("fn _bench() {{ {} }}", source);
            match wrapped.parse::<proc_macro2::TokenStream>() {
                Ok(t) => t,
                Err(_) => return 0,
            }
        }
    };
    fn walk(stream: proc_macro2::TokenStream) -> usize {
        let mut n = 0;
        for t in stream {
            n += 1;
            if let proc_macro2::TokenTree::Group(g) = t {
                n += walk(g.stream());
            }
        }
        n
    }
    walk(tokens)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: count-tokens <file.rs>...");
        std::process::exit(1);
    }
    let mut results = Vec::new();
    for path in &args {
        let src = fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("{}: {}", path, e);
            String::new()
        });
        let n = count_tokens(&src);
        results.push((n, path.clone()));
    }
    results.sort_by_key(|r| r.0);
    let min = results.first().map(|r| r.0).unwrap_or(0);
    println!("\n=== Token Count Results (real Rust lexer) ===\n");
    for (n, path) in &results {
        let extra = if *n > min {
            let pct = (*n as f64 - min as f64) / min as f64 * 100.0;
            format!(" ({:.1}% more code)", pct)
        } else {
            String::new()
        };
        println!("{:<20} {:>4} tokens{}", path, n, extra);
    }
    println!();
}
