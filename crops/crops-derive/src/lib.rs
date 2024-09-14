use std::collections::HashMap;

use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Field, Fields, GenericArgument,
    Ident, Meta, PathArguments, PathSegment, Type, Variant, Visibility,
};

const COPYABLE: [&str; 13] = [
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64", "bool",
];

#[proc_macro_attribute]
pub fn c_result_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let function_name = input.sig.ident.clone();

    let mut inner = input.clone();
    let inner_ident = Ident::new(&format!("_inner_{function_name}"), function_name.span());
    inner.sig.ident = inner_ident.clone();
    inner.sig.abi.take();
    inner.vis = Visibility::Inherited;

    let mut new_signature = input.sig.clone();
    new_signature.abi = parse_quote!(extern "C");
    new_signature.output = parse_quote!(-> i32);

    let inputs = inner.sig.inputs.iter().map(|input| match input {
        syn::FnArg::Receiver(_) => panic!("Self argument is not supported in ABI"),
        syn::FnArg::Typed(typed) => {
            let pat = typed.pat.as_ref();
            pat
        }
    });

    let args = input.attrs;
    quote::quote!(
        #inner

        #(#args)*
        #[no_mangle]
        pub #new_signature {
            match #inner_ident(#(#inputs),*) {
                Ok(_) => 0,
                Err(e) => {
                    eprintln!("{:?}", e);
                    1
                }
            }
        }
    )
    .into()
}

#[proc_macro_derive(CBuilder, attributes(c_builder))]
pub fn derive_d_builder(item: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(item as DeriveInput);

    match data {
        Data::Enum(e) => derive_c_builder_enum(ident, attrs, e),
        Data::Struct(s) => derive_c_builder_struct(ident, attrs, s),
        Data::Union(_) => panic!("Not supported for unions"),
    }
}

#[derive(Default)]
struct CBuilderFieldArgs {
    c_enum: bool,
    c_as: Option<syn::Type>,
    c_input: Option<syn::Expr>,
    c_parser: Option<proc_macro2::TokenStream>,
}

impl Parse for CBuilderFieldArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut res = Self::default();
        let result = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;

        for expr in result {
            if let syn::Expr::Path(p) = expr {
                if p.path.is_ident("c_enum") {
                    res.c_enum = true;
                }
            } else if let syn::Expr::Assign(assign) = expr {
                if let syn::Expr::Path(p) = assign.left.as_ref() {
                    if p.path.is_ident("c_as") {
                        res.c_as
                            .replace(syn::parse(assign.right.to_token_stream().into())?);
                    } else if p.path.is_ident("c_input") {
                        res.c_input.replace(assign.right.as_ref().clone());
                    } else if p.path.is_ident("c_parser") {
                        res.c_parser.replace(assign.right.to_token_stream());
                    }
                }
            }
        }

        Ok(res)
    }
}

#[derive(Debug)]
struct CBuilderArgs {
    c_new: bool,
    c_clone: bool,
    c_debug: bool,
    // TODO(bjc) permit extra constructors
    c_constructors: Vec<Vec<syn::Ident>>,
}

impl Default for CBuilderArgs {
    fn default() -> Self {
        Self {
            c_new: true,
            c_clone: true,
            c_debug: true,
            c_constructors: Default::default(),
        }
    }
}

impl CBuilderArgs {
    fn extract(&mut self, mut attrs: Vec<Attribute>) -> Vec<Attribute> {
        attrs.retain(|attr| {
            if let Meta::List(list) = &attr.meta {
                if list.path.is_ident("c_builder") {
                    let line = syn::parse::<CBuilderArgs>(list.tokens.clone().into()).unwrap();
                    self.c_clone = line.c_clone;
                    self.c_new = line.c_new;
                    self.c_debug = line.c_debug;
                    self.c_constructors.extend(line.c_constructors);
                    return false;
                }
            }
            false
        });

        attrs
    }
}

macro_rules! get_literal {
    ($e:expr, $t:ident) => {
        if let syn::Expr::Lit(lit) = $e {
            if let syn::Lit::$t(value) = &lit.lit {
                value.value()
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    };
}

impl Parse for CBuilderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut res = Self::default();
        let result = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;

        for expr in result {
            if let syn::Expr::Assign(assign) = expr {
                if let syn::Expr::Path(p) = assign.left.as_ref() {
                    if p.path.is_ident("new") {
                        res.c_new = get_literal!(assign.right.as_ref(), Bool);
                    } else if p.path.is_ident("clone") {
                        res.c_clone = get_literal!(assign.right.as_ref(), Bool);
                    } else if p.path.is_ident("debug") {
                        res.c_debug = get_literal!(assign.right.as_ref(), Bool);
                    } else if p.path.is_ident("constructor") {
                        let constructor = match assign.right.as_ref() {
                            Expr::Tuple(tuple) => {
                                tuple.elems.iter().map(parse_ident).cloned().collect_vec()
                            }
                            _ => panic!("Argument to c_construtor must be a tuple"),
                        };

                        res.c_constructors.push(constructor);
                    }
                }
            }
        }

        Ok(res)
    }
}

