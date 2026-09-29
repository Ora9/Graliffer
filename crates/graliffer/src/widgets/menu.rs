use ratatui::{
    style::{Style, Stylize},
    symbols,
    text::{Span, ToSpan},
};

use crate::{
    AppAction, Context, ViewId,
    input::{Key, Keystroke, Modifiers},
};

#[derive(Debug, Clone, Default)]
pub enum MenuLinePosition {
    #[default]
    Top,
    Bottom,
}

#[derive(Debug, Clone, Default)]
pub enum MenuLineAlignement {
    #[default]
    Left,
    Center,
    Right,
}

/// A MenuLine is a list of [`MenuGroup`], it has a [`MenuLinePosition`] and a [`MenuLineAlignement`]
#[derive(Debug, Clone, Default)]
pub struct MenuLine {
    pub groups: Vec<MenuGroup>,
    pub position: MenuLinePosition,
    pub alignement: MenuLineAlignement,
}

impl<'a> MenuLine {
    pub fn from_title(title: MenuTitle) -> Self {
        Self {
            groups: vec![MenuGroup::from_title(title)],
            ..Default::default()
        }
    }

    pub fn from_group(group: MenuGroup) -> Self {
        Self {
            groups: vec![group],
            ..Default::default()
        }
    }

    pub fn push_title(mut self, title: MenuTitle) -> Self {
        if let Some(last) = self.groups.last_mut() {
            *last = last.clone().push_title(title);
            self
        } else {
            self.push_title_in_new_group(title)
        }
    }

    pub fn push_title_in_new_group(self, title: MenuTitle) -> Self {
        self.push_group(MenuGroup::from_title(title))
    }

    pub fn push_group(mut self, group: MenuGroup) -> Self {
        self.groups.push(group);
        self
    }

    pub fn as_border(&'a self) -> Vec<Span<'a>> {
        self.groups.iter().fold(Vec::new(), |mut spans, groups| {
            if !spans.is_empty() {
                spans.push(Span::raw(symbols::line::HORIZONTAL));
            }

            spans.extend(groups.as_border());
            spans
        })
    }

    pub fn top(mut self) -> Self {
        self.position = MenuLinePosition::Top;
        self
    }

    pub fn bottom(mut self) -> Self {
        self.position = MenuLinePosition::Bottom;
        self
    }

    pub fn center(mut self) -> Self {
        self.alignement = MenuLineAlignement::Center;
        self
    }

    pub fn left(mut self) -> Self {
        self.alignement = MenuLineAlignement::Left;
        self
    }
    pub fn right(mut self) -> Self {
        self.alignement = MenuLineAlignement::Right;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct MenuGroup {
    titles: Vec<MenuTitle>,
}

impl<'a> MenuGroup {
    pub fn from_title(title: MenuTitle) -> Self {
        Self {
            titles: vec![title],
        }
    }

    pub fn push_title(mut self, title: MenuTitle) -> Self {
        self.titles.push(title);
        self
    }

    pub fn as_border(&'a self) -> Vec<Span<'a>> {
        self.titles.iter().fold(Vec::new(), |mut spans, title| {
            spans.extend(title.as_border());
            spans
        })
    }
}

#[derive(Debug, Clone)]
pub enum MenuTitle {
    Info {
        title: String,
        style: Style,

        highlighted: bool,
    },
    NumberPrefix {
        title: String,
        style: Style,

        highlighted: bool,

        prefix: NumberPrefix,
    },
    Inline {
        title: String,
        style: Style,

        highlighted: bool,

        highlight_char: String,
    },
}

impl<'a> MenuTitle {
    pub fn from_pane_title(view_id: ViewId, action: AppAction, context: &Context) -> Self {
        let focused = context.focus() == view_id;
        Self::from_context(view_id.to_string(), context, action, focused)
    }

    pub fn from_context(
        title: String,
        context: &Context,
        action: AppAction,
        highlighted: bool,
    ) -> Self {
        let keystroke =
            context.keys(|keymap, key_context| keymap.find_keystroke(action.into(), key_context));

        let style = Style::new();

        match keystroke {
            Some(Keystroke {
                modifiers: Modifiers::NONE,
                key: Key::Char(char),
            }) => {
                if let Some(prefix) = NumberPrefix::from_str(&char.to_string()) {
                    MenuTitle::NumberPrefix {
                        title,
                        style,
                        highlighted,
                        prefix,
                    }
                } else {
                    unimplemented!()
                    // MenuTitle::Inline { title, style, highlighted, highlight_char: () }
                }
            }
            _ => MenuTitle::Info {
                title,
                style,
                highlighted,
            },
        }
    }

    pub fn formated(&'a self) -> Vec<Span<'a>> {
        match self {
            Self::Info {
                title,
                style,
                highlighted,
            } => {
                let mut title = title.to_span().style(*style);

                if *highlighted {
                    title = title.bold();
                }

                vec![title]
            }
            Self::NumberPrefix {
                title,
                style,
                prefix,
                highlighted,
            } => {
                let prefix = prefix.superscript().blue();
                let mut title = title.to_span().style(*style);

                if *highlighted {
                    title = title.bold();
                }

                vec![prefix, title]
            }
            Self::Inline {
                title,
                style,
                highlight_char,
                highlighted,
            } => {
                let mut split = title.splitn(2, highlight_char);
                let start = split.next().unwrap_or("");
                let highlight = highlight_char.to_span().blue();
                let end = split.next().unwrap_or("");

                let mut spans = vec![start.to_owned().into(), highlight, end.to_owned().into()];

                for span in spans.iter_mut() {
                    span.style = *style;

                    if *highlighted {
                        span.style = span.style.bold().blue();
                    }
                }

                spans
            }
        }
    }

    pub fn as_border(&'a self) -> Vec<Span<'a>> {
        let mut spans = self.formated();
        spans.insert(0, symbols::line::VERTICAL_LEFT.into());
        spans.push(symbols::line::VERTICAL_RIGHT.into());

        spans
    }
}

#[derive(Debug, Clone)]
pub enum NumberPrefix {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

impl NumberPrefix {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::from_number(s.parse().ok()?)
    }

    pub fn from_number(number: u32) -> Option<Self> {
        use NumberPrefix::*;

        match number {
            0 => Some(Num0),
            1 => Some(Num1),
            2 => Some(Num2),
            3 => Some(Num3),
            4 => Some(Num4),
            5 => Some(Num5),
            6 => Some(Num6),
            7 => Some(Num7),
            8 => Some(Num8),
            9 => Some(Num9),
            _ => None,
        }
    }

    pub fn superscript(&self) -> String {
        use NumberPrefix::*;

        match self {
            Num0 => "⁰",
            Num1 => "¹",
            Num2 => "²",
            Num3 => "³",
            Num4 => "⁴",
            Num5 => "⁵",
            Num6 => "⁶",
            Num7 => "⁷",
            Num8 => "⁸",
            Num9 => "⁹",
        }
        .to_string()
    }
}
