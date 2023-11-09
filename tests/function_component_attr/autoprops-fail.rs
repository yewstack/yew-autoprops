use yew::prelude::*;
use yew::html::ImplicitClone;
use yew_autoprops::*;

#[autoprops]
#[function_component]
fn CantAcceptReceiver(&self, b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[autoprops]
fn not_a_function_component(b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[function_component(WrongAttrsOrder)]
#[autoprops]
fn wrong_attrs_order(b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[autoprops]
#[function_component(let)]
fn BadFunctionComponent(b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[derive(PartialEq)]
struct NotClonable(u32);

#[autoprops]
#[function_component]
fn TypeIsNotClone(stuff: NotClonable) -> Html {
    drop(stuff);
    html! {
        <p></p>
    }
}

#[derive(Clone)]
struct NotPartialEq(u32);

impl ImplicitClone for NotPartialEq {}

#[autoprops]
#[function_component]
fn TypeIsNotPartialEq(stuff: NotPartialEq) -> Html {
    drop(stuff);
    html! {
        <p></p>
    }
}

#[autoprops]
#[function_component]
fn InvalidFieldName(_: u32) -> Html {
    html! {
        <p></p>
    }
}

mod private_module {
    #[::yew_autoprops::autoprops]
    #[::yew::function_component(CompPrivateTest)]
    fn comp_private_test(#[prop_or_default] b: ::std::primitive::bool) -> ::yew::Html
    {
        let _: ::std::primitive::bool = b;
        ::yew::html! {
            <p></p>
        }
    }
}

#[allow(unused_imports)]
use private_module::*;

fn main() {
    let _ = html! { <CompPrivateTest /> };
}
