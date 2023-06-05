# bit_seq
---
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Jozott00/bit_seq/tests.yml?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/bit_seq?logo=rust&style=flat-square)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bit_seq-66c2a5?style=flat-square&labelColor=555555&logo=docs.rs">](https://docs.rs/bit_seq)

bit_seq is a convenient Rust crate that provides a procedural macro for creating bit sequences. This crate simplifies
the
generation of bit sequences, increasing readability and reducing the potential for errors. Bit sequences can be
specified directly, via hex values, or through identifiers or integers with a specific length. It is particularly useful
in systems programming and lower-level hardware or protocol interfacing where bit manipulation is common.

## Features

- Generate bit sequences using simple syntax
- Specify bit sequences directly or via hex values
- Use identifiers or integers to define bit sequence with a specific length
- Interpolate outer variables in length expressions
- Compiles to common bit manipulation operations, so using this crate does not add runtime overhead

## Usage

First, add the following in you project cmd-line:

```bash
cargo add bit_seq
```

Then import the crate in your Rust file:

```rust
use bit_seq::bseq;
```

Here are some examples of how to use the `bseq` macro:

```rust
// Direct raw bit sequence
let t = bseq!(0110 01 0 1);
assert_eq!(t, 0b0110_01_0_1);

// Using hex values
let t = bseq!(01 0x1f);
assert_eq!(t, 0b01_0001_1111);

// Using value length expression
let t = bseq!(3:1 0 0xf:2);
assert_eq!(t, 0b1_0_11);

// Using variable length expression
let var = 0xf;
let t = bseq!(10 var:2);
assert_eq!(t, 0b10_11);

// Using mixed variable types
let var_64: u64 = 0xf;
let var_16: u16 = 0xf;
let t = bseq_8!(var_16:4 var_64:4);
assert_eq!(t, 0xff);

// Using unary operators 
assert_eq!(bseq!(!0:6), 0b111111);
```

## Documentation

You can view the full API documentation [here](https://docs.rs/bit_seq).

## Contributing

Contributions to bit_seq are welcome! Please submit a pull request or create an issue on
the [GitHub page](https://github.com/Jozott00/bit_seq).

## License

This project is licensed under the MIT license.
