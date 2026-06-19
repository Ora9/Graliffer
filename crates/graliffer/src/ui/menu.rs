use log::debug;
use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    symbols,
    text::{Line, Span, ToLine, ToSpan},
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

#[derive(Debug, Clone, Default)]
pub struct MenuLine<'a> {
    pub groups: Vec<MenuGroup<'a>>,
    pub position: MenuLinePosition,
    pub alignement: MenuLineAlignement,
}

impl<'a> MenuLine<'a> {
    pub fn from_title(title: MenuTitle<'a>) -> Self {
        Self {
            groups: vec![MenuGroup::from_title(title)],
            ..Default::default()
        }
    }

    pub fn from_group(group: MenuGroup<'a>) -> Self {
        Self {
            groups: vec![group],
            ..Default::default()
        }
    }

    pub fn push_title(mut self, title: MenuTitle<'a>) -> Self {
        if let Some(mut last) = self.groups.last_mut() {
            *last = last.clone().push_title(title);
            self
        } else {
            self.push_title_in_new_group(title)
        }
    }

    pub fn push_title_in_new_group(mut self, title: MenuTitle<'a>) -> Self {
        self.push_group(MenuGroup::from_title(title))
    }

    pub fn push_group(mut self, group: MenuGroup<'a>) -> Self {
        self.groups.push(group);
        self
    }

    pub fn as_border(self) -> Line<'a> {
        let line = self
            .groups
            .into_iter()
            .fold(Line::default(), |mut line, groups| {
                if !line.spans.is_empty() {
                    line.spans.push(Span::raw(symbols::line::HORIZONTAL));
                }

                line.spans.extend(groups.as_border());
                line
            });

        match self.alignement {
            MenuLineAlignement::Left => line.left_aligned(),
            MenuLineAlignement::Center => line.centered(),
            MenuLineAlignement::Right => line.right_aligned(),
        }
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
pub struct MenuGroup<'a> {
    titles: Vec<MenuTitle<'a>>,
}

impl<'a> MenuGroup<'a> {
    pub fn from_title(title: MenuTitle<'a>) -> Self {
        Self {
            titles: vec![title],
        }
    }

    pub fn push_title(mut self, title: MenuTitle<'a>) -> Self {
        self.titles.push(title);
        self
    }

    pub fn as_border(self) -> Vec<Span<'a>> {
        self.titles
            .into_iter()
            .fold(Vec::new(), |mut spans, title| {
                spans.extend(title.as_border());
                spans
            })
    }
}

#[derive(Debug, Clone)]
pub enum MenuTitle<'a> {
    Info(Span<'a>),
    Inline {
        title: Span<'a>,
        highlight_char: String,
        focused: bool,
    },
    NumberPrefix {
        title: Span<'a>,
        prefix: NumberPrefix,
        focused: bool,
    },
}

impl<'a> MenuTitle<'a> {
    pub fn formated(self) -> Line<'a> {
        match self {
            Self::Info(title) => Line::from(title),
            Self::NumberPrefix {
                mut title,
                prefix,
                focused,
            } => {
                let prefix = prefix.superscript().blue();
                if focused {
                    title = title.bold()
                };

                Line::from(vec![prefix, title])
            }
            Self::Inline {
                title,
                highlight_char,
                focused,
            } => {
                let mut split = title.content.splitn(2, &highlight_char);
                let start = split.next().unwrap_or("");
                let highlight = highlight_char.to_owned().blue();
                let end = split.next().unwrap_or("");

                let mut spans = vec![
                    start.to_owned().into(),
                    highlight.to_owned(),
                    end.to_owned().into(),
                ];

                if focused {
                    spans = spans.into_iter().map(|e| e.bold()).collect();
                }

                Line::from(spans)
            }
        }
    }

    pub fn as_border(self) -> Line<'a> {
        let mut spans = self.formated().spans;

        spans.insert(0, symbols::line::VERTICAL_LEFT.into());
        spans.push(symbols::line::VERTICAL_RIGHT.into());
        Line::from(spans)
    }
}

#[derive(Debug, Clone, strum_macros::EnumString)]
pub enum NumberPrefix {
    #[strum(to_string = "0")]
    Num0,
    #[strum(to_string = "1")]
    Num1,
    #[strum(to_string = "2")]
    Num2,
    #[strum(to_string = "3")]
    Num3,
    #[strum(to_string = "4")]
    Num4,
    #[strum(to_string = "5")]
    Num5,
    #[strum(to_string = "6")]
    Num6,
    #[strum(to_string = "7")]
    Num7,
    #[strum(to_string = "8")]
    Num8,
    #[strum(to_string = "9")]
    Num9,
}

impl NumberPrefix {
    // pub fn from(number: u32) -> Option<Self> {
    //     use NumberPrefix::*;

    //     match number {
    //         0 => Some(Num0),
    //         1 => Some(Num1),
    //         2 => Some(Num2),
    //         3 => Some(Num3),
    //         4 => Some(Num4),
    //         5 => Some(Num5),
    //         6 => Some(Num6),
    //         7 => Some(Num7),
    //         8 => Some(Num8),
    //         9 => Some(Num9),
    //         _ => None,
    //     }
    // }

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
