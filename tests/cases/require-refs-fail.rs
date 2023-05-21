use yew_autoprops::autoprops_component;

#[autoprops_component]
fn CoolComponent(test: i8, smth: &usize) -> Html {
    println!("test: {}", test);

    html! {
        <div>
            <p>{ smth }</p>
        </div>
    }
}

fn main() {}
