use proc_macro::{TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token, Result, LitInt, ExprLit, ExprPath};
use quote::{quote, quote_spanned};
use proc_macro_error::*;
use syn::spanned::Spanned;

struct BitSeqInput {
    bit_segments: Vec<BitSegment>,
}

enum BitSegment {
    Bits(syn::LitInt),
    Expr(syn::Expr, syn::LitInt),
}

impl Parse for BitSeqInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut bit_segments = Vec::new();
        let mut last_span = input.span();
        while !input.is_empty() {
            if input.peek(syn::Ident) || (input.peek(syn::LitInt) && input.peek2(Token![:])) {
                let val = if input.peek(syn::Ident) {
                    let ident = input.parse::<syn::Ident>()?;
                    syn::Expr::Path(ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: ident.into(),
                    })
                } else {
                    let num = input.parse::<syn::LitInt>()?;
                    syn::Expr::Lit(
                        ExprLit {
                            attrs: vec![],
                            lit: syn::Lit::Int(num),
                        })
                };

                if !input.peek(Token![:]) {
                    return Err(input.error("expected `:`"));
                }

                input.parse::<Token![:]>()?;

                if !input.peek(syn::LitInt) {
                    return Err(input.error("expected integer that specifies size of bit sequence"));
                }

                let size = input.parse::<syn::LitInt>()?;
                let segment = BitSegment::Expr(val, size);
                bit_segments.push(segment);
            } else if input.peek(syn::LitInt) {
                let num = input.parse::<syn::LitInt>()?;
                let num_string = num.to_string();

                // check for hexadecimal literal
                if num_string.starts_with("0x") {
                    let bit_len = (num_string.len() - 2) * 4;
                    let lit_len = LitInt::new(&bit_len.to_string(), num.span());

                    let expr = syn::Expr::Lit(
                        ExprLit {
                            attrs: vec![],
                            lit: syn::Lit::Int(num),
                        });

                    bit_segments.push(BitSegment::Expr(expr, lit_len));
                    continue;
                }

                // check for binary literal
                let is_binary = num_string.chars().all(|c| c == '0' || c == '1');
                if !is_binary {
                    let err = format!("expected bitsequence but got integer instead.");
                    return Err(syn::Error::new(last_span, err));
                }
                bit_segments.push(BitSegment::Bits(num))
            }
            // parse an expression segment
            else {
                return Err(input.error("expected bit sequence, hex or length defined expression"));
            }

            // update the previous token cursor
            last_span = input.span();
        }
        Ok(BitSeqInput {
            bit_segments,
        })
    }
}


#[proc_macro]
#[proc_macro_error]
pub fn bseq(input: TokenStream) -> TokenStream {
    use BitSegment::*;

    let input = parse_macro_input!(input as BitSeqInput);

    // construct shift token streams
    let mut bit_len = 0;
    let shifts: Vec<_> = input.bit_segments.iter().rev().map(|seq| {
        let (val, len) = match seq {
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
        let bit_len_lit = LitInt::new(&bit_len.to_string(), span);
        let res = quote_spanned!(span=> (#val) << #bit_len_lit);
        bit_len += len;
        res
    }).collect();

    let span = proc_macro2::Span::call_site();
    let mut macro_out = quote!(#(#shifts)|*);
    if macro_out.is_empty() {
        macro_out = quote_spanned!(span=> 0);
    }

    macro_out.into()
}

