use proc_macro::TokenStream;
use quote::quote;
use syn::{parenthesized, parse_macro_input, DeriveInput, ExprLit, GenericArgument, Lit, PathArguments, Type};

#[proc_macro_derive(Rviewerable, attributes(rviewer))]
pub fn rviewerable_derive(stream: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(stream);

    let name = &ast.ident;
    let named_fields = match &ast.data {
        syn::Data::Struct(x) => &x.fields,
        _ => panic!(),
    };
    let field_idents = named_fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_idents_ = named_fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_names = named_fields.iter().map(|f| {
        if let Some(rename) = f
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("rviewer") {
                    match attr.parse_args::<ExprLit>().unwrap().lit {
                        Lit::Str(x) => Some(x.value()),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .next()
        {
            rename
        } else {
            f.ident.as_ref().unwrap().to_string()[..1].to_string()
        }
    });

    let mut option_fields = Vec::new();
    let mut vec_fields = Vec::new();
    let mut raw_fields = Vec::new();

    for f in named_fields.iter() {
        match &f.ty {
            Type::Path(p) => {
                if p.path.segments[0].ident == "Option" {
                    option_fields.push(f);
                } else if p.path.segments[0].ident == "Vec" {
                    vec_fields.push(f);
                } else {
                    raw_fields.push(f);
                }
            }
            _ => panic!(),
        }
    }

    let option_field_idents = option_fields.iter().map(|f| f.ident.as_ref().unwrap());
    let option_field_types = option_fields.iter().map(|f| match &f.ty {
        Type::Path(p) => match &p.path.segments[0].arguments {
            PathArguments::AngleBracketed(args) => match args.args.first().unwrap() {
                GenericArgument::Type(t) => Some(t),
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!(),
    });

    let vec_field_idents = vec_fields.iter().map(|f| f.ident.as_ref().unwrap());
    let vec_field_types = vec_fields.iter().map(|f| match &f.ty {
        Type::Path(p) => match &p.path.segments[0].arguments {
            PathArguments::AngleBracketed(args) => match args.args.first().unwrap() {
                GenericArgument::Type(t) => Some(t),
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!(),
    });

    let raw_field_idents = raw_fields.iter().map(|f| f.ident.as_ref().unwrap());
    let raw_field_types = raw_fields.iter().map(|f| &f.ty);

    let mut rviewer_name: Option<String> = None;

    for attr in ast.attrs.iter() {
        if attr.path().is_ident("rviewer") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: ExprLit = content.parse()?;
                    if let Lit::Str(x) = lit.lit {
                        rviewer_name = Some(x.value());
                    }
                }
                Ok(())
            })
            .unwrap();
        }
    }
    let sep = if rviewer_name.is_some() { " " } else { "\n" };
    let prefix = match &rviewer_name {
        Some(x) => format!("{} ", x),
        None => String::new(),
    };
    let suffix = match &rviewer_name {
        Some(_) => "\n",
        None => "",
    };

    let output = quote! {
        impl Rviewerable for #name {
            fn new() -> Self {
                Self {
                    #(#field_idents_: Default::default(),)*
                }
            }

            fn draw<T: Write>(self, writer: &mut T) {
                write!(writer, #prefix).unwrap();
                #(self.#field_idents.print_sep(#field_names, writer, #sep);)*
                write!(writer, #suffix).unwrap();
            }
        }

        impl #name {
            #(pub fn #option_field_idents(mut self, arg: impl Into<#option_field_types>) -> Self {
                self.#option_field_idents = Some(arg.into());
                self
            })*
            #(pub fn #vec_field_idents(mut self, arg: impl Into<#vec_field_types>) -> Self {
                self.#vec_field_idents.push(arg.into());
                self
            })*

            #(pub fn #raw_field_idents(mut self, arg: impl Into<#raw_field_types>) -> Self {
                self.#raw_field_idents = arg.into();
                self
            })*
        }
    };

    output.into()
}
