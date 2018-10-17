extern crate proc_macro;
use {
    syn::{Token, DeriveInput, parse_macro_input},
    quote::*,
    proc_macro2,
    self::proc_macro::TokenStream,
};

#[proc_macro_derive(New)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let result = match ast.data {
        syn::Data::Struct(ref s) => new_for_struct(&ast, &s.fields),
        _ => panic!("doesn't work with unions yet"),
    };
    result.into()
}

fn new_for_struct(ast: &syn::DeriveInput,fields: &syn::Fields) -> proc_macro2::TokenStream
{
    match *fields {
        syn::Fields::Named(ref fields) => {
            new_impl(&ast, Some(&fields.named), true)
        },
        syn::Fields::Unit => {
            new_impl(&ast, None, false)
        },
        syn::Fields::Unnamed(ref fields) => {
            new_impl(&ast, Some(&fields.unnamed), false)
        },
    }
}

fn new_impl(ast: &syn::DeriveInput,
            fields: Option<&syn::punctuated::Punctuated<syn::Field, Token![,]>>,
            named: bool) -> proc_macro2::TokenStream
{
    let struct_name = &ast.ident;

    let unit = fields.is_none();
    let empty = Default::default();

    let fields: Vec<_> = fields.unwrap_or(&empty)
        .iter()
        .enumerate()
        .map(|(i, f)| FieldExt::new(f, i, named)).collect();

    let args = fields.iter().map(|f| f.as_arg());
    let inits = fields.iter().map(|f| f.as_init());

    let inits = if unit {
        quote!()
    } else if named {
        quote![ { #(#inits),* } ]
    } else {
        quote![ ( #(#inits),* ) ]
    };


    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (new, doc) = (
        syn::Ident::new("new", proc_macro2::Span::call_site()),
        format!("Constructs a new `{}`.", struct_name)
    );
    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[doc = #doc]
            pub fn #new(#(#args),*) -> Self {
                #struct_name #inits
            }
        }
    }
}

struct FieldExt<'a> {
    ty: &'a syn::Type,
    ident: syn::Ident,
    named: bool,
}
impl<'a> FieldExt<'a> {
    pub fn new(field: &'a syn::Field, idx: usize, named: bool) -> FieldExt<'a> {
        FieldExt {
            ty: &field.ty,
            ident: if named {
                field.ident.clone().unwrap()
            } else {
                syn::Ident::new(&format!("f{}", idx), proc_macro2::Span::call_site())
            },
            named: named,
        }
    }
    pub fn as_arg(&self) -> proc_macro2::TokenStream {
        let f_name = &self.ident;
        let ty = &self.ty;
        quote!(#f_name: #ty)
    }

    pub fn as_init(&self) -> proc_macro2::TokenStream {
        let f_name = &self.ident;
        let init =  quote!(#f_name);
        if self.named {
            quote!(#f_name: #init)
        } else {
            quote!(#init)
        }
    }
}
