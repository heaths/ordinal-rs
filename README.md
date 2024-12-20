# Ordinal formatting

Format numbers as ordinals efficiently.
You can get the ordinal suffix e.g., "st", "nd", "rd", or "th" without allocations.

## Examples

Format a number as an ordinal, allocating a new `String`:

```rust
use ordinal_trait::Ordinal as _;
assert_eq!(12.to_ordinal(), "12th");
```

Get a number representing an ordinal you can use with comparisons and formatting.

```rust
use ordinal_trait::Ordinal as _;
let n = 12.to_number();
assert_eq!(*n, 12);
assert_eq!(format!("{n}"), "12th");
```

## Features

* **std**: (default) all functionality enabled.
* **alloc**: (default; enabled by **std**) all functionality enabled.

This crate supports `no_std`, though functionality is limited. If you disable default features, `no_std` will be enabled.
You can optionally add feature `alloc` (enabled by default with `std`) to enable all functionality without requiring `std`.

```bash
cargo add ordinal-trait --no-default-features --features alloc
```

## Performance

Compared to most other implementations that allocate a string just to check the last one or two characters, this implementation is much faster and does not allocate a string[^1].

![violin plot](docs/suffix_violin_plot.svg)

Formatting ordinals builds on this strategy and is faster than most other implementations:

![violin plot](docs/fmt_violin_plot.svg)

To [compare measurements](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html#baselines) across branches:

```bash
git checkout main
cargo bench -- --save-baseline main

git checkout feature
cargo bench -- --baseline main
```

[^1]: Criterion does not have built-in memory profiling but when I find an impl of `Measurement` to do so - or find time to write one - I'll include those stats as well; however, take into consideration that this implementation does not allocate a string at all for `suffix()`.
