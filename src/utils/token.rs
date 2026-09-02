pub enum TokenType {
     //one or two character token
     EQUAL,

    // keywords
    VAR,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    //single char token
    SEMICOLON,

    //eof
    EOF
 

}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String, 
    line: u16
}

impl Token {
    pub fn new (token_type: TokenType, lexeme: String, literal: String, line: u16) -> Self {
        return Token {
            token_type: token_type,
            lexeme: lexeme,
            literal:literal,
            line: line
        }
    }
}