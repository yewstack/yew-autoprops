# yew-autoprops

<a href="https://crates.io/crates/yew-autoprops"><img alt="Crate Info" src="https://img.shields.io/crates/v/yew-autoprops.svg"/></a>
<a href="https://docs.rs/yew-autoprops/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-yew-autoprops-green"/></a>

proc-macro to automatically derive Properties structs from args for Yew components

No more extra one-off Props structs!

# Examples

```
use yew_autoprops::autoprops_component;
use yew::prelude::*;

#[autoprops_component]
fn CoolComponent(#[prop_or_default] test: &i8, smth: &usize) -> Html {
    println!("test: {}", test);

    html! {
        <div>
            <p>{ smth }</p>
        </div>
    }
}
```

```
use yew_autoprops::autoprops_component;
use yew::prelude::*;

#[autoprops_component(CoolComponent)]
fn cool_component(#[prop_or_default] test: &i8, smth: &usize) -> Html {
    println!("test: {}", test);

    html! {
        <div>
            <p>{ smth }</p>
        </div>
    }
}
```
