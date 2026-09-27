use act::{
    Action,
    timeline::{Revert, TimelinedState},
};

#[derive(Debug, Clone)]
pub enum FrogAction {
    SetName(String),
    IncrementHappiness,
    DecrementHappiness,
    ToggleHide,
}

impl Action for FrogAction {}

#[derive(Debug)]
pub enum FrogError {
    NotThatLow,
    TooMuchHappiness,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Frog {
    pub name: String,
    pub happiness: u8,
    pub hidden: bool,
}

impl TimelinedState for Frog {
    type Action = FrogAction;
    type Error = FrogError;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert<Self>, Self::Error> {
        match action.into() {
            FrogAction::IncrementHappiness => {
                self.happiness = self
                    .happiness
                    .checked_add(1)
                    .ok_or(FrogError::TooMuchHappiness)?;

                Ok(Revert::new(FrogAction::DecrementHappiness))
            }
            FrogAction::DecrementHappiness => {
                self.happiness = self.happiness.checked_sub(1).ok_or(FrogError::NotThatLow)?;

                Ok(Revert::new(FrogAction::IncrementHappiness))
            }
            FrogAction::SetName(new_name) => {
                let old_name = std::mem::replace(&mut self.name, new_name);

                Ok(Revert::new(FrogAction::SetName(old_name)))
            }
            FrogAction::ToggleHide => {
                self.hidden = !self.hidden;

                Ok(Revert::new(FrogAction::ToggleHide))
            }
        }
    }
}

#[test]
fn meta_default_test() {
    assert_eq!(
        Frog::default(),
        Frog {
            happiness: 0,
            name: String::from(""),
            hidden: false,
        }
    );
}

#[test]
fn increment_happiness() {
    let mut simple = Frog::default();
    simple.act(FrogAction::IncrementHappiness);

    assert_eq!(simple.happiness, 1);
}

#[test]
fn decrement_happiness() {
    let mut simple = Frog::default();
    let error = simple.act(FrogAction::DecrementHappiness);

    // assert_eq!();
}

#[test]
fn set_name() {
    let mut simple = Frog::default();
    simple.act(FrogAction::SetName(String::from("sofa")));

    assert_eq!(simple.name, "sofa");
}

#[test]
fn toggle_hide() {
    let mut simple = Frog::default();
    simple.act(FrogAction::ToggleHide);

    assert_eq!(simple.hidden, true);
}

fn main() {}
