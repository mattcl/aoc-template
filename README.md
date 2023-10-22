# An Advent of Code cargo-generate template

This template generates a rust project that is compatible with the
[specifications](https://github.com/mattcl/aoc-ci-bencher/blob/master/SPECIFICATION.md)
for supporting comparative benchmarks between different advent of code
implementations.

More information about how to use the generated project can be found in the
generated README (which is in the source of the template).

The _template_ is MIT-licensed. Pick whatever license suits you for The
generated code.

Rather use python instead? That template is
[here](https://github.com/mattcl/aoc-python-template)


## Prerequisites

1. git
2. rust >=1.66 (1.73 preferred)
3. [cargo-generate](https://crates.io/crates/cargo-generate)
4. [just](https://github.com/casey/just#packages)


## Usage

Generate the template using the following command and follow the prompts:
```
cargo generate -g https://github.com/mattcl/aoc-template.git
```
