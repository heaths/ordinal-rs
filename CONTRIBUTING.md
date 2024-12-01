# Contributing Guide

## Performance

To generate performance comparison reports, run:

```bash
cargo bench
```

To format the `suffix()` report displayed in the [README.md], run:

```bash
xsltproc -o docs/suffix_violin_plot.svg docs/style-svg.xslt target/criterion/suffix/report/violin.svg
```
