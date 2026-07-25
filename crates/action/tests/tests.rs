use action::{Action, Revert, State};

#[derive(Debug, Clone)]
enum TestAction {
    IncrementFoo,
    DecrementFoo,
    SetBar(String),
}

impl Action for TestAction {}

#[derive(Debug)]
enum TestError {}

#[derive(Debug)]
struct Test {
    foo: u32,
    bar: String,
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
        }
    }
}
