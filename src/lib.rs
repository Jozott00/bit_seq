//! `bit_seq` provides procedural macros for generating bit sequences.
//!
//! # Overview
//!
//! This crate provides the macro [`bseq`], which allows for the creation of bit sequences
//! using a simple and intuitive syntax. Bit sequences can be created from raw binary values,
//! hexadecimal values, or even variable expressions. This makes the `bit_seq` crate a useful tool for
//! systems programming, hardware interfacing, or any application where bit manipulation is common.
//!
//! `bit_seq` also provides [`bseq_8`], [`bseq_16`], [`bseq_32`], [`bseq_64`] and [`bseq_128`] to
//! simply type mixing.
//!
//! # Examples
//!
//! The following examples illustrate some of the ways `bseq!` can be used.
//!
//! ## Raw Bit Sequences
//!
//! ```
//! use bit_seq::bseq;
//!
//! let t = bseq!(0110 01 0 1);
//! assert_eq!(t, 0b0110_01_0_1);
//! ```
//!
//! ## Hex Values
//!
//! Hexadecimal values are interpreted as 4-bit sequences.
//!
//! ```
//! use bit_seq::bseq;
//!
//! let t = bseq!(01 0x1f);
//! assert_eq!(t, 0b01_0001_1111);
//! ```
//!
//! ## Length Expressions
//!
//! Length expressions take the form `<val>:<len>`, where `<len>` is the number of bits from `<val>` to be used.
//!
//! ```
//! use bit_seq::bseq;
//!
//! let t = bseq!(3:1 0 0xf:2);
//! assert_eq!(t, 0b1_0_11);
//! ```
//!
//! ## Variable Interpolation
//!
//! Variable interpolation is supported for length expressions.
//!
//! ```
//! use bit_seq::bseq;
//! let var = 0xf;
//! let t = bseq!(10 var:2);
//! assert_eq!(t, 0b10_11);
//! ```
//!
//! ## Unary Operations
//!
//! The bseq syntax supports unary operations for length expressions. This simplifies bit sequences like
//! `0b111111`.
//!
//! ```
//! use bit_seq::bseq;
//! // bit negation
//! assert_eq!(bseq!(!0:6), 0b111111);
//!
//! // numerical negation with variable interpolation
//! let var = 1;
//! assert_eq!(bseq!(-var:8), 0xff);
//! ```
//!
//! # Performance
//!
//! The `bseq!` macro compiles down to standard bit manipulation operations, meaning there is no runtime overhead to using it.


use proc_macro::TokenStream;

use proc_macro_error::*;
use quote::{quote, quote_spanned};
use syn::{LitInt, parse_macro_input, parse_quote, Type};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

use crate::bit_seq_input::{BitSegment::{self, *}, BitSeqInput};

mod bit_seq_input;


/// `bseq` is a procedural macro for creating bit sequences.
///
/// This macro enables the generation of bit sequences using a simple syntax.
/// Bit sequences can be specified directly, through hex values, or by using identifiers or integers
/// each with a specific length. This proves especially useful in systems programming and when interacting
/// with low-level hardware or protocols where bit manipulation is a common requirement.
///
/// # Examples
///
/// #### Direct raw bit sequence:
/// ```
/// use bit_seq::bseq;
///
/// let t = bseq!(0110 01 0 1);
/// assert_eq!(t, 0b0110_01_0_1);
/// ```
///
/// #### Using hex values:
///
/// Hex values conveniently add 4 bits per hexadecimal place.
/// ```
/// use bit_seq::bseq;
///
/// let t = bseq!(01 0x1f);
/// assert_eq!(t, 0b01_0001_1111);
/// ```
///
/// #### Using value length expression:
///
/// Employ the format `<val>:<len>` where `len` specifies how many of the
/// least significant bits from `val` should be used.
/// ```
/// use bit_seq::bseq;
///
/// let t = bseq!(3:1 0 0xf:2);
/// assert_eq!(t, 0b1_0_11);
/// ```
///
/// #### Using variable length expression:
///
/// It is also possible to interpolate outer variables for length expressions.
/// ```
/// use bit_seq::bseq;
/// let var = 0xf;
/// let t = bseq!(10 var:2);
/// assert_eq!(t, 0b10_11);
/// ```
///
/// ## Unary Operations
///
/// The bseq syntax supports unary operations for length expressions. This simplifies bit sequences like
/// `0b111111`.
///
/// ```
/// use bit_seq::bseq;
/// // bit negation
/// assert_eq!(bseq!(!0:6), 0b111111);
///
/// // numerical negation with variable interpolation
/// let var = 1;
/// assert_eq!(bseq!(-var:8), 0xff);
/// ```
///
/// Note: Since the macros are compiled into common bit manipulation operations,
/// the usage of this macro doesn't introduce additional runtime overhead.
///
/// The macro outputs a numerical value with the appropriate bits set, providing an
/// efficient method to generate specific bit sequences.
#[proc_macro]
#[proc_macro_error]
pub fn bseq(input: TokenStream) -> TokenStream {
    process(input, None)
}

