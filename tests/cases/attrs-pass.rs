use yew::prelude::*;
use yew_autoprops::autoprops_component;

#[autoprops_component]
fn CoolComponent(#[prop_or_default] test: &i8, smth: &usize) -> Html {
    println!("test: {}", test);

    html! {
        <div>
            <p>{ smth }</p>
        </div>
    }
}

#[autoprops_component]
fn CoolComponent2(me: &i8, #[prop_or_default] pls: &usize) -> Html {
    println!("pls: {}", pls);

    html! {
        <div>
            <p>{ me }</p>
        </div>
    }
}

#[autoprops_component]
fn CoolComponent3(me: &i8, #[prop_or(2)] pls: &usize) -> Html {
    println!("pls: {}", pls);

    html! {
        <div>
            <p>{ me }</p>
        </div>
    }
}

fn main() {
    fn main() {
        let _ = html! { <CoolComponent smth=1 /> };
        let _ = html! { <CoolComponent3 me=2 /> };
        let _ = html! { <CoolComponent3 me=2 pls=4 /> };
    }
}
