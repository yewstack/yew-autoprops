use yew::prelude::*;
use yew_autoprops::autoprops_component;

#[autoprops_component(CoolComponent)]
fn cool_component(#[prop_or_default] test: &i8, smth: &usize) -> Html {
    println!("test: {}", test);

    let test_hook = use_state(|| 0);

    html! {
        <div>
            <p>{ smth }</p>
            <p>{ *test_hook }</p>
        </div>
    }
}

fn main() {
    let _ = html! { <CoolComponent smth=1 /> };
}
