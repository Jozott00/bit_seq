use proc_macro::TokenStream;

use proc_macro_error::*;
use quote::{quote, quote_spanned};
use syn::{LitInt, parse_macro_input};
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
/// Note: Since the macros are compiled into common bit manipulation operations,
/// the usage of this macro doesn't introduce additional runtime overhead.
///
/// The macro outputs a numerical value with the appropriate bits set, providing an
/// efficient method to generate specific bit sequences.
#[proc_macro]
#[proc_macro_error]
pub fn bseq(input: TokenStream) -> TokenStream {
    // parse input
    let input = parse_macro_input!(input as BitSeqInput);

    // construct shift token streams
    let mut bit_len = 0;
    let shifts: Vec<_> = input.segments()
        .iter().rev()
        .map(|seg| map_segment(seg, &mut bit_len))
        .collect();

    // combine all shift segments
    let span = proc_macro2::Span::call_site();
    let mut macro_out = quote!(#(#shifts)|*);
    if macro_out.is_empty() {
        // if no input provided, result is 0
        macro_out = quote_spanned!(span=> 0);
    }

    macro_out.into()
}


fn map_segment(seg: &BitSegment, curr_bit_len: &mut usize) -> TokenStream2 {
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
            let rep = quote_spanned!(span=> #expr & #mask_lit);
            (rep, len)
        }
    };

    let span = val.span();
    let bit_len_lit = LitInt::new(&curr_bit_len.to_string(), span);
    let res = quote_spanned!(span=> (#val) << #bit_len_lit);
    *curr_bit_len += len;
    res
}