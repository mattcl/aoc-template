# An Advent of Code cargo-generate template

This template generates a rust project that is compatible with the
[specifications](https://github.com/mattcl/aoc-benchmarks/blob/master/SPECIFICATION.md)
for supporting comparative benchmarks between different advent of code
implementations.

More information about how to use the generated project can be found in the
generated README (which is in the source of the template).

The _template_ is MIT-licensed. Pick whatever license suits you for the
generated code.

Rather use python instead? That template is
[here](https://github.com/mattcl/aoc-python-template)


## Prerequisites

1. git
2. rust >=1.77 (>= 1.82 preferred)
3. [cargo-generate](https://crates.io/crates/cargo-generate)
4. [just](https://github.com/casey/just#packages)

\* On OSX and others, you may need to install cargo generate with the following
(see [this issue](https://github.com/cargo-generate/cargo-generate/issues/1318)):

```
cargo install cargo-generate --features "vendored-openssl"
```


## Usage

Generate the template using the following command and follow the prompts:
```
cargo generate -g https://github.com/mattcl/aoc-template.git
```