/// The `bseq_8` procedural macro is specifically tailored for creating 8-bit sequences.
///
/// It is primarily utilized when there's a need to accommodate variable types different from those
/// provided by the macro or when working with variable-length expressions involving mixed types.
///
/// For instance, the following example would fail to compile due to a type mismatch:
/// ```compile_fail
/// use bit_seq::bseq;
/// let foo: u32 = 4;
/// let bar: u64 = 2;
/// let t: u8 = bseq!(foo:5 bar:3);
/// ```
///
/// The `bseq_8` macro addresses such scenarios, as demonstrated below:
/// ```
/// use bit_seq::bseq_8;
/// let foo: u32 = 4;
/// let bar: u64 = 2;
/// let t: u8 = bseq_8!(foo:5 bar:3);
/// ```
///
/// It is important to note that `bseq_8` effectively performs as `bseq!(...)`, albeit with intermediate type casts.
/// For a comprehensive understanding on the usage of `bseq_8`, please refer to the [`bseq`] documentation.
#[proc_macro]
pub fn bseq_8(input: TokenStream) -> TokenStream {
    let ty: Type = parse_quote!(u8);
    process(input, Some(ty))
}

/// The `bseq_16` procedural macro is specifically tailored for creating 16-bit sequences.
///
/// It is primarily utilized when there's a need to accommodate variable types different from those
/// provided by the macro or when working with variable-length expressions involving mixed types.
///
/// For instance, the following example would fail to compile due to a type mismatch:
/// ```compile_fail
/// use bit_seq::bseq;
/// let foo: u32 = 4;
/// let bar: u64 = 2;
/// let t: u16 = bseq!(foo:5 bar:11);
/// ```
///
/// The `bseq_16` macro addresses such scenarios, as demonstrated below:
/// ```
/// use bit_seq::bseq_16;
/// let foo: u32 = 4;
/// let bar: u64 = 2;
/// let t: u16 = bseq_16!(foo:5 bar:11);
/// ```
///
/// It is important to note that `bseq_16` effectively performs as `bseq!(...)`, albeit with intermediate type casts.
/// For a comprehensive understanding on the usage of `bseq_16`, please refer to the [`bseq`] documentation.
#[proc_macro]
pub fn bseq_16(input: TokenStream) -> TokenStream {
    let ty: Type = parse_quote!(u16);
    process(input, Some(ty))
}

/// The `bseq_32` procedural macro is specifically tailored for creating 32-bit sequences.
///
/// It is primarily utilized when there's a need to accommodate variable types different from those
/// provided by the macro or when working with variable-length expressions involving mixed types.
///
/// For instance, the following example would fail to compile due to a type mismatch:
/// ```compile_fail
/// use bit_seq::bseq;
/// let foo: u8 = 4;
/// let bar: u64 = 2;
/// let t: u32 = bseq!(foo:5 bar:27);
/// ```
///
/// The `bseq_32` macro addresses such scenarios, as demonstrated below:
/// ```
/// use bit_seq::bseq_32;
/// let foo: u8 = 4;
/// let bar: u64 = 2;
/// let t: u32 = bseq_32!(foo:5 bar:27);
/// ```
///
/// It is important to note that `bseq_32` effectively performs as `bseq!(...)`, albeit with intermediate type casts.
/// For a comprehensive understanding on the usage of `bseq_32`, please refer to the [`bseq`] documentation.
#[proc_macro]
pub fn bseq_32(input: TokenStream) -> TokenStream {
    let ty: Type = parse_quote!(u32);
    process(input, Some(ty))
}

