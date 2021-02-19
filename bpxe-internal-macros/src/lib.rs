use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_str, Block, ItemFn};

/// Picks a correct test harness for the target
#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let test_name = input.sig.ident.to_string();

    input.block.stmts.insert(
        0,
        parse_str(&format!(
            r#"
                {{
                  #[cfg(target_arch = "wasm32")]
                  console_error_panic_hook::set_once();

                  #[cfg(target_arch = "wasm32")]
                  web_sys::console::debug_1(&format!("Running {{}}::{0}", module_path!()).into());
                  #[cfg(not(target_arch = "wasm32"))]
                  eprintln!("Running {{}}::{0}", module_path!());
                }}
                "#,
            test_name
        ))
        .unwrap(),
    );

    let expanded = if input.sig.asyncness.is_some() {
        let mut wasm_input = input.clone();
        wasm_input.sig.asyncness = None;

        let stmts = wasm_input.block.stmts;

        let wrapper: TokenStream = quote! {
            {
               let result = wasm_rs_async_executor::single_threaded::run(Some(wasm_rs_async_executor::single_threaded::spawn(async { #(#stmts)* }).task()));
               wasm_rs_async_executor::single_threaded::evict_all();
               result
            }
        }
        .into();
        wasm_input.block = Box::new(parse_macro_input!(wrapper as Block));

        quote! {
            #[cfg(not(any(target_arch = "wasm32", feature = "wasm-executor")))]
            #[tokio::test]
            #input

            #[cfg(any(feature = "wasm-executor", target_arch = "wasm32"))]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
            #[cfg_attr(all(feature = "wasm-executor", not(target_arch = "wasm32")), test)]
            #wasm_input
        }
    } else {
        quote! {
            #[cfg_attr(not(target_arch = "wasm32"), test)]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
            #input
        }
    };

    TokenStream::from(expanded)
}
