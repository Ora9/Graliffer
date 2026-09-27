use act::{Action, State};

#[derive(Debug, Clone)]
enum Duck {
    Walking,
    Swiming,
    Flying,
}

#[derive(Debug, Clone)]
enum DuckAction {
    Walk,
    Swim,
    Fly,
}

impl Action for DuckAction {}

impl State for Duck {
    type Action = DuckAction;
    type Error = std::convert::Infallible;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<(), Self::Error> {
        match action.into() {
            DuckAction::Fly => *self = Duck::Flying,
            DuckAction::Swim => *self = Duck::Swiming,
            DuckAction::Walk => *self = Duck::Walking,
        }

        Ok(())
    }
}

fn main() {}
