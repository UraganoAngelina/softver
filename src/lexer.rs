use std::fmt::Debug;


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
    // Numeri
    Number(i64),

    // Variabili (identificatori)
    Identifier(String),

    // Operatori
    Plus,       // '+'
    Minus,      // '-'
    Multiply,   // '*'
    Divide,     // '/'
    Assign,     // ':='
    LessEqual,  // '<='
    Less,       // '<'
    Greater,    // '>'
    GreatEqual, // '>='
    Equal,      // '='
    And,        // '&&' 'and'
    Or,         // '||' 'or'
    Not,        // '!'
    PlusPlus,   // '++'

    // Parole chiave
    If,
    Then,
    Else,
    While,
    Repeat,
    Until,
    For,
    Skip,
    True,
    False,

    // Simboli
    Bra,       // '('
    Ket,       // ')'
    CBra,      // '{'
    Cket,      // '}'
    Semicolon, // ';'
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_ty: TokenType,
}

impl Token {
    pub fn new(value: String, token: TokenType) -> Self {
        Token { value, token_ty: token }
    }
}

pub struct Lexer {
    input: Vec<char>, // Input trattato come una sequenza di caratteri
    pos: usize,       // Posizione corrente nell'input
}

impl Lexer {
    // Inizializza il lexer con il programma come stringa
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    // Controlla se siamo alla fine dell'input
    fn is_at_end(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Restituisce il carattere corrente senza avanzare
    fn current_char(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }

    // Avanza di un carattere
    fn advance(&mut self) {
        self.pos += 1;
    }

    // Skippa gli spazi bianchi
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Estrai il prossimo token
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(current) = self.current_char() {
            let curr_token = match current {
                // Operatori singoli e doppi
                '+' => {
                    self.advance();
                    if let Some('+') = self.current_char() {
                        self.advance();
                        Token::new("++".to_string(), TokenType::PlusPlus)
                    } else {
                        Token::new("+".to_string(), TokenType::Plus)
                    }
                }
                '-' => {
                    self.advance();
                    Token::new("-".to_string(), TokenType::Minus)
                }
                '*' => {
                    self.advance();
                    Token::new("*".to_string(), TokenType::Multiply)
                }
                '/' => {
                    self.advance();
                    Token::new("/".to_string(), TokenType::Divide)
                }
                ':' => {
                    self.advance();
                    if let Some('=') = self.current_char() {
                        self.advance();
                        Token::new(":=".to_string(), TokenType::Assign)
                    } else {
                        return None; // Errore sintattico
                    }
                }
                '<' => {
                    self.advance();
                    if let Some('=') = self.current_char() {
                        self.advance();
                        Token::new("<=".to_string(), TokenType::LessEqual)
                    } else {
                        Token::new("<".to_string(), TokenType::Less)
                    }
                }
                '>' => {
                    self.advance();
                    if let Some('=') = self.current_char() {
                        self.advance();
                        Token::new(">=".to_string(), TokenType::GreatEqual)
                    } else {
                        Token::new(">".to_string(), TokenType::Greater)
                    }
                }
                '=' => {
                    self.advance();
                    Token::new("=".to_string(), TokenType::Equal)
                }
                '&' => {
                    self.advance();
                    if let Some('&') = self.current_char() {
                        self.advance();
                        Token::new("&&".to_string(), TokenType::And)
                    } else {
                        return None; // Errore sintattico
                    }
                }
                '|' => {
                    self.advance();
                    if let Some('|') = self.current_char() {
                        self.advance();
                        Token::new("||".to_string(), TokenType::Or)
                    } else {
                        return None; // Errore sintattico
                    }
                }
                '!' => {
                    self.advance();
                    Token::new("!".to_string(), TokenType::Not)
                }

                // Simboli
                '(' => {
                    self.advance();
                    Token::new("(".to_string(), TokenType::Bra)
                }
                ')' => {
                    self.advance();
                    Token::new(")".to_string(), TokenType::Ket)
                }
                '{' => {
                    self.advance();
                    Token::new("{".to_string(), TokenType::CBra)
                }
                '}' => {
                    self.advance();
                    Token::new("}".to_string(), TokenType::Cket)
                }
                ';' => {
                    self.advance();
                    Token::new(";".to_string(), TokenType::Semicolon)
                }

                // Identificatori o parole chiave (analisi per stringa completa)
                _ if current.is_alphabetic() => {
                    let identifier = self.consume_identifier();
                    match identifier.as_str() {
                        "if" => Token::new(identifier.clone(), TokenType::If),
                        "then" => Token::new(identifier.clone(), TokenType::Then),
                        "else" => Token::new(identifier.clone(), TokenType::Else),
                        "while" => Token::new(identifier.clone(), TokenType::While),
                        "repeat" => Token::new(identifier.clone(), TokenType::Repeat),
                        "until" => Token::new(identifier.clone(), TokenType::Until),
                        "for" => Token::new(identifier.clone(), TokenType::For),
                        "skip" => Token::new(identifier.clone(), TokenType::Skip),
                        "true" => Token::new(identifier.clone(), TokenType::True),
                        "false" => Token::new(identifier.clone(), TokenType::False),
                        _ => Token::new(identifier.clone(), TokenType::Identifier(identifier)),
                    }
                }

                // Numeri
                _ if current.is_digit(10) => {
                    let number = self.consume_number();
                    Token::new(number.to_string(), TokenType::Number(number))
                }

                _ => return None, // Carattere non riconosciuto
            };

            Some(curr_token)
        } else {
            None // Fine dell'input
        }
    }

    // Consuma numeri
    fn consume_number(&mut self) -> i64 {
        let mut number_str = String::new();
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                number_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        number_str.parse::<i64>().unwrap()
    }

    // Consuma identificatori o parole chiave
    fn consume_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphabetic() {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    // Tokenizza l'input completo
    pub fn tokenize(input: String) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        tokens
    }
}
