#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Sankhya(f64),      // Number
    Aksharalu(String), // String
    Nijam,             // True
    Abaddham,          // False
    Sunya,             // Null
    
    // Keywords
    Ayite,             // If
    Lekapothe,         // Else
    Leda,              // Or (for else if)
    Varaku,            // While
    Prathi,            // For/foreach
    Aagu,              // Break
    Kalugu,            // Continue
    Pani,              // Function
    TirigiPampu,       // Return
    Rakam,             // Type/class
    Kotha,             // New
    Sontham,           // Self/this
    Techu,             // Import
    Nunchi,            // From
    Penchu,            // Increment
    Bhagam,            // Module
    Prayatnam,         // Try
    Tappu,             // Catch/error
    Chivara,           // Finally
    Cheppu,            // Print
    Adugu,             // Input
    Pradhaanam,        // Main
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Mariyu,            // And
    LedaOp,            // Or operator
    Kadu,              // Not
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    
    // Identifiers
    Identifier(String),
    
    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token { token_type, lexeme, line, column }
    }
}