fn parse_angle_bracket(segment: &PathSegment) -> Option<&Type> {
    if let PathArguments::AngleBracketed(angle) = &segment.arguments {
        if let Some(GenericArgument::Type(t)) = angle.args.first() {
            return Some(t);
        }
    }
    None
}

fn parse_ident(expr: &Expr) -> &Ident {
    match expr {
        Expr::Path(p) => p.path.get_ident().unwrap(),
        _ => panic!("Invalid Expression"),
    }
}

fn filter_args(attrs: &[Attribute]) -> (CBuilderFieldArgs, Vec<&Attribute>) {
    attrs.iter().fold(
        (CBuilderFieldArgs::default(), vec![]),
        |(mut mine, mut others), attr| {
            match &attr.meta {
                Meta::List(list) => {
                    if list.path.is_ident("c_builder") {
                        let line =
                            syn::parse::<CBuilderFieldArgs>(list.tokens.clone().into()).unwrap();
                        mine.c_enum = line.c_enum;
                        mine.c_as = line.c_as;
                        mine.c_input = line.c_input;
                        mine.c_parser = line.c_parser;
                    } else {
                        others.push(attr)
                    }
                }
                _ => {
                    if !attr.path().is_ident("default") {
                        others.push(attr)
                    }
                }
            };
            (mine, others)
        },
    )
}

