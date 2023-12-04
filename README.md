# yew-autoprops

<a href="https://crates.io/crates/yew-autoprops"><img alt="Crate Info" src="https://img.shields.io/crates/v/yew-autoprops.svg"/></a>
<a href="https://docs.rs/yew-autoprops/"><img alt="API Docs" src="https://img.shields.io/docsrs/yew-autoprops"/></a>

proc-macro to automatically derive Properties structs from args for Yew components

No more extra one-off Props structs!

# Examples

```rust
use yew_autoprops::autoprops;
use yew::prelude::*;

#[autoprops]
#[function_component]
fn CoolComponent(#[prop_or_default] hidden: bool, smth: &AttrValue) -> Html {
    html! {
        <div class={classes!(hidden.then_some("hidden"))}>
            <p>{ smth }</p>
        </div>
    }
}
```

```rust
use yew_autoprops::autoprops;
use yew::prelude::*;

#[autoprops(CoolComponentProps)]
#[function_component(CoolComponent)]
fn cool_component(#[prop_or_default] hidden: bool, smth: &AttrValue) -> Html {
    html! {
        <div class={classes!(hidden.then_some("hidden"))}>
            <p>{ smth }</p>
        </div>
    }
}
```
