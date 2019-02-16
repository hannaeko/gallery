extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, DataStruct, Meta, Data, Field, NestedMeta, Ident};

const ATTR_EXIF_METADATA_NAME: &str = "exif";

#[proc_macro_derive(ExifExtractor, attributes(exif))]
pub fn exif_extractor_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_plouf(&ast)
}

fn impl_plouf(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (fields, tags, _names) = match ast.data {
        Data::Struct(ref strukt) => find_tags(&strukt),
        _ => panic!("ExifExtractor should be derived on struct")
    };

    let tags2 = tags.clone();

    let gen = quote! {
        impl ExifExtractor for #name {
            const TAG_LIST: &'static [exif::Tag] = &[
                #( exif::Tag::#tags, )*
            ];

            fn extract_exif(&mut self, path: &std::path::PathBuf) -> std::io::Result<()> {
                let mut exif_map = Self::extract_exif_map(path)?;
                #( self.#fields = exif_map.remove(&exif::Tag::#tags2); )*
                Ok(())
            }
        }
    };
    gen.into()
}

fn find_tags(strukt: &DataStruct) -> (Vec<Ident>, Vec<Ident>, Vec<String>) {
    let iter = strukt.fields
        .iter()
        .filter_map(build_tag_field);
    let mut fields = Vec::new();
    let mut tags = Vec::new();
    let mut names = Vec::new();

    for (field, tag, name) in iter {
        fields.push(field);
        tags.push(tag);
        names.push(name)
    }

    (fields, tags, names)
}

fn build_tag_field(field: &Field) -> Option<(Ident, Ident, String)> {
    let mut tag = None;
    let mut name = None;

    for meta_items in field.attrs.iter().filter_map(get_meta_item) {
        for meta_item in meta_items {
            if let NestedMeta::Meta(Meta::NameValue(ref v)) = meta_item {
                if v.ident == "tag" {
                    tag = get_litteral(&v.lit);
                } else if v.ident == "name" {
                    name = get_litteral(&v.lit);
                }
            }
        }
    }

    if tag.is_some() && name.is_some() {
        Some((
            field.ident.as_ref().unwrap().to_owned(),
            Ident::new(tag.unwrap().as_str(), Span::call_site()),
            name.unwrap()
        ))
    } else {
        None
    }
}

fn get_meta_item(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == ATTR_EXIF_METADATA_NAME {
        match attr.parse_meta() {
            Ok(syn::Meta::List(ref meta_list)) => Some(meta_list.nested.iter().cloned().collect()),
            _ => None
         }
    } else {
        None
    }
}

fn get_litteral(lit: &syn::Lit) -> Option<String> {
    if let syn::Lit::Str(lit_str) = lit {
        Some(lit_str.value())
    } else {
        None
    }
}
