use quote::{quote, ToTokens};
use syn::{Expr, ExprLit, ExprPath, ExprUnary, LitInt, parse, Result, Token};
use syn::parse::{Parse, ParseStream, Peek};
use syn::token::{Colon, Token};

pub struct BitSeqInput {
    bit_segments: Vec<BitSegment>,
}

pub enum BitSegment {
    Bits(syn::LitInt),
    Expr(syn::Expr, syn::LitInt),
}

impl BitSeqInput {
    pub fn segments(&self) -> &Vec<BitSegment> {
        &self.bit_segments
    }

    fn parse_length_definition(input: &ParseStream) -> Result<syn::LitInt> {
        if !input.peek(Token![:]) {
            return Err(input.error("expected `:`"));
        }

        input.parse::<Token![:]>()?;

        if !input.peek(syn::LitInt) {
            return Err(input.error("expected integer that specifies size of bit sequence"));
        }

        input.parse::<syn::LitInt>()
    }

    // parse unary operator to bit segment expression
    fn parse_unary(input: &ParseStream) -> Result<BitSegment> {
        let expr = input.parse::<Expr>()?;
        let size = BitSeqInput::parse_length_definition(&input)?;
        Ok(BitSegment::Expr(expr, size))
    }

    // parse ident and num expression to bit segement
    fn parse_expr(input: &ParseStream) -> Result<BitSegment> {
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

        let size = BitSeqInput::parse_length_definition(&input)?;
        Ok(BitSegment::Expr(val, size))
    }

    // parse raw bits
    fn parse_bits(input: &ParseStream) -> Result<BitSegment> {
        let num = input.parse::<syn::LitInt>()?;
        let num_string = num.to_string();

        if num_string.starts_with('-') {
            return Err(syn::Error::new(num.span(), "negative numbers are not allowed"));
        }

        // check for hexadecimal literal
        if num_string.starts_with("0x") {
            let bit_len = (num_string.len() - 2) * 4;
            let lit_len = LitInt::new(&bit_len.to_string(), num.span());

            let expr = syn::Expr::Lit(
                ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Int(num),
                });

            return Ok(BitSegment::Expr(expr, lit_len));
        }

        // check for binary literal
        let is_binary = num_string.chars().all(|c| c == '0' || c == '1');
        if !is_binary {
            let err = format!("expected bit sequence but got integer instead.");
            return Err(syn::Error::new(num.span(), err));
        }
        Ok(BitSegment::Bits(num))
    }
}

impl Parse for BitSeqInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut bit_segments = Vec::new();

        while !input.is_empty() {
            if peek_expr_with_token(|expr| matches!(expr, Expr::Unary(_)), Token![:], input) {
                let segment = BitSeqInput::parse_unary(&input)?;
                bit_segments.push(segment);
            } else if input.peek(syn::Ident)
                || (input.peek(syn::LitInt) && input.peek2(Token![:]))
            {
                let segment = BitSeqInput::parse_expr(&input)?;
                bit_segments.push(segment);
            } else if input.peek(syn::LitInt) {
                let segment = BitSeqInput::parse_bits(&input)?;
                bit_segments.push(segment);
            }
            // parse an expression segment
            else {
                return Err(input.error("expected bit sequence, hex or length defined expression"));
            }
        }

        Ok(BitSeqInput {
            bit_segments,
        })
    }
}

// Helper
fn peek_expr_with_token<T: Peek>(check: fn(Expr) -> bool, token: T, input: ParseStream) -> bool {
    let forked = input.fork();
    let expr_check = match forked.parse::<Expr>() {
        Ok(expr) => check(expr),
        Err(_) => false,
    };

    expr_check && forked.peek(token)
}