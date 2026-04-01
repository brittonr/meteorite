/// Sort direction for a table column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Column definition for a data table.
#[derive(Clone, PartialEq)]
pub struct Column {
    pub key: String,
    pub label: String,
    pub sortable: bool,
    pub width: Option<String>,
}

impl Column {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Column {
            key: key.into(),
            label: label.into(),
            sortable: false,
            width: None,
        }
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub fn width(mut self, w: impl Into<String>) -> Self {
        self.width = Some(w.into());
        self
    }
}
