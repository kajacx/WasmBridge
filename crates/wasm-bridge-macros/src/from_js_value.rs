use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

pub fn from_js_value_struct(name: Ident, data: DataStruct) -> TokenStream {
    let mut impl_block = TokenStream::new();
    let mut fields_constructor = TokenStream::new();

    for field in data.fields {
        let field_type = field.ty;
        let field_name = field.ident;

        let field_name_str = quote!(#field_name).to_string();
        let field_name_converted = field_name_str.to_lower_camel_case();

        let tokens = quote!(
            let js_field = wasm_bridge::js_sys::Reflect::get(value, &#field_name_converted.into())
                .map_err(wasm_bridge::helpers::map_js_error("Get struct field"))?;
            let #field_name = <#field_type>::from_js_value(&js_field)?;
        );
        impl_block.append_all(tokens);

        fields_constructor.append_all(quote!(#field_name, ));
    }

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                #impl_block

                Ok(Self { #fields_constructor })
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}

pub fn from_js_value_enum(name: Ident, data: DataEnum) -> TokenStream {
    let mut impl_block = TokenStream::new();

    for variant in data.variants {
        let variant_name = variant.ident;
        let variant_name_str = quote!(#variant_name).to_string();
        let variant_name_converted = variant_name_str.to_kebab_case();

        let tokens = quote!(
            if tag == #variant_name_converted {
                return Ok(Self::#variant_name);
            };
        );

        impl_block.append_all(tokens);
    }

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                let tag = value
                    .as_string()
                    .ok_or(value)
                    .map_err(wasm_bridge::helpers::map_js_error("Enum should be a string"))?;

                #impl_block

                Err(wasm_bridge::helpers::map_js_error("Unknown enum tag")(value))
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}

pub fn from_js_value_variant(name: Ident, data: DataEnum) -> TokenStream {
    let mut impl_block = TokenStream::new();

    for variant in data.variants {
        let variant_name = variant.ident;
        let variant_name_str = quote!(#variant_name).to_string();
        let variant_name_converted = variant_name_str.to_kebab_case();

        let field = variant.fields.iter().next();

        let return_value = match field {
            Some(field) => {
                let field_type = &field.ty;
                quote!( Self::#variant_name(<#field_type>::from_js_value(&val)?) )
            }
            None => quote!( Self::#variant_name ),
        };

        let tokens = quote!(
            if tag == #variant_name_converted {
                return Ok(#return_value);
            };
        );
        impl_block.append_all(tokens);
    }

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                let tag_str = wasm_bridge::helpers::static_str_to_js("tag");
                let tag = wasm_bridge::js_sys::Reflect::get(value, tag_str)
                    .map_err(wasm_bridge::helpers::map_js_error("Get variant tag"))?
                    .as_string()
                    .ok_or(value)
                    .map_err(wasm_bridge::helpers::map_js_error("Variant tag should be a string"))?;

                let val_str = wasm_bridge::helpers::static_str_to_js("val");
                let val = wasm_bridge::js_sys::Reflect::get(value, val_str)
                    .map_err(wasm_bridge::helpers::map_js_error("Get variant val"))?;

                #impl_block

                Err(wasm_bridge::helpers::map_js_error("Unknown variant tag")(value))
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}
