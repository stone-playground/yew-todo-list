#[derive(Debug, strum_macros::Display, strum_macros::EnumIter, PartialEq, Clone)]
pub enum Filter {
    All,
    Active,
    Completed,
}
