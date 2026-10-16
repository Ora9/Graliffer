#![allow(unused)]

#[test]
fn derive_unit_enum() {
    #[derive(Debug, Clone, act::Action)]
    enum Foo {
        A,
        B,
        C,
    }

    // test that Foo is Action
}

#[test]
fn derive_parametered_enum() {
    #[derive(Debug, Clone)]
    enum Bar {
        X,
        Y,
        Z,
    }

    #[derive(Debug, Clone, act::Action)]
    enum Foo {
        A(u32),
        B(Bar),
        C,
    }
}

#[test]
fn derive_action_name() {
    use act::{Action, ActionDisplay};

    #[derive(Debug, Clone, Action, ActionDisplay)]
    enum DuckAction {
        #[act(action(human_name = "fly away", machine_name = "FlyAway"))]
        Fly,
        Walk,
        Swim,
        QuackQuack,
    }

    assert_eq!(DuckAction::Fly.human_name(), "fly away");
    assert_eq!(DuckAction::Fly.machine_name(), "FlyAway");

    assert_eq!(DuckAction::Walk.human_name(), "walk");
    assert_eq!(DuckAction::Walk.machine_name(), "Walk");

    assert_eq!(DuckAction::QuackQuack.human_name(), "quackquack")
}

// #[test]
// fn derive_nested_namespace() {
//     #[derive(Debug, Clone, act::Action)]
//     enum Bar {
//         A,
//         B,
//     }
//
//     #[derive(Debug, Clone)]
//     enum Baz {
//         A,
//         B,
//     }
//
//     #[derive(Debug, Clone, act::Action)]
//     enum Foo {
//         #[act(nested(namespace = "bar", transparent))]
//         Bar(Bar),
//         Baz(Baz),
//     }
// }
