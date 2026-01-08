// Once we created the isolated crate for the macros. We defined as a procedural
// macro crate adding `proc-macro=true` in `Cargo.toml` along with some dependencies.
//
// Now we can create the macro definition.
use proc_macro::TokenStream;
// NOTE: The quote crate makes easier turn `syn` data structures into Rust code
use quote::quote;

// For this example, we split the hello_derive_macro to parse the TokenStream
// and then delegates the task of transforming the tree to impl_hello_macro.
// This way, the macro is convenient and easily extensable.
//
// Also, it's a convention to name the trait and the macro which implement it the
// same.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate. We use unwrap because it's important procedural
    // macros panics on errors which is a TokenStream.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// When parsing the TokenStream it returns a `DeriveInput` struct which represents
// the parsed Rust code. For the struct `struct Pancakes;` we have the result like
// ```
// DeriveInput {
//    // --snip--
//
//    ident: Ident {
//        ident: "Pancakes",
//        span: #0 bytes(95..103)
//    },
//    data: Struct(
//        DataStruct {
//            struct_token: Struct,
//            fields: Unit,
//            semi_token: Some(
//                Semi
//            )
//        }
//    )
//}
//```

// This way, we create the trait implementation function
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // This macro allows us to define the Rust code we want to return.
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // An converto to TokenStream by calling into method.
    generated.into()
}
