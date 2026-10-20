use act::Action;

#[derive(Clone, Debug, Action)]
enum DuckAction {
    Fly,
    Walk,
    Swim,
}

fn assert_action<T: Action>() {}

fn main() {
    assert_action::<DuckAction>();
}
