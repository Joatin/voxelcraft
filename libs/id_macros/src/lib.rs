use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn mod_id(input: TokenStream) -> TokenStream {
    let num = get_uuid_as_num(input);

    (quote::quote! {
        &voxelcraft_id::ModId::from_u128(#num)
    })
    .into()
}

#[proc_macro]
pub fn block_id(input: TokenStream) -> TokenStream {
    let num = get_uuid_as_num(input);

    (quote::quote! {
        &voxelcraft_id::BlockId::from_u128(#num)
    })
    .into()
}

#[proc_macro]
pub fn face_id(input: TokenStream) -> TokenStream {
    let num = get_uuid_as_num(input);

    (quote::quote! {
        &voxelcraft_id::FaceId::from_u128(#num)
    })
    .into()
}

#[proc_macro]
pub fn dimension_id(input: TokenStream) -> TokenStream {
    let num = get_uuid_as_num(input);

    (quote::quote! {
        &voxelcraft_id::DimensionId::from_u128(#num)
    })
    .into()
}

fn get_uuid_as_num(input: TokenStream) -> u128 {
    let tokens: Vec<_> = input.into_iter().collect();

    let raw_uuid = match tokens.as_slice() {
        [TokenTree::Literal(lit)] => unwrap_string_literal(lit),
        _ => panic!("This macro only accepts a single, non-empty string argument"),
    };

    let uuid = uuid::Uuid::parse_str(&raw_uuid).unwrap();

    uuid.as_u128()
}

fn unwrap_string_literal(lit: &proc_macro::Literal) -> String {
    let mut repr = lit.to_string();
    if !repr.starts_with('"') || !repr.ends_with('"') {
        panic!("This macro only accepts a single, non-empty string argument")
    }

    repr.remove(0);
    repr.pop();

    repr
}
