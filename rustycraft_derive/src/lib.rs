use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, ItemStruct, Token, Type};

struct DeriveIntoUpdateFieldsArgs {
    offset: Expr,
    tag: Expr,
}
#[derive(Eq, PartialEq)]
struct DeriveIntoUpdateFieldsTag;

impl Parse for DeriveIntoUpdateFieldsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut offset = None;
        let mut tag = None;
        for _ in 0..2 {
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            if &format_ident!("offset") == &ident {
                offset = Some(input.parse()?);
            } else if &format_ident!("tag") == &ident {
                tag = Some(input.parse()?);
            }
            input.parse::<Option<Token![,]>>()?;
        }
        Ok(DeriveIntoUpdateFieldsArgs {
            offset: offset.unwrap(),
            tag: tag.unwrap(),
        })
    }
}

#[proc_macro_derive(IntoUpdateFields, attributes(nested, meta))]
pub fn derive_into_update_fields(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let name = item.ident;
    let mut from_body = vec![];
    let mut nested = quote! {let mut item = UpdateFields::new();};
    let mut args = None;
    for attr in item.attrs {
        match attr.parse_args::<DeriveIntoUpdateFieldsArgs>() {
            Ok(meta) => {
                args = Some(meta);
            }
            Err(err) => return err.into_compile_error().into(),
        }
    }
    let DeriveIntoUpdateFieldsArgs { offset, tag } = args.unwrap();
    'o: for field in item.fields {
        let field_name = field.ident.expect("Tuple struct not supported.");
        let field_ty = field.ty;
        for attr in field.attrs {
            if let Ok(meta) = attr.parse_meta() {
                if meta.path().is_ident(&format_ident!("nested")) {
                    nested = quote! {
                        let mut item = UpdateFields::from(from_value.#field_name);
                    };
                    continue 'o;
                }
            }
        }
        if let Type::Array(ref arr) = field_ty {
            let inner_ty = &arr.elem;
            from_body.push(quote! {
                for (i, maybe_value) in from_value.#field_name.into_iter().enumerate() {
                    if let Some(value) = maybe_value {
                        item.set_value::<_, #offset>(value, local_offset + <#inner_ty>::SIZE * i);
                    }
                }
                #[cfg(test)]
                { println!("{}: {} offset: 0x{:0>4X}", stringify!(#name), stringify!(#field_name), local_offset); }
                local_offset += <#field_ty>::SIZE;
            })
        } else {
            from_body.push(quote! {
                if let Some(value) = from_value.#field_name {
                    item.set_value::<_, #offset>(value, local_offset);
                }
                #[cfg(test)]
                { println!("{}: {} offset: 0x{:0>4X}", stringify!(#name), stringify!(#field_name), local_offset); }
                local_offset += <#field_ty>::SIZE;
            })
        }
    }
    let res = quote! {
        #[automatically_derived]
        impl From<#name> for UpdateFields {
            fn from(from_value: #name) -> Self {
                #nested
                let mut local_offset = 0;
                #(
                    #from_body;
                )*
                item.set_value::<_, 0x0000>(#tag, 0x0002);
                item
            }
        }
    };
    res.into()
}
