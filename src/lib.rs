use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::{visit_expr_mut, VisitMut};
use syn::{parse_macro_input, parse_quote, Expr, Lit};

struct LitVisitor;

impl VisitMut for LitVisitor {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        let mut tmp = None;
        if let Expr::Lit(l) = i {
            if let Lit::Str(l_s) = &l.lit {
                let q: Expr = parse_quote!(#l_s.to_string());
                tmp = Some(q);
                //println!("Processing {:?}", l_s.value());
            }
        }
        if let Some(q) = tmp {
            *i = q;
        } else {
            visit_expr_mut(self, i)
        }
    }
}

#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    //println!("{:?}", &input);
    let mut expr = parse_macro_input!(input as Expr);

    LitVisitor.visit_expr_mut(&mut expr);
    TokenStream::from(quote!(#expr))
}
