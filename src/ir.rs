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
    ElseIfStart(String),
    ElseStart,
    LoopStart,
    WhileStart(String),
    BlockEnd,
    Print(String),
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
    ExpressionStmt(String),
    Return(Option<String>),
    Todo(String),
    Empty,
}
