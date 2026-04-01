/// Sort direction for a table column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn toggle(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Ascending => "↑",
            Self::Descending => "↓",
        }
    }
}

/// Active sort state: which column, which direction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortState {
    pub column_key: String,
    pub direction: SortDirection,
}

/// Column definition for a data table.
#[derive(Clone, PartialEq)]
pub struct Column {
    pub key: String,
    pub label: String,
    pub sortable: bool,
    pub resizable: bool,
    pub editable: bool,
    pub width: Option<String>,
    pub min_width: Option<String>,
}

impl Column {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Column {
            key: key.into(),
            label: label.into(),
            sortable: false,
            resizable: false,
            editable: false,
            width: None,
            min_width: None,
        }
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    pub fn editable(mut self) -> Self {
        self.editable = true;
        self
    }

    pub fn width(mut self, w: impl Into<String>) -> Self {
        self.width = Some(w.into());
        self
    }

    pub fn min_width(mut self, w: impl Into<String>) -> Self {
        self.min_width = Some(w.into());
        self
    }
}
