use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Default)]
pub struct Cell(String);

pub struct TooBig;

impl Cell {
    /// Obtain a `Cell` given a string
    ///
    /// # Errors
    /// Return `TooBig` if the string is more than 3 graphems long
    /// To trim automatically the string to always fit, use [`Cell::new_trim`]
    pub fn new(string: &str) -> Result<Self, TooBig> {
        if string.graphemes(true).count() <= 3 {
            Ok(Self(string.to_string()))
        } else {
            Err(TooBig)
        }
    }

    /// Obtain a `Cell` given a string, trimming any excess (more than 3 graphems)
    pub fn new_trim(string: &str) -> Self {
        Self(string.graphemes(true).take(3).collect::<String>())
    }

    /// Remove all content of the `Cell`
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Return the number of graphems present in the `Cell`
    pub fn len(&self) -> usize {
        self.0.graphemes(true).count()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn content(&self) -> String {
        self.0.clone()
    }
}

impl From<Cell> for String {
    fn from(value: Cell) -> Self {
        value.content()
    }
}
