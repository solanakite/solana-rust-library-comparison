# Solana Rust Library Comparison

The same simple LiteSVM test task implemented with different Solana Rust testing libraries. Measured by Rust token count using the real Rust lexer (`proc_macro2`, which uses the same lexer as rustc). Lower is less code. Comments and whitespace are excluded.

**Task:** create a funded wallet, create a token mint, create an associated token account, and mint tokens to it.

## Libraries

- **solana-kite** — high-level helpers for LiteSVM tests
- **Plain LiteSVM** — raw LiteSVM + SPL instruction builders

## Results

Run `cargo run --release -- kite.rs plain-litesvm.rs` from the `count-tokens` directory. As of November 2026, using the real Rust lexer:

```
Kite              120 tokens
Plain LiteSVM     403 tokens (235.8% more code)
```

## Running

These are comparison snippets. They are not a full Cargo project — they illustrate the amount of code required for the same task.

The token counter is in `count-tokens/` and uses `proc_macro2::TokenStream` (the same lexer as rustc) to count tokens.
