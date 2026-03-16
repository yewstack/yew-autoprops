#[::yew_autoprops::autoprops]
#[::yew::component]
fn CompUseFnName() -> ::yew::Html
{
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::component(CompNoProperties)]
fn comp_no_properties() -> ::yew::Html
{
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::component(CompNoGenerics)]
fn comp_no_generics(#[prop_or_default] b: ::std::primitive::bool, a: &::yew::AttrValue) -> ::yew::Html
{
    let _: ::std::primitive::bool = b;
    let _: &::yew::AttrValue = a;
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::component(CompSingleGeneric)]
fn comp_single_generic<T>() -> ::yew::Html {
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::component(CompGenerics)]
fn comp_generics<T1, T2>(b: T1, a: &T2) -> ::yew::Html
where
    T1: ::std::cmp::PartialEq + ::implicit_clone::ImplicitClone,
    T2: ::std::cmp::PartialEq,
{
    let _: T1 = b;
    let _: &T2 = a;
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::component(CompGenericsWithoutField)]
fn comp_generics_without_field<T1, T2>(b: ::std::primitive::bool) -> ::yew::Html {
    let _ = b;
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::component(ConstGenerics)]
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
    #[::yew::component(CompPrivateTest)]
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
#[::yew::component]
fn CompHtmlResult() -> ::yew::HtmlResult {
    ::std::result::Result::Ok(::yew::html! {
        <p></p>
    })
}

#[::yew_autoprops::autoprops]
#[::yew::component]
fn CompWithDefaultGeneric<T = ()>() -> ::yew::Html
where
    T: ::std::default::Default,
{
    let _ = T::default();
    ::yew::html! {
        <p></p>
    }
}

#[::yew_autoprops::autoprops]
#[::yew::component]
fn CompWithIntendedRef(s: &'static ::std::primitive::str) -> ::yew::Html {
    let _ = s;
    ::yew::html! {}
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
    let _ = ::yew::html! { <CompWithDefaultGeneric<::std::primitive::u32> /> };
    let _ = ::yew::html! { <CompWithIntendedRef s="foo" /> };
}

fn main() {}