fn consider_copyable(args: &CBuilderFieldArgs, ty: &Type) -> CTypes {
    if let Type::Path(path) = ty {
        if let Some(ident) = path.path.get_ident() {
            if COPYABLE.contains(&ident.to_string().as_str()) || args.c_enum {
                return CTypes {
                    rust: ty.clone(),
                    from_c: parse_quote!(#ty),
                    from_c_parser: Box::new(|ident| parse_quote!(#ident)),
                    to_c: parse_quote!(*mut #ty),
                    to_c_parser: if args.c_enum {
                        Box::new(
                            |c_value, value| parse_quote!(*::crops::utils::check_null(#c_value)? = #value.clone()),
                        )
                    } else {
                        Box::new(
                            |c_value, value| parse_quote!(*::crops::utils::check_null(#c_value)? = *#value),
                        )
                    },
                };
            }
        }
    }

    CTypes {
        rust: ty.clone(),
        from_c: parse_quote!(&#ty),
        from_c_parser: Box::new(|ident| parse_quote!(#ident.clone())),
        to_c: parse_quote!(*mut #ty),
        to_c_parser: Box::new(
            |c_value, value| parse_quote!(*::crops::utils::check_null(#c_value)? = #value.clone()),
        ),
    }
}

struct CTypes {
    rust: Type,
    from_c: Type,
    from_c_parser: Box<dyn Fn(&Ident) -> TokenStream2>,
    to_c: Type,
    to_c_parser: Box<dyn Fn(&Ident, &Ident) -> TokenStream2>,
}

fn get_wrapper_ty_ident(ty: &Type) -> &Ident {
    if let Type::Path(path) = ty {
        return &path.path.segments.last().unwrap().ident;
    }
    panic!()
}

fn gen_c_types_inner(args: &CBuilderFieldArgs, ty: &Type) -> CTypes {
    if let Type::Path(path) = &ty {
        let last = path.path.segments.last().unwrap();
        let ty_outer = last.ident.to_string();

        match ty_outer.as_str() {
            "String" => CTypes {
                rust: ty.clone(),
                from_c: parse_quote!(&::crops::_macros::libc::c_char),
                from_c_parser: Box::new(|ident| parse_quote!(= ::crops::utils::as_string(#ident)?)),
                to_c: parse_quote!(*mut ::crops::_macros::libc::c_char),
                to_c_parser: Box::new(
                    |c_value, value| parse_quote!(::crops::utils::copy_string(#c_value, #value)?),
                ),
            },
            // If we wanted to do other special cases
            // that would go here
            e => {
                if let Some(inner) = parse_angle_bracket(last) {
                    let CTypes {
                        from_c,
                        from_c_parser,
                        to_c,
                        to_c_parser,
                        ..
                    } = gen_c_types_inner(args, inner);

                    match e {
                        "Vec" => CTypes {
                            rust: ty.clone(),
                            from_c_parser: Box::new(move |ident| {
                                let inner = (from_c_parser)(ident);
                                parse_quote!(.push({
                                    let item #inner;
                                    item
                                }))
                            }),
                            from_c,
                            to_c,
                            to_c_parser,
                        },
                        "Option" => CTypes {
                            rust: ty.clone(),
                            from_c_parser: Box::new(move |ident| {
                                let inner = (from_c_parser)(ident);
                                parse_quote!(.replace({
                                    let item #inner;
                                    item
                                }))
                            }),
                            from_c,
                            to_c,
                            to_c_parser,
                        },
                        _ => CTypes {
                            rust: ty.clone(),
                            from_c,
                            from_c_parser,
                            to_c,
                            to_c_parser,
                        },
                    }
                } else {
                    let CTypes {
                        rust,
                        from_c,
                        from_c_parser,
                        to_c,
                        to_c_parser,
                    } = consider_copyable(args, ty);
                    CTypes {
                        from_c,
                        from_c_parser: Box::new(move |ident| {
                            let inner = (from_c_parser)(ident);
                            parse_quote!(= #inner)
                        }),
                        rust,
                        to_c,
                        to_c_parser,
                    }
                }
            }
        }
    } else {
        panic!("only owned path types supported")
    }
}

fn gen_c_types(field: &Field) -> CTypes {
    let (args, _) = filter_args(&field.attrs);

    gen_c_types_inner(&args, args.c_as.as_ref().unwrap_or(&field.ty))
}

fn generate_struct_field_api(ident: &Ident, field: &Field) -> Option<TokenStream2> {
    let (_, filtered_attrs) = filter_args(&field.attrs);

    let field_ident = field
        .ident
        .as_ref()
        .expect("All fields must be named ident fields");

    let CTypes {
        rust,
        from_c,
        from_c_parser,
        to_c,
        to_c_parser,
    } = gen_c_types(field);

    let wrapper_ty = get_wrapper_ty_ident(&rust).to_string();

    let fn_ident = |action| {
        Ident::new(
            &format!(
                "{}_{action}_{}",
                ident.to_string().to_case(Case::Snake),
                field_ident.to_string().to_case(Case::Snake)
            ),
            field_ident.span(),
        )
    };

    let parser = from_c_parser(&format_ident!("value"));
    let unparser = to_c_parser(&format_ident!("c_value"), &format_ident!("value"));

    match wrapper_ty.as_str() {
        "Vec" => {
            let pusher = fn_ident("push");
            let getter = fn_ident("get");
            let remove = fn_ident("remove");

            Some(quote::quote! {
                #(#filtered_attrs)*
                /// ------
                /// Pushes the new value to the end of the vector
                /// ------
                #[::crops::c_result_fn]
                fn #pusher(source: *mut #ident, value: #from_c) -> ::crops::utils::CResult {
                    ::crops::utils::check_null(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident #parser;

                    Ok(())
                }

                #(#filtered_attrs)*
                /// ------
                /// Gets the current value inside the field
                /// ------
                #[::crops::c_result_fn]
                fn #getter(source: *const #ident, idx: usize, c_value: #to_c) -> ::crops::utils::CResult {
                    let value = ::crops::utils::check_null_const(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident
                        .get(idx)
                        .ok_or_else(|| format!("Index Out of Range"))?;

                    #unparser;

                    Ok(())
                }

                #(#filtered_attrs)*
                /// ------
                /// Removes the element at the provided index, if it doesn't exist, returns an error.
                /// ------
                #[::crops::c_result_fn]
                fn #remove(source: *mut #ident, idx: usize, c_value: #to_c) -> ::crops::utils::CResult {
                    let _ = ::crops::utils::check_null_const(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident
                        .get(idx)
                        .ok_or_else(|| format!("Index Out of Range"))?;

                    let value = &::crops::utils::check_null(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident
                        .remove(idx);

                    #unparser;

                    Ok(())
                }
            })
        }
        "Option" => {
            let setter = fn_ident("replace");
            let taker = fn_ident("take");
            let getter = fn_ident("get");

            Some(quote::quote! {
                #(#filtered_attrs)*
                /// ------
                /// Replaces the current value with the provided value
                /// ------
                #[::crops::c_result_fn]
                fn #setter(source: *mut #ident, value: #from_c) -> ::crops::utils::CResult {
                    ::crops::utils::check_null(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident #parser;

                    Ok(())
                }

                #(#filtered_attrs)*
                /// ------
                /// Takes the value, removing it from the option.
                /// ------
                #[::crops::c_result_fn]
                fn #taker(source: *mut #ident, c_value: #to_c) -> ::crops::utils::CResult {
                    let value = &::crops::utils::check_null(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident
                        .take()
                        .ok_or_else(|| format!("Option Empty"))?;

                    #unparser;

                    Ok(())
                }

                #(#filtered_attrs)*
                /// ------
                /// Gets the current value within the option.
                /// ------
                #[::crops::c_result_fn]
                fn #getter(source: *const #ident, c_value: #to_c) -> ::crops::utils::CResult {
                    if let Some(value) = ::crops::utils::check_null_const(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident.as_ref()
                    {
                        #unparser;
                    }

                    Ok(())
                }
            })
        }
        _ => {
            let setter = fn_ident("with");
            let getter = fn_ident("get");

            Some(quote::quote! {
                #(#filtered_attrs)*
                /// ------
                /// Replaces the current value with the provided value
                /// ------
                #[::crops::c_result_fn]
                fn #setter(source: *mut #ident, value: #from_c) -> ::crops::utils::CResult {
                    ::crops::utils::check_null(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident #parser;

                    Ok(())
                }

                #(#filtered_attrs)*
                /// ------
                /// Gets the current value
                /// ------
                #[::crops::c_result_fn]
                fn #getter(source: *const #ident, c_value: #to_c) -> ::crops::utils::CResult {
                    let value = &::crops::utils::check_null_const(source)
                        .map_err(|e| format!("{e} ({})", stringify!(#ident)))?
                        .#field_ident;

                    #unparser;

                    Ok(())
                }
            })
        }
    }
}

fn derive_c_builder_struct(ident: Ident, attrs: Vec<Attribute>, s: DataStruct) -> TokenStream {
    let DataStruct { fields, .. } = s;

    let fields = match fields {
        Fields::Named(named) => named
            .named
            .into_iter()
            .map(|field| (field.ident.clone().unwrap().to_string(), field))
            .collect::<HashMap<_, _>>(),
        _ => panic!("Only named fields supported"),
    };

    let by_type = fields
        .values()
        .filter_map(|field| generate_struct_field_api(&ident, field));

    let mut args = CBuilderArgs::default();
    let filtered_attrs = args.extract(attrs);

    let c_default = args.c_new.then(|| {
        let new_ident = syn::Ident::new(
            &format!("{}_default", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        );
        quote::quote!(
            #(#filtered_attrs)*
            /// ------
            /// Construct a new model
            /// ------
            #[no_mangle]
            pub extern "C" fn #new_ident() -> *mut #ident {
                Box::into_raw(Box::default())
            }
        )
    });

    let c_clone = args.c_clone.then(|| {
        let clone_ident = syn::Ident::new(
            &format!("{}_clone", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        );

        quote::quote!(
            #(#filtered_attrs)*
            /// ------
            /// Clone the structure
            /// ------
            #[no_mangle]
            pub extern "C" fn #clone_ident(s: &#ident) -> *mut #ident {
                Box::into_raw(Box::new(s.clone()))
            }
        )
    });

    let c_debug = args.c_debug.then(|| {
        let debug_ident = syn::Ident::new(
            &format!("{}_debug", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        );

        quote::quote!(
            #(#filtered_attrs)*
            /// ------
            /// Print a debug of the struct to stdout
            /// ------
            #[no_mangle]
            pub extern "C" fn #debug_ident(s: &#ident) {
                println!("{:?}", s);
            }
        )
    });

    let extra_constructors = args.c_constructors.iter().map(|constructor| {
        let constructor_ident = syn::Ident::new(
            &format!(
                "{}_from_{}",
                ident.to_string().to_case(Case::Snake),
                constructor
                    .iter()
                    .map(|ident| ident.to_string().to_case(Case::Snake))
                    .join("_")
            ),
            ident.span(),
        );

        let inner_constructor =
            syn::Ident::new(&format!("inner_{}", constructor_ident), ident.span());

        let (inputs, setters): (Vec<_>, Vec<_>) = constructor
            .iter()
            .map(|ident| -> (syn::FnArg, proc_macro2::TokenStream) {
                let field = fields
                    .get(&ident.to_string())
                    .expect("Constructor fields must match struct");

                let CTypes {
                    from_c,
                    from_c_parser,
                    ..
                } = gen_c_types(field);

                let parser = from_c_parser(ident);

                (parse_quote!(#ident: #from_c), parse_quote!(.#ident #parser))
            })
            .unzip();

        quote::quote!(
            fn #inner_constructor(#(#inputs),*) -> Result<#ident, String> {
                let mut res = #ident::default();

                #(res #setters;)*

                Ok(res)
            }
            #(#filtered_attrs)*
            /// ------
            /// Unique Constructor with sepcific fields
            /// ------
            #[no_mangle]
            pub extern "C" fn #constructor_ident(#(#inputs),*) -> *mut #ident {
                let res = #inner_constructor(#(#constructor),*)
                    .expect(&format!("Error creating: {:?}", stringify!(#ident)));

                Box::into_raw(Box::new(res))
            }
        )
    });

    quote::quote! {

        #c_default

        #c_clone

        #c_debug

        #(#extra_constructors)*

        #(#by_type)*
    }
    .into()
}

fn derive_c_builder_enum(ident: Ident, attrs: Vec<Attribute>, s: DataEnum) -> TokenStream {
    let DataEnum { variants, .. } = s;

    let mut args = CBuilderArgs::default();
    let filtered_attrs = args.extract(attrs);

    let c_default = args.c_new.then(|| {
        let new_ident = syn::Ident::new(
            &format!("{}_default", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        );
        quote::quote!(
            #(#filtered_attrs)*
            /// ------
            /// Construct a new blank enum
            /// ------
            #[no_mangle]
            pub extern "C" fn #new_ident() -> *mut #ident {
                Box::into_raw(Box::default())
            }
        )
    });

    let c_clone = args.c_new.then(|| {
        let clone_ident = syn::Ident::new(
            &format!("{}_clone", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        );
        quote::quote!(
            #(#filtered_attrs)*
            /// ------
            /// Clone the enum value
            /// ------
            #[no_mangle]
            pub extern "C" fn #clone_ident(s: &#ident) -> *mut #ident {
                Box::into_raw(Box::new(s.clone()))
            }
        )
    });

    let c_debug = args.c_debug.then(|| {
        let debug_ident = syn::Ident::new(
            &format!("{}_debug", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        );

        quote::quote!(
            #(#filtered_attrs)*
            /// ------
            /// Print a debug string of the enum to stdout
            /// ------
            #[no_mangle]
            pub extern "C" fn #debug_ident(s: &#ident) {
                println!("{:?}", s);
            }
        )
    });

    let variants = variants.iter().map(|variant| {
        let Variant {
            attrs,
            fields,
            ident: var_ident,
            ..
        } = variant;

        let (_, filtered_attrs) = filter_args(attrs);

        let (input_args, enum_filler): (Vec<_>, _) = if let Fields::Unnamed(unnamed) = &fields {
            if unnamed.unnamed.len() != 1 {
                panic!("Only single unnamed fields supported");
            }

            let field = unnamed.unnamed.first().unwrap();

            let CTypes {
                from_c,
                from_c_parser,
                ..
            } = gen_c_types(field);
            let parser = from_c_parser(&parse_quote!(value));

            (
                vec![quote::quote!(value: #from_c)],
                quote::quote! {
                    ({
                        let inner #parser;
                        inner
                    })
                }
            )
        } else if matches!(fields, Fields::Unit) {
            (Default::default(), Default::default())
        } else {
            panic!("Please use a single struct as your enum data")
        };

        let as_variant_ident = syn::Ident::new(
            &format!(
                "{}_as_{}",
                ident.to_string().to_case(Case::Snake),
                var_ident.to_string().to_case(Case::Snake)
            ),
            var_ident.span(),
        );

        let new_variant_ident = syn::Ident::new(
            &format!(
                "{}_from_{}",
                ident.to_string().to_case(Case::Snake),
                var_ident.to_string().to_case(Case::Snake)
            ),
            var_ident.span(),
        );

        quote::quote!(
            #(#filtered_attrs)*
            /// ------
            /// Convert the enum into a new variant type
            /// ------
            #[::crops::c_result_fn]
            pub fn #as_variant_ident(res: *mut #ident #(, #input_args)*) -> ::crops::utils::CResult {
                let res = ::crops::utils::check_null(res)
                    .map_err(|e| format!("{e} ({})", stringify!(#ident)))?;
                *res = #ident::#var_ident #enum_filler;

                Ok(())
            }
        )
    });

    quote::quote! {
        #c_default

        #c_clone

        #c_debug

        #(#variants)*

    }
    .into()
}
