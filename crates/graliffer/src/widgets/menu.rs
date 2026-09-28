use ratatui::{
    style::{Style, Stylize},
    symbols,
    text::{Line, Span, ToSpan},
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

    pub fn as_border(&'a self) -> Line<'a> {
        let line = self
            .groups
            .iter()
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
    },
    NumberPrefix {
        title: String,
        style: Style,
        prefix: NumberPrefix,
        highlighted: bool,
    },
    Inline {
        title: String,
        style: Style,
        highlight_char: String,
        highlighted: bool,
    },
}

impl<'a> MenuTitle {
    pub fn formated(&'a self) -> Line<'a> {
        match self {
            Self::Info { title, style } => Line::raw(title).style(*style),
            Self::NumberPrefix {
                title,
                style,
                prefix,
                highlighted,
            } => {
                let prefix = prefix.superscript().blue();

                let title = if *highlighted {
                    title.to_span().bold().blue()
                } else {
                    title.to_span().clone()
                };

                Line::from(vec![prefix, title]).style(*style)
            }
            Self::Inline {
                title,
                style,
                highlight_char,
                highlighted,
            } => {
                let mut split = title.splitn(2, highlight_char);
                let start = split.next().unwrap_or("");
                let highlight = highlight_char.to_owned().blue();
                let end = split.next().unwrap_or("");

                let mut spans = vec![
                    start.to_owned().into(),
                    highlight.to_owned(),
                    end.to_owned().into(),
                ];

                if *highlighted {
                    spans = spans.into_iter().map(|e| e.bold().blue()).collect();
                }

                Line::from(spans)
            }
        }
    }

    pub fn as_border(&'a self) -> Line<'a> {
        let mut spans = self.formated().spans;

        spans.insert(0, symbols::line::VERTICAL_LEFT.into());
        spans.push(symbols::line::VERTICAL_RIGHT.into());
        Line::from(spans)
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
