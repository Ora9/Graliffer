use std::{
    cell::RefCell,
    hash::{BuildHasher, Hash, RandomState},
    rc::Rc,
    str::FromStr,
};

use action::{Action, AnyAction, Revert, State};
use eyre::eyre;
use log::debug;
use rand::seq::SliceRandom;

use crate::{
    ConsoleAction, ConsoleState, GridAction, GridState,
    input::{InputMode, KeyContext, Keymap},
    ui::PickerState,
};

#[derive(Debug)]
pub struct AppState {
    pub should_run: bool,

    pub console_state: ConsoleState,
    pub grid_state: GridState,
    pub command_picker_state: PickerState,

    pub keymap: Keymap,

    pub last_focused_pane: Option<PaneId>,
    pub context: Context,
}

#[derive(Debug)]
pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut grid = grai::Grid::new();

        grid.set(
            grai::Position::from_string("AA").unwrap(),
            grai::Cell::new_trim("100"),
        );
        grid.set(
            grai::Position::from_string("BA").unwrap(),
            grai::Cell::new_trim("&BB"),
        );
        grid.set(
            grai::Position::from_string("CA").unwrap(),
            grai::Cell::new_trim("div"),
        );
        grid.set(
            grai::Position::from_string("BB").unwrap(),
            grai::Cell::new_trim("@CB"),
        );
        grid.set(
            grai::Position::from_string("CB").unwrap(),
            grai::Cell::new_trim("3"),
        );

        grid.set(
            grai::Position::from_string("EA").unwrap(),
            grai::Cell::new_trim("20"),
        );
        grid.set(
            grai::Position::from_string("FA").unwrap(),
            grai::Cell::new_trim("sub"),
        );
        grid.set(
            grai::Position::from_string("HA").unwrap(),
            grai::Cell::new_trim("@AB"),
        );
        grid.set(
            grai::Position::from_string("IA").unwrap(),
            grai::Cell::new_trim("set"),
        );
        grid.set(
            grai::Position::from_string("aa").unwrap(),
            grai::Cell::new_trim("jmp"),
        );
        let mut frame = Rc::new(RefCell::new(grai::Frame {
            grid,
            head: grai::Head::default(),
            stack: grai::Stack::default(),
        }));

        let context = Context::new(PaneId::Grid, InputMode::Insert);

        let mut app = Self {
            context: context.clone(),

            should_run: true,

            keymap: Keymap::new(),

            console_state: ConsoleState::new(1000, context.clone()),
            grid_state: GridState::new(frame, context),
            command_picker_state: PickerState::new(),

            last_focused_pane: None,
        };

        let mut rng = rand::rng();
        let phrase = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();

        let mut shuffler = || {
            let mut phrase = phrase.split(" ").collect::<Vec<&str>>();
            phrase.shuffle(&mut rng);
            phrase.join(" ").to_string()
        };

        for i in 0..100 {
            app.console_state.append_line(shuffler());
        }

        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // self.console_state.scroll_down_by(1);

        // self.console_state.scroll_offset = self.console_state.scroll_offset.wrapping_add(1);
    }

    pub fn key_context(&self) -> KeyContext {
        KeyContext {
            focus: self.focused(),
            input_mode: self.input_mode(),
        }
    }

    pub fn is_focused(&self, focus_id: impl Into<FocusId>) -> bool {
        self.focused() == focus_id.into()
    }

    pub fn focused(&self) -> FocusId {
        self.context.get_focus()
    }

    pub fn set_focus(&mut self, focus_id: impl Into<FocusId>) {
        self.context.set_focus(focus_id);
    }

    pub fn popup_opened(&self) -> bool {
        matches!(self.focused(), FocusId::Popup(_))
    }

    pub fn close_popup(&mut self) {
        if let Some(last_focus) = self.last_focused_pane {
            self.set_focus(last_focus);
        }
    }

    pub fn toggle_popup(&mut self, focus_id: PopupId) {
        if self.is_focused(focus_id) {
            self.close_popup();
        } else {
            if let FocusId::Pane(pane) = self.focused() {
                self.last_focused_pane = Some(pane);
            }
            self.set_focus(focus_id);
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.context.get_input_mode()
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.context.set_input_mode(input_mode);
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_run = false;
    }
}

