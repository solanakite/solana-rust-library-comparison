#!/usr/bin/env python3
"""Rough Rust token counter: split on whitespace and punctuation, skip comments."""
import re
from pathlib import Path

files = [
    ("kite.rs", "Kite"),
    ("litesvm-utils.rs", "litesvm-utils"),
    ("plain-litesvm.rs", "Plain LiteSVM"),
]

def count(path: str) -> int:
    text = Path(path).read_text()
    text = re.sub(r"//.*?$", "", text, flags=re.M)
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    tokens = re.findall(r"[A-Za-z_][A-Za-z0-9_]*|::|->|=>|!=|==|<=|>=|[0-9_]+|[{}()\[\],.;&*<>!=+\-/%]", text)
    return len(tokens)

results = [(name, count(f)) for f, name in files]
results.sort(key=lambda x: x[1])
min_t = results[0][1]
print("\n=== Token Count Results ===\n")
for name, n in results:
    extra = ""
    if n > min_t:
        pct = (n - min_t) / min_t * 100
        extra = f" ({pct:.1f}% more code)"
    print(f"{name:<20} {n:>4} tokens{extra}")
print()
