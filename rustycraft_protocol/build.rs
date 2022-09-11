use std::collections::HashMap;
use std::env;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use prost_build::Service;
use prost_types::field_descriptor_proto::Type;
use protobuf::descriptor::{FieldDescriptorProto, MethodOptions, ServiceOptions};
use protobuf::UnknownFields;
use quick_protobuf::BytesReader;
use quote::{ToTokens, TokenStreamExt};

fn collect_protos(proto_dir: &str) -> Vec<String> {
    let mut protos = Vec::new();
    let dir = std::fs::read_dir(proto_dir).expect(&format!("Failed to read dir {}", &proto_dir));
    for entry in dir {
        let entry = entry.expect("Failed to get dir entry");
        let metadata = entry
            .metadata()
            .expect(&format!("Failed to get metadata of {:?}", entry.path()));
        if metadata.is_file() {
            if entry
                .file_name()
                .to_str()
                .expect("OsString decode error")
                .ends_with(".proto")
            {
                protos.push(
                    entry
                        .path()
                        .to_str()
                        .expect("OsString decode error")
                        .to_owned(),
                )
            }
        } else if metadata.is_dir() {
            protos.extend(collect_protos(
                entry.path().to_str().expect("OsString decode error"),
            ))
        }
    }
    protos
}

#[derive(Debug)]
enum Mod {
    Node(String, Vec<Mod>, TokenStream),
}

impl Mod {
    fn get_name(&self) -> &String {
        match self {
            Mod::Node(name, _, _) => name,
        }
    }
}

