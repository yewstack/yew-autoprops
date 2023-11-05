#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

#[::yew_autoprops::autoprops]
#[::yew::function_component]
fn CompUseFnName() -> ::yew::Html
{
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::function_component(CompNoProperties)]
fn comp_no_properties() -> ::yew::Html
{
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::function_component(CompNoGenerics)]
fn comp_no_generics(#[prop_or_default] b: ::std::primitive::bool, a: &::yew::AttrValue) -> ::yew::Html
{
    let _: ::std::primitive::bool = b;
    let _: &::yew::AttrValue = a;
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::function_component(CompSingleGeneric)]
fn comp_single_generic<T>() -> ::yew::Html {
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::function_component(CompGenerics)]
fn comp_generics<T1, T2>(b: T1, a: &T2) -> ::yew::Html
where
    T1: ::std::cmp::PartialEq + ::std::clone::Clone,
    T2: ::std::cmp::PartialEq,
{
    let _: T1 = b;
    let _: &T2 = a;
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::function_component(CompGenericsWithoutField)]
fn comp_generics_without_field<T1, T2>(b: ::std::primitive::bool) -> ::yew::Html {
    let _ = b;
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::function_component(ConstGenerics)]
fn const_generics<const N: ::std::primitive::usize>(xs: [::std::primitive::u32; N]) -> ::yew::Html {
    let _: [::std::primitive::u32; N] = xs;
    ::yew::html! {
        <div>
            { N }
        </div>
    }
}

mod private_module {
    #[::yew_autoprops::autoprops]
    #[::yew::function_component(CompPrivateTest)]
    pub fn comp_private_test(#[prop_or_default] b: ::std::primitive::bool) -> ::yew::Html
    {
        let _: ::std::primitive::bool = b;
        ::yew::html! {
            <p></p>
        }
    }
}

use private_module::*;

#[::yew_autoprops::autoprops]
#[::yew::function_component]
fn CompHtmlResult() -> ::yew::HtmlResult
{
    ::std::result::Result::Ok(::yew::html! {
        <p></p>
    })
}

#[::yew_autoprops::autoprops]
#[::yew::function_component]
fn CompWithDefaultGeneric<T = ()>() -> ::yew::Html
{
    ::yew::html! {
        <p></p>
    }
}

fn compile_pass() {
    let _ = ::yew::html! { <CompUseFnName /> };
    let _ = ::yew::html! { <CompNoProperties /> };
    let _ = ::yew::html! { <CompNoGenerics a="foo" /> };
    let _ = ::yew::html! { <CompNoGenerics b=true a="foo" /> };
    let _ = ::yew::html! { <CompSingleGeneric<()> /> };
    let _ = ::yew::html! { <CompGenerics<::std::primitive::bool, ::yew::AttrValue> b=true a="foo" /> };
    let _ = ::yew::html! { <ConstGenerics<2> xs={[1_u32, 2_u32]} /> };
    let _ = ::yew::html! { <CompPrivateTest b=true /> };
    let _ = ::yew::html! { <CompHtmlResult /> };
    let _ = ::yew::html! { <CompWithDefaultGeneric /> };
    let _ = ::yew::html! { <CompWithDefaultGeneric<u32> /> };
}

fn main() {}
