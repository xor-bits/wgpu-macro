use darling::{
    ast::{Data, Fields},
    FromDeriveInput, FromField,
};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, Generics, Type};

//

#[derive(FromDeriveInput)]
#[darling(attributes(layout), supports(struct_any))]
struct DeriveVertexLayout {
    ident: Ident,
    generics: Generics,

    data: Data<(), VertexAttribute>,
}

#[derive(FromField)]
#[darling(attributes(layout))]
struct VertexAttribute {
    // ident: Option<Ident>,
    ty: Type,
    // #[darling(default)]
    // location: Option<u32>,
}

#[proc_macro_derive(VertexLayout)]
pub fn derive_vertex_layout(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input as DeriveInput);
    let input: DeriveVertexLayout = match DeriveVertexLayout::from_derive_input(&input) {
        Ok(v) => v,
        Err(err) => {
            return syn::Error::new(Span::call_site(), err)
                .to_compile_error()
                .into();
        }
    };

    let DeriveVertexLayout {
        ident,
        generics,
        data,
    } = input;

    let (imp, gen, wher) = generics.split_for_impl();

    let fields: Fields<_> = data.take_struct().unwrap();
    let mut types = vec![];
    let mut i: u32 = 0;

    let attribs = fields.fields.iter().fold(quote! {}, |acc, field| {
        let ty = &field.ty;

        let offset = types.iter().fold(quote! { 0u64 }, |acc, next| {
            quote! {
                #acc
                + std::mem::size_of::<#next>() as u64
            }
        });
        types.push(ty);

        let shader_location = i;
        i += 1;

        quote! {
            #acc
            wgpu::VertexAttribute {
                format: <#ty as VertexAttribute>::FORMAT,
                offset: #offset,
                shader_location: #shader_location
            },
        }
    });

    (quote! {
        impl #imp VertexLayout for #ident #gen #wher {
            const ATTRIBUTES: &'static [wgpu::VertexAttribute] = &[
                #attribs
            ];
        }
    })
    .into()
}
