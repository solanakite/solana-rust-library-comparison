# Solana Rust Library Comparison

The same simple LiteSVM test task implemented with different Solana Rust testing libraries. Measured by Rust token count (identifiers, keywords, operators, punctuation — excluding comments and whitespace). Lower is less code.

**Task:** create a funded wallet, create a token mint, create an associated token account, and mint tokens to it.

## Libraries

- **solana-kite** — high-level helpers for LiteSVM tests
- **litesvm-utils** — Solana Foundation helpers (Brianna / brimigs)
- **Plain LiteSVM** — raw LiteSVM + SPL instruction builders

## Results

Run `python3 count-tokens.py` to regenerate. As of November 2026:

```
litesvm-utils     101 tokens
Kite              127 tokens (25.7% more code)
Plain LiteSVM     436 tokens (331.7% more code)
```

## Running

These are comparison snippets. They are not a full Cargo project — they illustrate the amount of code required for the same task.
