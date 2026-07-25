use std::{collections::HashMap, convert::Infallible, error::Error, fmt::Display};

use act::{Action, Revert, State};

use crate::common::pond::PondAction::BeginPatPatingFrog;

#[derive(Debug)]
pub struct Camp {
    hamoc: bool,
    fire: bool,
}

#[derive(Debug, Clone)]
pub enum CampAction {
    SetupHamoc(bool),
    LitFire(bool),
}

impl Action for CampAction {}

impl State for Camp {
    type Action = CampAction;
    type Error = Infallible;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        match action.into() {
            CampAction::LitFire(state) => {
                let revert = Revert::new_apply(CampAction::LitFire(self.fire));
                self.fire = state;

                Ok(revert)
            }
            CampAction::SetupHamoc(state) => {
                let revert = Revert::new_apply(CampAction::SetupHamoc(self.hamoc));
                self.hamoc = state;

                Ok(revert)
            }
        }
    }
}

type FrogName = String;
#[derive(Debug, Default)]
pub struct Frog {
    pub happy: bool,
}

#[derive(Debug)]
pub struct Pond {
    camp: Camp,

    is_patpating: bool,
    frogs: HashMap<FrogName, Frog>,
}

#[derive(Debug)]
pub enum PondError {
    NoFrogThisName(FrogName),
}

impl Display for PondError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFrogThisName(from_name) => write!(f, "no frog has the name `{from_name}`"),
        }
    }
}

impl Error for PondError {}

#[derive(Debug, Clone)]
pub enum PondAction {
    CampAction(CampAction),

    IntroduceFrog(FrogName),
    SendFrogToATrip(FrogName),
    BeginPatPatingFrog(FrogName),
    StopPatPatingFrog(FrogName), // But why tho ??
}

impl Action for PondAction {}

impl State for Pond {
    type Action = PondAction;
    type Error = PondError;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        match action.into() {
            PondAction::CampAction(camp_action) => {
                self.camp.act(camp_action).map_err(|_| unreachable!())
            }
            PondAction::IntroduceFrog(frog_name) => {
                self.frogs.insert(frog_name.clone(), Frog::default());
                Ok(Revert::new_apply(PondAction::SendFrogToATrip(frog_name)))
            }
            PondAction::SendFrogToATrip(frog_name) => {
                self.frogs.remove(&frog_name);
                Ok(Revert::new_apply(PondAction::IntroduceFrog(frog_name)))
            }
            PondAction::BeginPatPatingFrog(frog_name) => {
                if let Some(frog) = self.frogs.get_mut(&frog_name) {
                    frog.happy = true;
                    Ok(Revert::new_apply(PondAction::StopPatPatingFrog(frog_name)))
                } else {
                    return Err(PondError::NoFrogThisName(frog_name));
                }
            }
            PondAction::StopPatPatingFrog(frog_name) => {
                if let Some(frog) = self.frogs.get_mut(&frog_name) {
                    frog.happy = false;
                    Ok(Revert::new_apply(BeginPatPatingFrog(frog_name)))
                } else {
                    Err(PondError::NoFrogThisName(frog_name))
                }
            }
        }
    }
}
