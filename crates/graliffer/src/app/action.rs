use std::{convert::Infallible, str::FromStr};

use crate::{
    AboutView, AppState, ConsoleAction, GridAction, InputMode, PickerAction, PickerView, PopupId,
    StackView, View,
};
use serde::{Deserialize, Serialize};

use act::{Action, IntoState, Revert, State};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ActionParseError {
    #[error("unknown action namespace in `{source}`")]
    UnknownNamespace { r#source: String },

    #[error("unknown action `{action}` in `{source}`")]
    UnknownAction { action: String, r#source: String },
}

#[derive(Debug, Clone, strum::EnumString, Serialize, Deserialize)]
pub enum GralifferAction {
    Quit,
    ClosePopup,
    FocusStack,
    ToggleCommandPicker,
    ToggleAbout,
    InsertMode,
    CommandMode,
}

#[derive(Debug, Clone, strum::EnumString, Serialize, Deserialize)]
pub enum GraiAction {
    Step,

    #[strum(disabled)]
    #[serde(skip)]
    Frame(grai::FrameAction),
}

impl From<grai::FrameAction> for GraiAction {
    fn from(value: grai::FrameAction) -> Self {
        Self::Frame(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub enum AppAction {
    GralifferAction(GralifferAction),
    Grai(GraiAction),
    ConsoleAction(ConsoleAction),
    GridAction(GridAction),
    PickerAction(PickerAction),
}

impl Action for AppAction {}

impl From<GraiAction> for AppAction {
    fn from(value: GraiAction) -> Self {
        Self::Grai(value)
    }
}

impl From<grai::FrameAction> for AppAction {
    fn from(value: grai::FrameAction) -> Self {
        Self::Grai(value.into())
    }
}

impl From<GralifferAction> for AppAction {
    fn from(value: GralifferAction) -> Self {
        Self::GralifferAction(value)
    }
}

impl From<ConsoleAction> for AppAction {
    fn from(value: ConsoleAction) -> Self {
        Self::ConsoleAction(value)
    }
}

impl From<GridAction> for AppAction {
    fn from(value: GridAction) -> Self {
        Self::GridAction(value)
    }
}

impl From<PickerAction> for AppAction {
    fn from(value: PickerAction) -> Self {
        Self::PickerAction(value)
    }
}

impl FromStr for AppAction {
    type Err = ActionParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        if let Some((namespace, action)) = source.rsplit_once("::") {
            match namespace {
                "grai" => Ok(GraiAction::from_str(action)
                    .map_err(|_| ActionParseError::UnknownAction {
                        action: action.to_string(),
                        source: namespace.to_string(),
                    })?
                    .into()),
                "grid" => Ok(GridAction::from_str(action)
                    .map_err(|_| ActionParseError::UnknownAction {
                        action: action.to_string(),
                        source: namespace.to_string(),
                    })?
                    .into()),
                "console" => Ok(ConsoleAction::from_str(action)
                    .map_err(|_| ActionParseError::UnknownAction {
                        action: action.to_string(),
                        source: namespace.to_string(),
                    })?
                    .into()),
                "picker" => Ok(PickerAction::from_str(action)
                    .map_err(|_| ActionParseError::UnknownAction {
                        action: action.to_string(),
                        source: namespace.to_string(),
                    })?
                    .into()),
                _ => Err(ActionParseError::UnknownNamespace {
                    source: namespace.to_string(),
                }),
            }
        } else {
            // Defaults to GralifferAction
            Ok(GralifferAction::from_str(source)
                .map_err(|_| ActionParseError::UnknownAction {
                    action: source.to_string(),
                    source: source.to_string(),
                })?
                .into())
        }
    }
}

impl TryFrom<String> for AppAction {
    type Error = ActionParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl State for AppState {
    type Action = AppAction;
    type Error = Infallible;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert<Self>, Self::Error> {
        match action.into() {
            AppAction::GridAction(grid_action) => match self.grid_state.act(grid_action) {
                Ok(revert) => Ok(revert.into_state()),
            },
            AppAction::ConsoleAction(console_action) => {
                match self.console_state.act(console_action) {
                    Ok(revert) => Ok(revert.into_state()),
                }
            }
            AppAction::PickerAction(picker_action) => {
                match self.command_picker_state.act(picker_action) {
                    Ok(revert) => Ok(revert.into_state()),
                }
            }
            AppAction::Grai(grai_action) => {
                use GraiAction::*;
                let revert = match grai_action {
                    // TODO: These unwraps must go away!
                    Step => self.frame.act(grai::FrameAction::Step).unwrap(),
                    Frame(action) => self.frame.act(action).unwrap(),
                };

                Ok(revert.into_state())
            }
            AppAction::GralifferAction(app_action) => {
                use GralifferAction::*;
                match app_action {
                    Quit => {
                        self.quit();
                    }
                    ToggleAbout => {
                        self.toggle_popup(PopupId::from(AboutView::title().as_str()));
                    }
                    ToggleCommandPicker => {
                        self.toggle_popup(PopupId::from(PickerView::title().as_str()));
                    }
                    ClosePopup => {
                        self.close_popup();
                    }
                    FocusStack => {
                        self.set_focus(StackView::view_id());
                    }
                    InsertMode => {
                        self.set_input_mode(InputMode::Insert);
                    }
                    CommandMode => {
                        self.set_input_mode(InputMode::Command);
                    }
                };
                Ok(Revert::None)
            }
        }
    }
}
