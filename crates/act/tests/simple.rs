use act::State;

mod common;
use common::{Simple, SimpleAction};

#[test]
fn meta_default_test() {
    assert_eq!(
        Simple::default(),
        Simple {
            foo: 0,
            bar: String::from(""),
            baz: false,
        }
    );
}

#[test]
fn increment_foo() {
    let mut simple = Simple::default();
    simple.act(SimpleAction::IncrementFoo);

    assert_eq!(simple.foo, 1);
}

#[test]
fn decrement_foo() {
    let mut simple = Simple::default();
    simple.act(SimpleAction::DecrementFoo);

    assert_eq!(simple.foo, 255);
}

#[test]
fn set_bar() {
    let mut simple = Simple::default();
    simple.act(SimpleAction::SetBar(String::from("sofa")));

    assert_eq!(simple.bar, "sofa");
}

#[test]
fn toggle_baz() {
    let mut simple = Simple::default();
    simple.act(SimpleAction::ToggleBaz);

    assert_eq!(simple.baz, true);
}
