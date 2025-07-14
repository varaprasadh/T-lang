#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    Sankhya(f64),
    Aksharalu(String),
    Nijam,
    Abaddham,
    Sunya,
    
    // Variables
    Variable(String),
    
    // Binary operations
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    
    // Unary operations
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
    
    // Function call
    Call {
        name: String,
        args: Vec<Expr>,
    },
    
    // Assignment
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    
    // Array literal
    Array(Vec<Expr>),
    
    // Array indexing
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    // Expression statement
    Expression(Expr),
    
    // Variable declaration
    VarDecl {
        var_type: String,
        name: String,
        initializer: Option<Expr>,
    },
    
    // If statement
    Ayite {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    
    // While loop
    Varaku {
        condition: Expr,
        body: Box<Stmt>,
    },
    
    // For loop
    Prathi {
        variable: String,
        iterable: Expr,
        body: Box<Stmt>,
    },
    
    // Block
    Block(Vec<Stmt>),
    
    // Function declaration
    Pani {
        name: String,
        params: Vec<(String, String)>, // (type, name)
        return_type: Option<String>,
        body: Box<Stmt>,
    },
    
    // Return statement
    TirigiPampu(Option<Expr>),
    
    // Break
    Aagu,
    
    // Continue
    Kalugu,
    
    // Print statement
    Cheppu(Expr),
    
    // Input statement
    Adugu {
        prompt: Option<String>,
        variable: String,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Mariyu,  // And
    Leda,    // Or
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
    Kadu,  // Not
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}