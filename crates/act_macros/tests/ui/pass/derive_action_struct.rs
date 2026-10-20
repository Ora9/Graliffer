use act::Action;

#[derive(Clone, Debug, Action)]
struct A;

#[derive(Clone, Debug, Action)]
struct B {
    i: isize,
    b: bool,
}

fn assert_action<T: Action>() {}

fn main() {
    assert_action::<A>();
    assert_action::<B>();
}