fn node_to_syn(node: &Mod, path: &str, out_dir: &str) -> TokenStream {
    match node {
        Mod::Node(name, vt, services) if vt.is_empty() => {
            let include_path = format!("{}/{}.{}.rs", out_dir, path, name);
            let nested_tokenstream: TokenStream = std::fs::read(&include_path)
                .expect(&format!("Failed to open {}", &include_path))
                .iter()
                .map(|c| *c as char)
                .collect::<String>()
                .parse()
                .expect(&format!("Failed to parse {}", &include_path));
            std::fs::remove_file(&include_path)
                .expect(&format!("Failed to remove {}", &include_path));
            let mod_name = quote::format_ident!("{}", name);
            quote::quote! {
                pub mod #mod_name {
                    #nested_tokenstream
                    #services
                }
            }
        }
        Mod::Node(name, nested, services) => {
            let mod_name = quote::format_ident!("{}", name);
            let nested_path = if path.eq("._") {
                name.clone()
            } else {
                format!("{}.{}", path, name)
            };

            let mut nested_ts = TokenStream::new();
            for node in nested {
                nested_ts.append_all(node_to_syn(node, &nested_path, out_dir))
            }
            let include_path = format!("{}/{}.{}.rs", out_dir, path, name).replace("._.", "");
            if let Ok(file_content) = std::fs::read(&include_path) {
                let file_tokens: TokenStream = file_content
                    .iter()
                    .map(|c| *c as char)
                    .collect::<String>()
                    .parse()
                    .expect(&format!("Failed to parse {}", &include_path));
                nested_ts.append_all(file_tokens);
                std::fs::remove_file(&include_path)
                    .expect(&format!("Failed to remove {}", &include_path));
            };
            nested_ts.append_all(services.to_token_stream());
            if path.is_empty() {
                nested_ts
            } else {
                quote::quote! {
                    pub mod #mod_name {
                        #nested_ts
                    }
                }
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum ExtensionTy {
    File,
    Message,
    Service,
    Field,
    Method,
}

impl ToString for ExtensionTy {
    fn to_string(&self) -> String {
        match self {
            ExtensionTy::File => "File",
            ExtensionTy::Message => "Message",
            ExtensionTy::Service => "Service",
            ExtensionTy::Field => "Field",
            ExtensionTy::Method => "Method",
        }
        .to_owned()
    }
}

trait OptionsWrapper {
    fn unknown_fields(&self) -> &UnknownFields;
    fn extension_ty() -> ExtensionTy;
}

struct ServiceWrapper {
    inner: Box<ServiceOptions>,
}

impl OptionsWrapper for ServiceWrapper {
    fn unknown_fields(&self) -> &UnknownFields {
        &self.inner.unknown_fields
    }

    fn extension_ty() -> ExtensionTy {
        ExtensionTy::Service
    }
}

struct MethodWrapper {
    inner: Box<MethodOptions>,
}

impl OptionsWrapper for MethodWrapper {
    fn unknown_fields(&self) -> &UnknownFields {
        &self.inner.unknown_fields
    }

    fn extension_ty() -> ExtensionTy {
        ExtensionTy::Method
    }
}

fn parse_extensions<T>(
    extensions: &HashMap<ExtensionTy, HashMap<i32, HashMap<u8, FieldDescriptorProto>>>,
    options: T,
) -> HashMap<String, ResWrapper>
where
    T: OptionsWrapper,
{
    let mut extension_fields = HashMap::new();
    let service_extensions = extensions.get(&T::extension_ty()).unwrap();
    for (id, value) in options.unknown_fields().iter() {
        let ext_handler = service_extensions
            .get(&(id as i32))
            .expect("Unexpected field.")
            .clone();
        let data = value.length_delimited.first().unwrap().as_slice();
        let mut bp = BytesReader::from_bytes(data);
        while !bp.is_eof() {
            let field = bp.next_tag(data).unwrap() / 8;
            let field_tp = ext_handler.get(&(field as u8)).expect("Unexpected field.");
            let tp: Type = Type::from_i32(field_tp.field_type() as i32).unwrap();
            let res = match tp {
                Type::Double => ResWrapper::Float(bp.read_double(data).unwrap() as f64),
                Type::Bool => ResWrapper::Bool(bp.read_bool(data).unwrap()),
                Type::String => {
                    let res_str = bp.read_string(data).unwrap();
                    ResWrapper::String(res_str.to_owned())
                }
                Type::Uint32 => ResWrapper::U32(bp.read_uint32(data).unwrap()),
                Type::Uint64 => ResWrapper::U64(bp.read_uint64(data).unwrap()),
                Type::Enum => ResWrapper::Usize(bp.read_uint32(data).unwrap() as usize),
                Type::Int32 => ResWrapper::Int(bp.read_int32(data).unwrap() as isize),
                Type::Int64 => ResWrapper::Int(bp.read_int64(data).unwrap() as isize),
                Type::Sint32 => ResWrapper::Int(bp.read_sint32(data).unwrap() as isize),
                Type::Sint64 => ResWrapper::Int(bp.read_sint64(data).unwrap() as isize),
                t => todo!("{} {:?}", field_tp.name(), t),
            };
            extension_fields.insert(field_tp.name().to_owned(), res);
        }
    }
    extension_fields
}

fn collect_extensions(
    package: &protobuf::descriptor::FileDescriptorProto,
) -> HashMap<ExtensionTy, HashMap<i32, HashMap<u8, FieldDescriptorProto>>> {
    let mut extensions: HashMap<ExtensionTy, HashMap<i32, HashMap<u8, FieldDescriptorProto>>> =
        HashMap::new();
    let mut extension_handlers = HashMap::new();
    for msg in &package.message_type {
        let handler_name = format!(".{}.{}", package.package(), &msg.name());
        let mut fields = HashMap::new();
        for field in msg.field.clone() {
            fields.insert(field.number() as u8, field);
        }
        extension_handlers.insert(handler_name, fields);
    }
    for ext in &package.extension {
        let ext_ty =
            ExtensionTy::try_from(ext.extendee().to_owned()).expect("Parse extensions failed");

        let ext_handler = extension_handlers
            .remove(ext.type_name())
            .expect("Unknown handler");
        if extensions.contains_key(&ext_ty) {
            let field_map = extensions.get_mut(&ext_ty).unwrap();
            field_map.insert(ext.number(), ext_handler.clone());
        } else {
            let mut field_map = HashMap::new();
            field_map.insert(ext.number(), ext_handler.clone());
            extensions.insert(ext_ty, field_map);
        }
        extension_handlers.insert(ext.type_name().to_owned(), ext_handler);
    }
    extensions
}

impl TryFrom<String> for ExtensionTy {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            ".google.protobuf.FieldOptions" => Self::Field,
            ".google.protobuf.MessageOptions" => Self::Message,
            ".google.protobuf.MethodOptions" => Self::Method,
            ".google.protobuf.ServiceOptions" => Self::Service,
            ".google.protobuf.FileOptions" => Self::File,
            _ => Err("Unknown extendee type.".to_owned())?,
        })
    }
}

struct ServiceGenerator {
    service_extensions: HashMap<
        String,
        (
            HashMap<String, ResWrapper>,
            HashMap<String, HashMap<String, ResWrapper>>,
        ),
    >,
}

