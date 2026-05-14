#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrItem {
    Comment(String),
    FunctionStart {
        name: String,
    },
    BlockEnd,
    Println(String),
    VarDecl {
        name: String,
        rust_type: String,
        value: Option<String>,
    },
    Return(String),
    Todo(String),
    Empty,
}
