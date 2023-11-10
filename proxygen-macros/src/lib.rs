use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn};

const GET_ARG_TYPES: fn(&FnArg) -> syn::Ident = |arg: &FnArg| match arg {
    FnArg::Receiver(_) => panic!("Cannot use receivers (self) with proxy functions"),
    FnArg::Typed(arg) => {
        if let syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) = arg.ty.as_ref()
        {
            return segments.first().unwrap().ident.clone();
        }
        panic!("Unsupported function signature");
    }
};

const GET_ARG_NAMES: fn(&FnArg) -> syn::Ident = |arg: &FnArg| match arg {
    FnArg::Receiver(_) => panic!("Cannot use receivers (self) with proxy functions"),
    FnArg::Typed(arg) => {
        let syn::PatType { pat, .. } = &arg;
        let pat = pat.clone();
        match *pat {
            syn::Pat::Ident(ident) => ident.ident,
            _ => panic!("Unexpected arg name: {:?}", pat),
        }
    }
};

/// Makes the original function available as `orig_func` with the same args and return type as the interceptor
#[proc_macro_attribute]
pub fn proxy(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = syn::parse(item).expect("You may only proxy a function");
    let func_name = input.sig.ident.to_string();
    let func_sig = input.sig.clone();
    let func_body = input.block.stmts.clone();
    let ret_type = input.sig.output.clone();
    let orig_index_ident = syn::parse_str::<syn::Ident>(&format!("Index_{}", &func_name)).unwrap();
    let arg_types = input.sig.inputs.iter().map(GET_ARG_TYPES);
    let attrs = input.attrs.into_iter().filter(|attr|!attr.path().is_ident("proxy"));
    TokenStream::from(quote!(
        #(#attrs)*
        #func_sig {
            let orig_func: fn (#(#arg_types,)*) #ret_type = unsafe { std::mem::transmute(ORIGINAL_FUNCS[#orig_index_ident]) };
            #(#func_body)*
        }
    ))
}

/// Makes the original function available as `orig_func` with the same args and return type as the interceptor function
///
/// Additionally, any code in this function will be run just before the original function is called.
///
/// Note: Returning in this function will skip running the original.
#[proc_macro_attribute]
pub fn pre_hook(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let proxied = proxy(attrs, item);
    let input: ItemFn = syn::parse(proxied).expect("You may only proxy a function");
    let func_sig = input.sig.clone();
    let func_body = input.block.stmts.clone();
    let arg_names = input.sig.inputs.iter().map(GET_ARG_NAMES);
    let attrs = input.attrs.into_iter().filter(|attr|!attr.path().is_ident("pre_hook"));
    TokenStream::from(quote!(
        #(#attrs)*
        #func_sig {
            #(#func_body)*
            orig_func(#(#arg_names,)*)
        }
    ))
}

/// Makes the original function available as `orig_func` with the same args and return type as the interceptor function
///
/// Additionally, any code in this function will be run after the orginal function is called.
///
/// The result of calling the original function will be accessible in `orig_result`.
///
/// Note: `orig_result` will be returned unless you choose to return your own result from this function.
#[proc_macro_attribute]
pub fn post_hook(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = syn::parse(item).expect("You may only proxy a function");
    let func_name = input.sig.ident.to_string();
    let func_sig = input.sig.clone();
    let func_body = input.block.stmts.clone();
    let ret_type = input.sig.output.clone();
    let orig_index_ident = syn::parse_str::<syn::Ident>(&format!("Index_{}", &func_name)).unwrap();
    let arg_names = input.sig.inputs.iter().map(GET_ARG_NAMES);
    let arg_types = input.sig.inputs.iter().map(GET_ARG_TYPES);
    let attrs = input.attrs.into_iter().filter(|attr|!attr.path().is_ident("post_hook"));

    TokenStream::from(quote!(
        #(#attrs)*
        #func_sig {
            let orig_func: fn (#(#arg_types,)*) #ret_type = unsafe { std::mem::transmute(ORIGINAL_FUNCS[#orig_index_ident]) };
            let orig_result = orig_func(#(#arg_names,)*);
            #(#func_body)*
            orig_result
        }
    ))
}
