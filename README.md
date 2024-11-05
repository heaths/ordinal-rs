# Formatting ordinals

An example of formatting ordinals e.g., 1st, 2nd, 3rd, 12th, etc., without string allocations.

## Examples

Format a number as an ordinal, allocating a new `String`:

```rust
use ordinal::Ordinal as _;
assert_eq!(12.to_ordinal(), "12th");
```

Get a number representing an ordinal you can use with comparisons and formatting.

```rust
use ordinal::Ordinal as _;
let n = 12.to_number();
assert_eq!(*n, 12);
assert_eq!(format!("{n}"), "12th");
```

## Performance

Compared to [github.com/gleich/ordinal](https://github.com/gleich/ordinal/commit/a4bf9bdc37d05940f87d8ceea1c4b47cda0da5b4) or even [PR #3](https://github.com/gleich/ordinal/pull/3), this is much faster and does not allocate a string[^1].

![violin plot](docs/suffix_violin_plot.svg)

To [compare measurements](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html#baselines) across branches:

```bash
git checkout main
cargo bench -- --save-baseline main

git checkout feature
cargo bench -- --baseline main
```

[^1]: Criterion does not have built-in memory profiling but when I find an impl of `Measurement` to do so - or find time to write one - I'll include those stats as well; however, take into consideration that this implementation does not allocate a string at all for `suffix()`.