#[derive(Debug, Clone)]
pub struct ContextInner {
    focus: FocusId,
    input_mode: InputMode,
}

#[derive(Debug, Clone)]
pub struct Context(RefCell<ContextInner>);

impl Context {
    pub fn new(focus: impl Into<FocusId>, input_mode: InputMode) -> Self {
        Self(RefCell::new(ContextInner {
            focus: focus.into(),
            input_mode,
        }))
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.0.borrow().input_mode
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.0.get_mut().input_mode = input_mode
    }

    pub fn get_focus(&self) -> FocusId {
        self.0.borrow().focus
    }

    pub fn set_focus(&mut self, focus_id: impl Into<FocusId>) {
        self.0.get_mut().focus = focus_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupId {
    About,
    CommandPicker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneId {
    Grid,
    Console,
    Stack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusId {
    Pane(PaneId),
    Popup(PopupId),
}

impl FocusId {
    pub fn is_pane(&self) -> bool {
        matches!(self, FocusId::Pane(_))
    }

    pub fn is_popup(&self) -> bool {
        matches!(self, FocusId::Popup(_))
    }
}

impl From<PaneId> for FocusId {
    fn from(value: PaneId) -> Self {
        Self::Pane(value)
    }
}

impl From<PopupId> for FocusId {
    fn from(value: PopupId) -> Self {
        Self::Popup(value)
    }
}

#[derive(Debug, Clone, strum::EnumString)]
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

#[derive(Debug, Clone)]
pub enum ConcreteAnyAction {
    AppAction(AppAction),
    ConsoleAction(ConsoleAction),
    GridAction(GridAction),
}

impl TryFrom<AnyAction> for ConcreteAnyAction {
    type Error = eyre::Error;

    fn try_from(action: AnyAction) -> Result<Self, Self::Error> {
        if let Some(app_action) = action.downcast_ref::<AppAction>() {
            Ok(Self::AppAction(app_action.clone()))
        } else if let Some(console_action) = action.downcast_ref::<ConsoleAction>() {
            Ok(Self::ConsoleAction(console_action.clone()))
        } else if let Some(grid_action) = action.downcast_ref::<GridAction>() {
            Ok(Self::GridAction(grid_action.clone()))
        } else {
            Err(eyre!("unknown action"))
        }
    }
}

impl TryFrom<&str> for ConcreteAnyAction {
    type Error = eyre::Error;

    fn try_from(action: &str) -> Result<Self, Self::Error> {
        if let Some((namespace, action)) = action.rsplit_once("::") {
            match namespace.to_ascii_lowercase().as_str() {
                "console" => Ok(Self::ConsoleAction(ConsoleAction::from_str(action)?)),
                "grid" => Ok(Self::GridAction(GridAction::from_str(action)?)),
                _ => Err(eyre!("unknown action namespace")),
            }
        } else {
            Ok(Self::AppAction(AppAction::from_str(action)?))
        }
    }
}

impl Action for ConcreteAnyAction {}

impl State for AppState {
    type Action = ConcreteAnyAction;
    type Error = eyre::Error;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error> {
        use AppAction::*;

        match action {
            ConcreteAnyAction::ConsoleAction(console_action) => {
                self.console_state.act(console_action);
            }
            ConcreteAnyAction::GridAction(grid_action) => {
                self.grid_state.act(grid_action);
            }
            ConcreteAnyAction::AppAction(app_action) => match app_action {
                Quit => {
                    self.quit();
                }
                ToggleAbout => {
                    self.toggle_popup(PopupId::About);
                }
                ToggleCommandPicker => {
                    self.toggle_popup(PopupId::CommandPicker);
                }
                ClosePopup => {
                    self.close_popup();
                }
                FocusStack => {
                    self.set_focus(PaneId::Stack);
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
