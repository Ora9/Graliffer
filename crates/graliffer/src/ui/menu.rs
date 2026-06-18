use log::debug;
use ratatui::{
    style::{Style, Stylize},
    symbols,
    text::{Line, Span, ToLine, ToSpan},
};

#[derive(Debug, Clone)]
pub struct MenuBar {
    groups: Vec<MenuGroup>,
}

impl MenuBar {
    pub fn empty() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn from_title(title: MenuTitle) -> Self {
        Self {
            groups: vec![MenuGroup::from_title(title)],
        }
    }

    pub fn push_group(mut self, group: MenuGroup) -> Self {
        self.groups.push(group);
        self
    }

    pub fn push_title(mut self, title: MenuTitle) -> Self {
        if let Some(mut last) = self.groups.last_mut() {
            *last = last.clone().push_title(title);
            self
        } else {
            self.push_title_in_new_group(title)
        }
    }

    pub fn push_title_in_new_group(mut self, title: MenuTitle) -> Self {
        self.push_group(MenuGroup::from_title(title))
    }

    pub fn as_border(self) -> Vec<Span<'static>> {
        self.groups
            .into_iter()
            .fold(Vec::new(), |mut spans, groups| {
                if !spans.is_empty() {
                    spans.push(Span::raw(symbols::line::HORIZONTAL));
                }

                spans.extend(groups.as_border());
                spans
            })
    }
}

#[derive(Debug, Clone)]
pub struct MenuGroup {
    titles: Vec<MenuTitle>,
}

impl MenuGroup {
    pub fn empty() -> Self {
        Self { titles: Vec::new() }
    }

    pub fn from_title(title: MenuTitle) -> Self {
        Self {
            titles: vec![title],
        }
    }

    pub fn push_title(mut self, title: MenuTitle) -> Self {
        self.titles.push(title);
        self
    }

    pub fn as_border(self) -> Vec<Span<'static>> {
        // let spans = Vec::new();

        self.titles
            .into_iter()
            .fold(Vec::new(), |mut spans, title| {
                spans.extend(title.as_border());
                spans
            })

        // vec![]
    }
}

#[derive(Debug, Clone)]
pub enum MenuTitle {
    NumberPrefix {
        title: String,
        prefix: NumberPrefix,
        focused: bool,
    },
    Inline {
        title: String,
        highlight: String,
        focused: bool,
    },
}

impl MenuTitle {
    pub fn formated(self) -> Vec<Span<'static>> {
        match self {
            Self::NumberPrefix {
                title,
                prefix,
                focused,
            } => {
                let prefix = prefix.superscript().blue();
                let title = if focused {
                    title.bold()
                } else {
                    Span::raw(title)
                };

                vec![prefix, title]
            }
            Self::Inline {
                title,
                highlight,
                focused,
            } => {
                let title = title.to_owned();

                let mut split = title.splitn(2, &highlight);
                let start = split.next().unwrap_or("");
                let highlight = highlight.to_owned().blue();
                let end = split.next().unwrap_or("");

                let spans = vec![
                    start.to_owned().into(),
                    highlight.to_owned(),
                    end.to_owned().into(),
                ];

                if focused {
                    spans.into_iter().map(|e| e.bold()).collect()
                } else {
                    spans
                }
            }
        }
    }

    pub fn as_border(self) -> Vec<Span<'static>> {
        let mut line = self.formated();
        line.insert(0, symbols::line::VERTICAL_LEFT.into());
        line.push(symbols::line::VERTICAL_RIGHT.into());
        line
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
    pub fn from(number: u32) -> Option<Self> {
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