fn match_wrapper(value: &ResWrapper) -> Option<(TokenStream, TokenStream)> {
    match value {
        ResWrapper::Float(val) => Some((quote::quote! { f64 }, quote::quote! { #val })),
        ResWrapper::Int(val) => Some((quote::quote! { isize }, quote::quote! { #val })),
        ResWrapper::U32(val) => Some((quote::quote! { u32 }, quote::quote! { #val })),
        ResWrapper::U64(val) => Some((quote::quote! { u64 }, quote::quote! { #val })),
        ResWrapper::Usize(val) => Some((quote::quote! { usize }, quote::quote! { #val })),
        ResWrapper::Bool(val) => Some((quote::quote! { bool }, quote::quote! { #val })),
        ResWrapper::String(val) => Some((quote::quote! { &'static str }, quote::quote! { #val })),
    }
}

impl prost_build::ServiceGenerator for ServiceGenerator {
    fn generate(&mut self, service: Service, buf: &mut String) {
        let (service_extensions, methods_extensions) = self
            .service_extensions
            .get(&service.name)
            .expect("Unexpected service");
        let mut fields = Vec::new();
        let mut types = Vec::new();
        let mut values = Vec::new();
        let original_hash = if let Some(ResWrapper::String(descriptor_name)) =
            service_extensions.get("descriptor_name")
        {
            service_name_hash(format!("{}", descriptor_name.clone()))
        } else {
            panic!("Unknown name hash")
        };
        let name_hash = service_name_hash(format!("{}.{}", &service.package, &service.proto_name));

        for (field, value) in service_extensions {
            fields.push(quote::format_ident!("{}", field.to_uppercase()));
            let (type_, val) = match_wrapper(&value).unwrap();
            types.push(type_);
            values.push(val);
        }
        let mut methods = Vec::new();
        let mut methods_consts = Vec::new();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut ids = Vec::new();

        let mut ret_rpc_types = Vec::new();
        let mut ret_rpc_names = Vec::new();
        let mut ret_rpc_rets = Vec::new();
        for method in service.methods {
            let extensions = methods_extensions.get(&method.proto_name).unwrap();
            let method_id = extensions.get("id").expect("Method id not found");
            let id_inner = if let ResWrapper::U32(id) = method_id {
                *id as u8
            } else {
                panic!("Bad type.")
            };
            ids.push(id_inner);
            methods.push(quote::format_ident!("{}", method.name));
            methods_consts.push(quote::format_ident!("{}", method.name.to_uppercase()));
            let input: TokenStream = method.input_type.parse().unwrap();
            let output: TokenStream = method.output_type.parse().unwrap();
            inputs.push(input.clone());
            outputs.push(output.clone());
            ret_rpc_rets.push(quote::format_ident!(
                "{}",
                method
                    .output_proto_type
                    .split(".")
                    .last()
                    .unwrap()
                    .to_case(Case::UpperCamel)
            ));
            if !ret_rpc_names.contains(&method.output_proto_type) {
                ret_rpc_names.push(method.output_proto_type.clone());
                ret_rpc_types.push(quote::format_ident!(
                    "{}",
                    method
                        .output_proto_type
                        .split(".")
                        .last()
                        .unwrap()
                        .to_case(Case::UpperCamel)
                ));
            };
        }
        let service_name = quote::format_ident!("{}", service.name);

        let quoted = quote::quote! {
            #[async_trait::async_trait]
            pub trait #service_name: crate::messages::LoggingAttributes {
                const NAME_HASH: u32 = #name_hash;
                const ORIGINAL_HASH: u32 = #original_hash;
                # ( const #fields: #types = #values; ) *
                # ( const #methods_consts: u8 = #ids; ) *
                # ( async fn #methods(&mut self, _: #inputs) -> Result<#outputs, crate::rpc_responses::WowRpcResponse> {Err(crate::rpc_responses::WowRpcResponse::NotImplemented)} ) *

                async fn dispatch(&mut self, msg: crate::messages::RawMessage) -> Result<bytes::Bytes, crate::rpc_responses::WowRpcResponse> {
                    use prost::Message;
                    let method_id = msg.headers.method_id.ok_or_else(|| crate::rpc_responses::WowRpcResponse::RpcMalformedRequest )? as u8;
                    match method_id {
                      # ( #ids => {
                            let parsed = if let Some(0) = msg.headers.size
                                {<#inputs>::default()}
                                else
                                {<#inputs>::decode(&mut msg.data.clone())?};
                            log::debug!(target: stringify!(#service_name), "[{:?}] Method `{}` called with data: {:?}", self.get_client_addr(), stringify!(#methods), &parsed);
                            let response = self.#methods(parsed).await;
                            let mut headers = crate::bgs::protocol::Header::default();
                            headers.method_id = Some(#ids as u32);
                            headers.service_hash = Some(Self::ORIGINAL_HASH);
                            headers.token = msg.headers.token;
                            log::debug!(target: stringify!(#service_name), "[{:?}] Method `{}` response: {:?}", self.get_client_addr(), stringify!(#methods), &response);
                            let mut outoing_message = crate::messages::OutgoingMessage{ headers, message: Some(response?) };
                            Ok(outoing_message.encode(true))
                        } ) *
                        _ => Err( crate::rpc_responses::WowRpcResponse::RpcNotImplemented ),
                    }
                }
            }
        };
        buf.push_str(&quoted.to_string())
    }
}

fn build_module(proto_dir: &str, out_filename: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let protos = collect_protos(proto_dir);
    let mut extensions = HashMap::new();

    let proto_files = protobuf_parse::Parser::new()
        .include(proto_dir)
        .inputs(&protos)
        .parse_and_typecheck()
        .expect("Protobuf typechecking failed.");
    let mut has_services = Vec::new();

    for package in &proto_files.file_descriptors {
        extensions.extend(collect_extensions(&package));
        if !package.service.is_empty() {
            has_services.push(package);
        };
    }

    let mut service_extensions = HashMap::new();

    for package in has_services {
        for service in &package.service {
            let mut extension_fields = HashMap::new();
            if let Some(ref options) = service.options.0 {
                extension_fields = parse_extensions(
                    &extensions,
                    ServiceWrapper {
                        inner: options.clone(),
                    },
                )
            }
            let mut method_extension_fields = HashMap::new();
            for method in &service.method {
                let mut extension_fields = HashMap::new();
                if let Some(ref options) = method.options.0 {
                    extension_fields = parse_extensions(
                        &extensions,
                        MethodWrapper {
                            inner: options.clone(),
                        },
                    )
                }
                method_extension_fields.insert(method.name().to_owned(), extension_fields);
            }
            service_extensions.insert(
                service.name().to_owned(),
                (extension_fields, method_extension_fields),
            );
        }
    }

    let mut module = Mod::Node("_".to_owned(), Vec::new(), TokenStream::default());
    for package in &proto_files.file_descriptors {
        let mut last_node = &mut module;
        for submod in package.package().split(".") {
            let submod = if submod.eq("v1") {
                submod.to_owned()
            } else {
                submod.to_case(Case::Snake)
            };
            match last_node {
                Mod::Node(_, ref mut nested, _) => {
                    let nested_idx = nested.iter().position(|x| x.get_name().eq(&submod));
                    if nested_idx.is_none() {
                        let new_node = Mod::Node(submod, Vec::new(), TokenStream::default());
                        nested.push(new_node);
                        last_node = nested.last_mut().unwrap();
                    } else {
                        last_node = nested.get_mut(nested_idx.unwrap()).unwrap();
                    }
                }
            }
        }
    }

    prost_build::Config::new()
        .out_dir(&out_dir)
        .type_attribute(".", " #[derive(serde::Serialize, serde::Deserialize)]")
        .service_generator(Box::new(ServiceGenerator { service_extensions }))
        .compile_protos(protos.as_slice(), &[proto_dir])
        .expect("Failed to build by prost");

    let nested_mods_ts = node_to_syn(&module, "", &out_dir);
    let quoted_module = quote::quote! {
        #[allow(dead_code)]
        #[allow(warnings)]
        #nested_mods_ts
    };
    std::fs::write(
        format!("{}/{}", out_dir, out_filename),
        quoted_module.to_string().as_bytes(),
    )
    .expect("Failed to write result");
    std::process::Command::new("rustfmt")
        .args(["--edition=2018", &format!("{}/{}", out_dir, out_filename)])
        .output()
        .expect("Failed to run rustfmt");
}

#[derive(Debug)]
enum ResWrapper {
    Float(f64),
    Int(isize),
    U32(u32),
    U64(u64),
    Usize(usize),
    Bool(bool),
    String(String),
}

impl ToTokens for ResWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            ResWrapper::Float(_) => quote::quote! {f64},
            ResWrapper::Int(_) => quote::quote! {isize},
            ResWrapper::U32(_) => quote::quote! {u32},
            ResWrapper::U64(_) => quote::quote! {u64},
            ResWrapper::Usize(_) => quote::quote! {usize},
            ResWrapper::Bool(_) => quote::quote! {bool},
            ResWrapper::String(_) => quote::quote! {std::string::String},
        })
    }
}

fn service_name_hash(name: String) -> u32 {
    let mut hash = 0x811C9DC5;
    for c in name.chars() {
        hash ^= c as u32;
        hash = hash.wrapping_mul(0x1000193);
    }
    hash
}

fn main() {
    build_module("./protoc", "autogen.rs");
}
