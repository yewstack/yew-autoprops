use yew::prelude::*;
use yew_autoprops::autoprops_component;

#[autoprops_component]
fn GenericComponent<T: ToString>(value: &T) -> Html {
    html! {
        <div>
            { value.to_string() }
        </div>
    }
}

fn main() {
    html! {
        <>
            <GenericComponent<u64> value=1 />
            <GenericComponent<String> value="hi" />
        </>
    };
}
