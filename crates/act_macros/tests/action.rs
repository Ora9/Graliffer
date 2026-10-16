#![allow(unused)]

#[test]
fn derive_unit_enum() {
    #[derive(Debug, Clone, act::Action)]
    enum Unit {
        A,
        B,
        C,
    }
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
    #[derive(Debug, Clone, act::Action, act::ActionDisplay)]
    enum Foo {
        #[act(action(human_name = "human a", machine_name = "MachineA"))]
        A,
        B,
        C,
    }
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
