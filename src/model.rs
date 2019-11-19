pub struct Model {
    pub columns: Vec<String>,
    pub column_visibility: Vec<ColumnVisibility>,
    pub filters: Vec<Filter>,
    pub sort_columns: Vec<Option<usize>>,
    pub items: Vec<Vec<String>>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            column_visibility: Vec::new(),
            filters: Vec::new(),
            sort_columns: Vec::new(),
            items: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ColumnVisibility {
    Auto,
    Hidden,
    Shown
}

#[derive(Clone)]
pub struct Filter {
    pub boolean_op: BooleanOp,
    pub column: Option<usize>,
    pub value: String
}

#[derive(Clone, Copy, PartialEq)]
pub enum BooleanOp {
    And,
    AndNot,
    Or,
    OrNot
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            boolean_op: BooleanOp::And,
            column: Option::None,
            value: String::new()
        }
    }
}