use std::convert::Infallible;

use act::{Action, Revert, State};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Offset, Rect, Size, Spacing},
    style::{
        Color::{Black, White},
        Modifier, Style,
    },
    symbols::{border, merge::MergeStrategy},
    text::{Line, Text},
    widgets::{Block, Borders, StatefulWidget, Widget},
};
use serde::{Deserialize, Serialize};
use tui_input::{Input, InputRequest};

use crate::{AppAction, Context, View, ViewType, widgets::Popup};

#[derive(Debug, Clone)]
pub struct PickerItem {
    title: String,
}

impl PickerItem {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl From<String> for PickerItem {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for PickerItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug)]
pub struct PickerView {
    #[allow(unused)]
    context: Context,

    input: Input,
    items: Vec<PickerItem>,
    selection: usize,
}

impl PickerView {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            input: Input::default(),
            items: vec![
                "lorem".into(),
                "ipsum".into(),
                "constructeris".into(),
                "sit".into(),
            ],
            selection: 0,
        }
    }

    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    pub fn selection_down(&mut self) {
        self.selection = self.selection.saturating_add(1).min(self.items_len());
    }

    pub fn selection_up(&mut self) {
        self.selection = self.selection.saturating_sub(1);
    }
}

pub struct Picker;

impl StatefulWidget for Picker {
    type State = PickerView;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let width = 70;
        let height = 20;

        let popup = Popup::new(Size { width, height }).borders(Borders::empty());
        let popup_inner = popup.inner(area);

        let [input_block_area, item_area] = popup_inner.layout(
            &Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)])
                .spacing(Spacing::Overlap(1)),
        );

        let mut item_list: Vec<Line> = Vec::new();

        for (i, item) in state.items.iter().enumerate() {
            let style = if state.selection == i {
                Style::new().bg(White).fg(Black)
            } else {
                Style::default()
            };

            let mut title = item.title.clone();
            title.insert(0, ' ');

            item_list.push(Line::raw(title).style(style));
        }

        // let mut items_list = Text::default();
        // items_list.lines = items;

        popup.render(area, buf);

        let border_block = Block::new()
            .borders(Borders::all())
            .border_set(border::ROUNDED)
            .merge_borders(MergeStrategy::Fuzzy);

        let input_area = input_block_area.inner(Margin::new(2, 1));
        let cursor_position = input_area
            .as_position()
            .offset(Offset::new(state.input.visual_cursor() as i32, 0));

        border_block.clone().render(input_block_area, buf);
        Text::raw(state.input.value()).render(input_area, buf);

        if let Some(cursor_cell) = buf.cell_mut(cursor_position) {
            cursor_cell.modifier = cursor_cell.modifier.union(Modifier::REVERSED);
        }

        border_block.clone().render(item_area, buf);
        Text::from(item_list).render(border_block.inner(item_area), buf);
    }
}

#[derive(Debug, Clone, strum::EnumString, Serialize, Deserialize)]
pub enum PickerAction {
    SelectionUp,
    SelectionDown,

    Select,

    Insert(String),
    CursorRight,
    CursorLeft,
    DeletePrevChar,
    DeleteNextChar,

    DeleteTillStart,
}

impl Action for PickerAction {}

impl State for PickerView {
    type Action = PickerAction;
    type Error = Infallible;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert<Self>, Self::Error>
    where
        Self: Sized,
    {
        use PickerAction::*;
        match action.into() {
            SelectionDown => self.selection_down(),
            SelectionUp => self.selection_up(),

            Insert(input) => {
                for char in input.chars() {
                    self.input.handle(InputRequest::InsertChar(char));
                }
            }
            DeleteNextChar => {
                self.input.handle(InputRequest::DeleteNextChar);
            }
            DeletePrevChar => {
                self.input.handle(InputRequest::DeletePrevChar);
            }

            DeleteTillStart => {
                self.input.handle(InputRequest::DeleteLine);
            }

            CursorLeft => {
                self.input.handle(InputRequest::GoToPrevChar);
            }
            CursorRight => {
                self.input.handle(InputRequest::GoToNextChar);
            }

            Select => {}
        };

        Ok(Revert::None)
    }
}

impl View for PickerView {
    fn title() -> String {
        String::from("Picker")
    }

    fn view_type() -> ViewType {
        ViewType::Popup
    }

    fn input_sink_action(input: String) -> Option<AppAction> {
        Some(AppAction::PickerAction(PickerAction::Insert(input)))
    }
}
