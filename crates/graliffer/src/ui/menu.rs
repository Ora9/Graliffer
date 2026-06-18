use log::debug;
use ratatui::{
    style::{Style, Stylize},
    symbols,
    text::{Line, Span, ToLine, ToSpan},
};

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub fn formated(&self) -> Vec<Span<'_>> {
        match self {
            Self::NumberPrefix {
                title,
                prefix,
                focused,
            } => {
                let prefix = prefix.superscript().blue();
                let title = if *focused {
                    title.to_span().bold()
                } else {
                    title.to_span()
                };

                vec![prefix, title]
            }
            Self::Inline {
                title,
                highlight,
                focused,
            } => {
                let mut split = title.splitn(2, highlight);
                let start = split.next().unwrap_or("");
                let highlight = highlight.to_owned().blue();
                let end = split.next().unwrap_or("");

                if *focused {
                    vec![start.into(), highlight, end.into()]
                        .into_iter()
                        .map(|e| e.bold())
                        .collect()
                } else {
                    vec![start.into(), highlight, end.into()]
                }
            }
        }
    }

    pub fn framed(&self) -> Vec<Span<'_>> {
        let mut line = self.formated();
        line.insert(0, symbols::line::VERTICAL_LEFT.into());
        line.push(symbols::line::VERTICAL_RIGHT.into());
        line
    }
}
