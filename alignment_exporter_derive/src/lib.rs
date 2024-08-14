use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Any struct that uses this procedural macro is automatically annotated with `#[repr(C)]`. Learn more about the significance of this annotation and why it is required [here](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc).
#[proc_macro_attribute]
pub fn export_alignment(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let data = match &input.data {
        Data::Struct(data) => data,
        _ => {
            return TokenStream::from(quote! {
                compile_error!("export_alignment can only be used on structs");
            });
        }
    };
    let mut code = Vec::new();
    if let syn::Fields::Named(ref fields) = data.fields {
        for field in &fields.named {
            let ty = &field.ty;
            code.push(quote! {{
                let size = std::mem::size_of::<#ty>();
                let alignment = std::mem::align_of::<#ty>();
                vec.push(Alignment {
                    size,
                    offset,
                    ty_name: stringify!(#ty)
                });
                if alignment > max_alignment {
                    max_alignment = alignment;
                }
                if offset % alignment != 0 {
                    let padding = alignment - (offset % alignment);
                    offset += padding;
                }
                let field_offset = offset;
                offset += size;
            }});
        }
    }
    let output = quote! {
        #[repr(C)]
        #input

        use ::std::sync::LazyLock;
        use ::std::vec::Vec;
        use ::alignment_exporter::{Alignment, AlignmentExporter};

        impl AlignmentExporter for #struct_name {
            fn get_alignment() -> &'static [Alignment] {
                static RESULT: LazyLock<Vec<Alignment>> = LazyLock::new(|| {
                    let mut vec = Vec::new();
                    let mut offset = 0;
                    let mut max_alignment = 0;
                    #(#code)*

                    vec
                });
                RESULT.as_slice()
            }
        }
    };
    output.into()
}
