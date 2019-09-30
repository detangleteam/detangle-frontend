pub struct Model {
    pub columns: Vec<String>,
    pub column_visibility: Vec<bool>,
    pub filters: Vec<(Option<usize>, String)>,
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
