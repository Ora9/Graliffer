use act::{Action, State};

#[derive(Clone, Debug, Action)]
enum DuckAction {
    Fly,
    Walk,
    Swim,
}

// use `State::Action` bound to verify that `DuckAction` implement `Action`
#[derive(Debug)]
struct DummyState;

impl State for DummyState {
    type Action = DuckAction;
    type Error = std::convert::Infallible;

    fn act(&mut self, _action: impl Into<Self::Action>) -> Result<(), <Self as State>::Error> {
        unimplemented!()
    }
}

fn main() {}
