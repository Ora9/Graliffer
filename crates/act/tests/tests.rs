use act::{Action, Revert, State};

#[derive(Debug, Clone)]
enum TestAction {
    IncrementFoo,
    DecrementFoo,
    SetBar(String),
    ToggleBaz,
}

impl Action for TestAction {}

#[derive(Debug)]
enum TestError {}

#[derive(Debug, Default, PartialEq, Eq)]
struct Test {
    foo: u8,
    bar: String,
    baz: bool,
}

impl State for Test {
    type Action = TestAction;
    type Error = TestError;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        match action.into() {
            TestAction::IncrementFoo => {
                self.foo = self.foo.wrapping_add(1);

                Ok(Revert::new_apply(TestAction::DecrementFoo))
            }
            TestAction::DecrementFoo => {
                self.foo = self.foo.wrapping_sub(1);

                Ok(Revert::new_apply(TestAction::IncrementFoo))
            }
            TestAction::SetBar(new_bar) => {
                let old_bar = self.bar.clone();
                self.bar = new_bar;

                Ok(Revert::new_apply(TestAction::SetBar(old_bar)))
            }
            TestAction::ToggleBaz => {
                self.baz = !self.baz;

                Ok(Revert::new_apply(TestAction::ToggleBaz))
            }
        }
    }
}

#[test]
fn meta_default_test() {
    assert_eq!(
        Test::default(),
        Test {
            foo: 0,
            bar: String::from(""),
            baz: false,
        }
    );
}

#[test]
fn increment_foo() {
    let mut test = Test::default();
    test.act(TestAction::IncrementFoo);

    assert_eq!(test.foo, 1);
}

#[test]
fn decrement_foo() {
    let mut test = Test::default();
    test.act(TestAction::DecrementFoo);

    assert_eq!(test.foo, 255);
}

#[test]
fn set_bar() {
    let mut test = Test::default();
    test.act(TestAction::SetBar(String::from("sofa")));

    assert_eq!(test.bar, "sofa");
}

#[test]
fn toggle_baz() {
    let mut test = Test::default();
    test.act(TestAction::ToggleBaz);

    assert_eq!(test.baz, true);
}
