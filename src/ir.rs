#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub rust_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrItem {
    Comment(String),
    FunctionStart {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<String>,
    },
    IfStart(String),
    BlockEnd,
    Println(String),
    VarDecl {
        name: String,
        rust_type: String,
        value: Option<String>,
    },
    ShortVarDecl {
        name: String,
        value: String,
    },
    Assignment {
        name: String,
        value: String,
    },
    Return(String),
    Todo(String),
    Empty,
}
