use std::str::FromStr;

use crate::{
    AboutView, AppState, ConsoleAction, ConsoleActionError, GridAction, GridActionError, InputMode,
    PickerAction, PickerActionError, PickerView, PopupId, StackView, View,
};
use serde::{Deserialize, Serialize};

use action::{Action, AnyAction, Revert, State};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ActionParseError {
    #[error("unknown action namespace in `{source}`")]
    UnknownNamespace { r#source: String },

    #[error("unknown action `{action}` in `{source}`")]
    UnknownAction { action: String, r#source: String },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AnyAppActionError {
    #[error("app action error")]
    App(#[from] AppActionError),

    #[error("grid action error")]
    Grid(#[from] GridActionError),

    #[error("console action error")]
    Console(#[from] ConsoleActionError),

    #[error("picker action error")]
    Picker(#[from] PickerActionError),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("app action error")]
pub struct AppActionError;

#[derive(Debug, Clone, strum::EnumString, Serialize, Deserialize)]
pub enum AppAction {
    Quit,
    ClosePopup,
    FocusStack,
    ToggleCommandPicker,
    ToggleAbout,
    InsertMode,
    CommandMode,
}

impl Action for AppAction {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub enum AnyAppAction {
    AppAction(AppAction),
    ConsoleAction(ConsoleAction),
    GridAction(GridAction),
    PickerAction(PickerAction),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IntoActionError {
    #[error("unknown action")]
    UnknownAction,
}

impl TryFrom<AnyAction> for AnyAppAction {
    type Error = IntoActionError;

    fn try_from(action: AnyAction) -> Result<Self, Self::Error> {
        if let Some(app_action) = action.downcast_ref::<AppAction>() {
            Ok(Self::AppAction(app_action.clone()))
        } else if let Some(console_action) = action.downcast_ref::<ConsoleAction>() {
            Ok(Self::ConsoleAction(console_action.clone()))
        } else if let Some(grid_action) = action.downcast_ref::<GridAction>() {
            Ok(Self::GridAction(grid_action.clone()))
        } else if let Some(picker_action) = action.downcast_ref::<PickerAction>() {
            Ok(Self::PickerAction(picker_action.clone()))
        } else {
            Err(IntoActionError::UnknownAction)
        }
    }
}

impl TryFrom<String> for AnyAppAction {
    type Error = ActionParseError;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        if let Some((namespace, action)) = source.rsplit_once("::") {
            // let action = action.to_ascii_lowercase();
            // let namespace = namespace.to_ascii_lowercase();

            match namespace {
                "console" => Ok(Self::ConsoleAction(
                    ConsoleAction::from_str(&action).map_err(|_| {
                        ActionParseError::UnknownAction {
                            action: action.to_string(),
                            source: source.to_owned(),
                        }
                    })?,
                )),
                "grid" => Ok(Self::GridAction(GridAction::from_str(&action).map_err(
                    |_| ActionParseError::UnknownAction {
                        action: action.to_string(),
                        source: source.to_owned(),
                    },
                )?)),
                "picker" => Ok(Self::PickerAction(
                    PickerAction::from_str(&action).map_err(|_| {
                        ActionParseError::UnknownAction {
                            action: action.to_string(),
                            source: source.to_owned(),
                        }
                    })?,
                )),
                _ => Err(ActionParseError::UnknownNamespace { source }),
            }
        } else {
            let action = source.clone();
            Ok(Self::AppAction(AppAction::from_str(&action).map_err(
                |_| ActionParseError::UnknownAction {
                    action: action,
                    source: source,
                },
            )?))
        }
    }
}

impl Action for AnyAppAction {}

impl State for AppState {
    type Action = AnyAppAction;
    type Error = AnyAppActionError;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error> {
        use AppAction::*;

        match action {
            AnyAppAction::ConsoleAction(console_action) => {
                self.console_state.act(console_action)?;
            }
            AnyAppAction::GridAction(grid_action) => {
                self.grid_state.act(grid_action)?;
            }
            AnyAppAction::PickerAction(picker_action) => {
                self.command_picker_state.act(picker_action)?;
            }
            AnyAppAction::AppAction(app_action) => match app_action {
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
            },
        };
        Ok(Revert::None)
    }
}
