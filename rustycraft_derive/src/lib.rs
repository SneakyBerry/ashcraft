use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Ident, ItemStruct, Token, Pat};

struct DeriveIntoUpdateFieldsArgs {
    offset: Expr,
    tag: Expr,
}

impl Parse for DeriveIntoUpdateFieldsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut offset = None;
        let mut tag = None;
        for _ in 0..2 {
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            if format_ident!("offset") == ident {
                offset = Some(input.parse()?);
            } else if format_ident!("tag") == ident {
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

struct DeriveServerPacket {
    opcode: Pat,
}

impl Parse for DeriveServerPacket {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let opcode = input.parse()?;
        Ok(DeriveServerPacket { opcode })
    }
}

#[proc_macro_derive(ServerPacket, attributes(opcode))]
pub fn derive_server_packet(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let name = &item.ident;
    let mut args = None;
    for attr in &item.attrs {
        match attr.parse_args::<DeriveServerPacket>() {
            Ok(meta) => {
                args = Some(meta);
            }
            Err(err) => return err.into_compile_error().into(),
        }
    }
    let Some( DeriveServerPacket { opcode}) = args else {
        return syn::Error::new(item.span(),"Arguments are not present").to_compile_error().into();
    };
    let res = quote! {
        #[automatically_derived]
        impl crate::ServerPacket for #name {
            fn get_opcode(&self) -> Opcode {
                #opcode
            }
        }
    };
    res.into()
}

#[proc_macro_derive(CalcUpdate, attributes(nested, meta))]
pub fn derive_calc_update(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let name = &item.ident;
    let mut body = vec![];
    let mut args = None;
    for attr in &item.attrs {
        match attr.parse_args::<DeriveIntoUpdateFieldsArgs>() {
            Ok(meta) => {
                args = Some(meta);
            }
            Err(err) => return err.into_compile_error().into(),
        }
    }
    let Some(DeriveIntoUpdateFieldsArgs { offset, tag }) = args else {
        return syn::Error::new(item.span(),"Arguments are not present").to_compile_error().into();
    };
    let mut field_offsets = vec![];
    'o: for field in item.fields {
        let field_name = field.ident.expect("Tuple struct not supported.");
        let field_ty = field.ty;
        for attr in field.attrs {
            if let Ok(meta) = attr.parse_meta() {
                if meta.path().is_ident(&format_ident!("nested")) {
                    body.push(quote! {
                        item.update(self.#field_name.get_diff(old.map(|o| &o.#field_name)));
                    });
                    continue 'o;
                }
            }
        }
        body.push(quote! {
            #[cfg(test)]
            { println!("{}: {} offset: 0x{:0>4X}", stringify!(#name), stringify!(#field_name), #offset # (+ #field_offsets) * ); }
            item.update(
                <#field_ty as crate::objects::calc_update::CalcUpdate<{#offset # (+ #field_offsets) *}>>::get_diff(&self.#field_name, old.map(|o| &o.#field_name))
            );
        });
        field_offsets.push(quote! {<#field_ty>::SIZE});
    }
    let res = quote! {
        #[automatically_derived]
        impl crate::objects::calc_update::CalcUpdate<#offset> for #name {
            fn get_diff(&self, old: Option<&Self>) -> crate::objects::UpdateFields {
                use crate::objects::size_helper::FieldSize;
                use deku::DekuWrite;

                let mut item = crate::objects::UpdateFields::new();
                # (
                    #body;
                ) *
                let mut tag_buf = deku::bitvec::BitVec::new();
                (#tag as u32).write(&mut tag_buf, ()).expect("Write failed");
                item.set_value::<0x0000>(tag_buf.as_raw_slice(), 0x0002);
                item
            }
        }
    };
    res.into()
}