/// The `bseq_64` procedural macro is designed for creating 64-bit sequences.
///
/// It is primarily utilized when dealing with variable types that are different from those
/// provided by the macro or when working with variable-length expressions that involve mixed types.
///
/// The following example won't compile due to a type mismatch:
/// ```compile_fail
/// use bit_seq::bseq;
/// let foo: u32 = 4;
/// let bar: u16 = 2;
/// let t: u64 = bseq!(foo:5 bar:59);
/// ```
///
/// The `bseq_64` macro provides a solution for such cases:
/// ```
/// use bit_seq::bseq_64;
/// let foo: u32 = 4;
/// let bar: u16 = 2;
/// let t: u64 = bseq_64!(foo:5 bar:59);
/// ```
///
/// Note that `bseq_64` is essentially `bseq!(...)` with intermediate type casts. For details on how to use `bseq_64`,
/// please refer to the [`bseq`] documentation.
#[proc_macro]
pub fn bseq_64(input: TokenStream) -> TokenStream {
    let ty: Type = parse_quote!(u64);
    process(input, Some(ty))
}

/// The `bseq_128` procedural macro is designed for creating 128-bit sequences.
///
/// It is primarily utilized when dealing with variable types that are different from those
/// provided by the macro or when working with variable-length expressions that involve mixed types.
///
/// The following example won't compile due to a type mismatch:
/// ```compile_fail
/// use bit_seq::bseq;
/// let foo: u32 = 4;
/// let bar: u64 = 2;
/// let t: u128 = bseq!(foo:5 bar:59);
/// ```
///
/// The `bseq_128` macro provides a solution for such cases:
/// ```
/// use bit_seq::bseq_128;
/// let foo: u32 = 4;
/// let bar: u64 = 2;
/// let t: u128 = bseq_128!(foo:5 bar:59);
/// ```
///
/// Note that `bseq_128` is essentially `bseq!(...)` with intermediate type casts. For details on how to use `bseq_128`,
/// please refer to the [`bseq`] documentation.
#[proc_macro]
pub fn bseq_128(input: TokenStream) -> TokenStream {
    let ty: Type = parse_quote!(u128);
    process(input, Some(ty))
}

/// Processes the `bseq` input stream with a specified variable type.
///
/// `bseq!` has variable type None \
/// `bseq8!` has variable type Option<Type<u8>> \
/// ...
fn process(input: TokenStream, var_type: Option<Type>) -> TokenStream {
    // parse input
    let input = parse_macro_input!(input as BitSeqInput);

    // construct shift token streams
    let mut bit_len = 0;
    let shifts: Vec<_> = input.segments()
        .iter().rev()
        .map(|seg| map_segment(seg, &mut bit_len, &var_type))
        .collect();

    // combine all shift segments
    let span = proc_macro2::Span::call_site();

    let mut macro_out = if let Some(ty) = var_type {
        quote!((#(#shifts)|*) as #ty)
    } else {
        quote!(#(#shifts)|*)
    };

    if macro_out.is_empty() {
        // if no input provided, result is 0
        macro_out = quote_spanned!(span=> 0);
    }

    macro_out.into()
}


fn map_segment(seg: &BitSegment, curr_bit_len: &mut usize, expr_type: &Option<Type>) -> TokenStream2 {
    let (val, len) = match seg {
        Bits(bits) => {
            let b = bits.to_string();
            let num = usize::from_str_radix(&b, 2).unwrap();
            let num_lit = LitInt::new(&num.to_string(), b.span());
            let span = bits.span();
            let rep = quote_spanned!(span=> #num_lit);
            (rep, b.len())
        }
        Expr(expr, len_lit) => {
            let len: usize = len_lit.base10_parse().unwrap_or_else(|_| abort!(len_lit, "Couldn't be parsed!"));
            let mask: u128 = (1 << len) - 1;
            let mask_lit = LitInt::new(&mask.to_string(), expr.span());
            let span = expr.span();

            let rep = if let Some(ty) = expr_type {
                quote_spanned!(span=> (#expr as #ty) & #mask_lit)
            } else {
                quote_spanned!(span=> #expr & #mask_lit)
            };

            (rep, len)
        }
    };

    let span = val.span();
    let bit_len_lit = LitInt::new(&curr_bit_len.to_string(), span);
    let res = quote_spanned!(span=> (#val) << #bit_len_lit);
    *curr_bit_len += len;
    res
}