use act::Action;

#[derive(Debug, Clone, Action)]
enum A {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
struct Z(u8);

#[derive(Debug, Clone, Action)]
enum B {
    X(isize),
    Y(bool),
    Z(Z),
}

fn assert_action<T: Action>() {}

fn main() {
    assert_action::<A>();
    assert_action::<B>();
}
