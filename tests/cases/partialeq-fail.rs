use yew::prelude::*;
use yew_autoprops::autoprops_component;

struct TestPEqStruct<T> {
    t: T,
}

impl<T: PartialEq> PartialEq for TestPEqStruct<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

struct NotPartialEq();

#[autoprops_component]
fn TestComponent(test_struct: &TestPEqStruct<NotPartialEq>) -> Html {
    html! {
        <div>
            { "TestComponent" }
        </div>
    }
}

fn main() {
    html! {
        <TestComponent test_struct={TestPEqStruct { t: NotPartialEq() }} />
    };
}
