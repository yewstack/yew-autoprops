use std::marker::PhantomData;
use yew::prelude::*;
use yew_autoprops::autoprops_component;

struct TestPEqStruct<T> {
    _phantom: PhantomData<T>,
}

impl<T> PartialEq for TestPEqStruct<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
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
        <TestComponent test_struct=&TestPEqStruct { _phantom: PhantomData } />
    };
}
