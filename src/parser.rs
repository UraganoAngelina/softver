use crate::ast::arithmetic::Add;
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::arithmetic::Minus;
use crate::ast::arithmetic::Numeral;
use crate::ast::arithmetic::Product;
use crate::ast::arithmetic::Uminus;
use crate::ast::arithmetic::Variable;
use crate::ast::boolean::And;
use crate::ast::boolean::Boolean;
use crate::ast::boolean::BooleanExpression;
use crate::ast::boolean::Equal;
use crate::ast::boolean::Great;
use crate::ast::boolean::GreatEqual;
use crate::ast::boolean::Less;
use crate::ast::boolean::LessEqual;
use crate::ast::boolean::Not;
use crate::ast::boolean::Or;
use crate::ast::statement::Assign;
use crate::ast::statement::Concat;
use crate::ast::statement::For;
use crate::ast::statement::IfThenElse;
use crate::ast::statement::RepeatUntil;
use crate::ast::statement::Skip;
use crate::ast::statement::Statement;
use crate::ast::statement::While;
use crate::ast::State;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;

use std::fmt;
use std::fmt::{Display, Formatter};

pub struct TokenVec {
    tokens: Vec<Token>,
}

impl Display for TokenVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for token in &self.tokens {
            // Usa un match per ogni tipo di token
            let token_str = match &token.token_ty {
                TokenType::Number(n) => format!("Number({})", n),
                TokenType::Identifier(id) => format!("Identifier({})", id),
                TokenType::Plus => "Plus".to_string(),
                TokenType::Minus => "Minus".to_string(),
                TokenType::Multiply => "Multiply".to_string(),
                TokenType::Divide => "Divide".to_string(),
                TokenType::Assign => "Assign(:=)".to_string(),
                TokenType::LessEqual => "LessEqual(<=)".to_string(),
                TokenType::Less => "Less(<)".to_string(),
                TokenType::Greater => "Greater(>)".to_string(),
                TokenType::GreatEqual => "GreatEqual(>=)".to_string(),
                TokenType::Equal => "Equal(=)".to_string(),
                TokenType::And => "And(&&)".to_string(),
                TokenType::Or => "Or(||)".to_string(),
                TokenType::Not => "Not(!)".to_string(),
                TokenType::PlusPlus => "PlusPlus(++)".to_string(),
                TokenType::If => "If".to_string(),
                TokenType::Then => "Then".to_string(),
                TokenType::Else => "Else".to_string(),
                TokenType::While => "While".to_string(),
                TokenType::Repeat => "Repeat".to_string(),
                TokenType::Until => "Until".to_string(),
                TokenType::For => "For".to_string(),
                TokenType::Skip => "Skip".to_string(),
                TokenType::True => "True".to_string(),
                TokenType::False => "False".to_string(),
                TokenType::Bra => "Bra (".to_string(),
                TokenType::Ket => "Ket )".to_string(),
                TokenType::CBra => "CBra {".to_string(),
                TokenType::Cket => "Cket }".to_string(),
                TokenType::Semicolon => "Semicolon ; ".to_string(),
            };

            // Scrive il token corrente nel formatter
            writeln!(f, "{}", token_str)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Any {
    BooleanExpression(Box<dyn BooleanExpression>), // Memorizzo i tipi che implementano BooleanExpression
    ArithmeticExpression(Box<dyn ArithmeticExpression>), // Memorizzo i tipi che implementano ArithmeticExpression
    Statement(Box<dyn Statement>), // Memorizzo i tipi che implementano Statement
    Token(Token),                  // Memorizzo i token direttamente
}

impl Any {
    // Funzione per creare un Any da una BooleanExpression
    pub fn from_boolean_expr(expr: Box<dyn BooleanExpression>) -> Self {
        Any::BooleanExpression(expr)
    }

    // Funzione per creare un Any da una ArithmeticExpression
    pub fn from_arithmetic_expr(expr: Box<dyn ArithmeticExpression>) -> Self {
        Any::ArithmeticExpression(expr)
    }

    // Funzione per creare un Any da uno Statement
    pub fn from_statement(stmt: Box<dyn Statement>) -> Self {
        Any::Statement(stmt)
    }

    // Funzione per creare un Any da un Token
    pub fn from_token(token: Token) -> Self {
        Any::Token(token)
    }

    // Funzione per ottenere un riferimento a BooleanExpression (se presente)
    pub fn as_boolean_expr(&self) -> Option<&Box<dyn BooleanExpression>> {
        if let Any::BooleanExpression(expr) = self {
            Some(expr)
        } else {
            None
        }
    }

    // Funzione per ottenere un riferimento a ArithmeticExpression (se presente)
    pub fn as_arithmetic_expr(&self) -> Option<&Box<dyn ArithmeticExpression>> {
        if let Any::ArithmeticExpression(expr) = self {
            Some(expr)
        } else {
            None
        }
    }

    // Funzione per ottenere un riferimento a Statement (se presente)
    pub fn as_statement(&self) -> Option<&Box<dyn Statement>> {
        if let Any::Statement(stmt) = self {
            Some(stmt)
        } else {
            None
        }
    }

    // Funzione per ottenere un riferimento a Token (se presente)
    pub fn as_token(&self) -> Option<&Token> {
        if let Any::Token(token) = self {
            Some(token)
        } else {
            None
        }
    }
    //Funzione per ritornare un riferimento ad Any
    pub fn as_any(&self) -> &Self {
        self
    }

    pub fn as_assign(&self) -> Option<&Assign> {
        if let Any::Statement(stmt) = self {
            // Tenta di ottenere un riferimento a `Assign` usando `as_any` e `downcast_ref`
            stmt.as_any().downcast_ref::<Assign>()
        } else {
            None
        }
    }
}
pub struct AnyVec {
    nodes: Vec<Any>,
}

impl AnyVec {
    pub fn push_boolean_expr(&mut self, expr: Box<dyn BooleanExpression>) {
        self.nodes.push(Any::from_boolean_expr(expr));
    }

    pub fn push_arithmetic_expr(&mut self, expr: Box<dyn ArithmeticExpression>) {
        self.nodes.push(Any::from_arithmetic_expr(expr));
    }

    pub fn push_statement(&mut self, stmt: Box<dyn Statement>) {
        self.nodes.push(Any::from_statement(stmt));
    }

    pub fn push_token(&mut self, token: Token) {
        self.nodes.push(Any::from_token(token));
    }
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

impl Display for AnyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.nodes {
            match node {
                Any::BooleanExpression(expr) => {
                    writeln!(f, "Boolean Expression: {:?}", expr)?;
                }
                Any::ArithmeticExpression(expr) => {
                    writeln!(f, "Arithmetic Expression: {:?}", expr)?;
                }
                Any::Statement(stmt) => {
                    writeln!(f, "Statement: {:?}", stmt)?;
                }
                Any::Token(token) => {
                    writeln!(f, "Token: {:?}", token)?;
                }
            }
        }
        Ok(())
    }
}

pub fn parse_lit(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        unreachable!("parse_lit:: Errore di parsing: indice fuori limite");
    }

    // Controlla che il nodo corrente sia un Token
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match token.token_ty {
            TokenType::Number(value) => {
                // Crea un Numeral e sostituisci il Token con un ArithmeticExpression
                let numeral = Numeral(value);
                let arithmetic_expr = Any::from_arithmetic_expr(Box::new(numeral));

                // Sostituisce il token corrente con l'espressione aritmetica
                tok_vec.nodes[*index] = arithmetic_expr;
            }
            _ => {}
        },
        _ => {
            unreachable!("parse_lit:: Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_bool_value(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        unreachable!("parse_lit:: Errore di parsing: indice fuori limite");
    }
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match token.token_ty {
            TokenType::True => {
                let booleanv = Boolean(true);

                let bool_expr = Any::from_boolean_expr(Box::new(booleanv));

                // Sostituisce il token corrente con l'espressione booleana
                tok_vec.nodes[*index] = bool_expr;
            }
            TokenType::False => {
                let booleanv = Boolean(false);

                let bool_expr = Any::from_boolean_expr(Box::new(booleanv));

                // Sostituisce il token corrente con l'espressione booleana
                tok_vec.nodes[*index] = bool_expr;
            }
            _ => {}
        },
        _ => {
            unreachable!("parse_bool_value:: Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_var(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        unreachable!("parse_var:: Errore di parsing: indice fuori limite");
    }

    // Controlla che il nodo corrente sia un Token
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match &token.token_ty {
            TokenType::Identifier(ref id) => {
                // Crea una Variable e sostituisci il Token con un ArithmeticExpression
                let var = Variable {
                    value: id.to_string(),
                };
                let arithmetic_expr = Any::from_arithmetic_expr(Box::new(var));

                // Sostituisce il token corrente con l'espressione aritmetica
                tok_vec.nodes[*index] = arithmetic_expr;
            }
            _ => {}
        },
        _ => {
            unreachable!("parse_var:: Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_skip(tok_vec: &mut AnyVec, index: &mut usize) {
    if *index >= tok_vec.nodes.len() {
        unreachable!(" parse_skip:: Errore di parsing: indice fuori limite");
    }

    // Controlla che il nodo corrente sia un Token
    match &tok_vec.nodes[*index] {
        Any::Token(token) => match token.token_ty {
            TokenType::Skip => {
                // Creiamo un'espressione o dichiarazione Skip
                let skip_stmt = Skip;
                let statement_expr = Any::from_statement(Box::new(skip_stmt));

                // Sostituisce il token corrente con la dichiarazione di skip
                tok_vec.nodes[*index] = statement_expr;
            }
            _ => {}
        },
        _ => {
            unreachable!("parse_skip::  Errore di parsing: il nodo corrente non è un token");
        }
    }
}

pub fn parse_atomic(tok_vec: &mut AnyVec, index: &mut usize) {
    while *index < tok_vec.nodes.len() {
        // Controlla se il nodo attuale è un token
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Number(_) => {
                    parse_lit(tok_vec, index); // Chiama parse_lit per i numeri
                }
                TokenType::Identifier(_) => {
                    parse_var(tok_vec, index); // Chiama parse_var per le variabili
                }
                TokenType::Skip => {
                    parse_skip(tok_vec, index); // Chiama parse_skip per il token Skip
                }
                TokenType::True | TokenType::False => {
                    parse_bool_value(tok_vec, index);
                }
                _ => {}
            }
        }
        // Incrementa l'indice per passare al prossimo token
        *index += 1;
    }
}

pub fn parse_arithmetic_subexpression(
    tok_vec: &mut AnyVec,
    index: &mut usize,
) -> Box<dyn ArithmeticExpression> {
    // Incrementa l'indice per saltare la parentesi aperta
    *index += 1;

    let start = *index;
    let mut depth = 1; // Traccia la profondità delle parentesi

    // Cerca la parentesi chiusa corrispondente
    while *index < tok_vec.nodes.len() {
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Bra => depth += 1, // Nuova parentesi aperta, aumenta la profondità
                TokenType::Ket => {
                    depth -= 1; // Parentesi chiusa, diminuisci la profondità
                    if depth == 0 {
                        break; // Trovata la parentesi chiusa corrispondente
                    }
                }
                _ => {}
            }
        }
        *index += 1;
    }

    if depth != 0 {
        unreachable!("Errore di parsing: parentesi chiusa mancante.");
    }
    let num_removed = *index - start;
    // Parsiamo la sottoespressione tra start e index-1
    let sub_tok_vec = tok_vec.nodes.drain(start..*index).collect::<Vec<Any>>();

    // Aggiorna l'indice principale in base alla nuova lunghezza di tok_vec
    // Sottrai il numero di elementi drenati (index - start) per correggere l'indice
    *index -= num_removed;

    // Creo il vettore Any contenente solo la sottoespressione da parsare
    let mut sub_any_vec = AnyVec { nodes: sub_tok_vec };
    // let mut k = 0;
    // println!("vector arithmetic subexpression: ");
    // while k < sub_any_vec.nodes.len() {
    //     println!("{:?}", sub_any_vec.nodes[k]);
    //     k += 1;
    // }
    // Richiama il parsing della sottoespressione
    let mut sub_index = 0; // Indice locale per la sottoespressione
    parse_arithmetic_expression(&mut sub_any_vec, &mut sub_index);

    // Controlla il risultato del parsing
    if let Some(Any::ArithmeticExpression(expr)) = sub_any_vec.nodes.pop() {
        //println!("parsed subexpression {:?}", expr);
        expr // Ritorna l'espressione parsata
    } else {
        unreachable!("Errore di parsing: expected ArithmeticExpression in sottoespressione.");
    }
}

pub fn parse_bool_subexpression(
    tok_vec: &mut AnyVec,
    index: &mut usize,
) -> Box<dyn BooleanExpression> {
    // Incrementa l'indice per saltare la parentesi aperta
    *index += 1;

    let start = *index;
    let mut depth = 1; // Traccia la profondità delle parentesi

    // Cerca la parentesi chiusa corrispondente
    while *index < tok_vec.nodes.len() {
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Bra => depth += 1, // Nuova parentesi aperta, aumenta la profondità
                TokenType::Ket => {
                    depth -= 1; // Parentesi chiusa, diminuisci la profondità
                    if depth == 0 {
                        break; // Trovata la parentesi chiusa corrispondente
                    }
                }
                _ => {}
            }
        }
        *index += 1;
    }

    if depth != 0 {
        unreachable!("Errore di parsing: parentesi chiusa mancante.");
    }
    let num_removed = *index - start;
    // Parsiamo la sottoespressione tra start e index-1
    let mut sub_tok_vec = tok_vec.nodes.drain(start..*index).collect::<Vec<Any>>();

    // Aggiorna l'indice principale in base alla nuova lunghezza di tok_vec
    // Sottrai il numero di elementi drenati (index - start) per correggere l'indice
    *index -= num_removed;

    // Creo il vettore Any contenente solo la sottoespressione da parsare
    let mut sub_any_vec = AnyVec { nodes: sub_tok_vec };
    // let mut k = 0;
    //println!("vector bool subexpression: ");
    // while k < sub_any_vec.nodes.len() {
    //     println!("{:?}", sub_any_vec.nodes[k]);
    //     k += 1;
    // }

    // Richiama il parsing della sottoespressione
    let mut sub_index = 0; // Indice locale per la sottoespressione
    parse_bool_expression(&mut sub_any_vec, &mut sub_index);

    // Controlla il risultato del parsing
    if let Some(Any::BooleanExpression(expr)) = sub_any_vec.nodes.pop() {
        //println!("parsed subexpression {:?}", expr);
        expr // Ritorna l'espressione parsata
    } else {
        unreachable!("Errore di parsing: expected ArithmeticExpression in sottoespressione.");
    }
}

pub fn parse_bool_expression(tok_vec: &mut AnyVec, index: &mut usize) {
    //println!("index:= {}", index);
    while *index < tok_vec.nodes.len() {
        // Controlla se il nodo attuale è un token
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                //TODO RICERCA UNARY OPERATOR
                TokenType::And => {
                    // Prima dell' and si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!("Errore di parsing: operando sinistro mancante per l'and.");
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::BooleanExpression(expr) => expr,
                        _ => unreachable!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra dell'and."
                        ),
                    };

                    // Dopo l'and, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!("Errore di parsing: operando destro mancante per l'and.");
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_bool_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_bool_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'and."),
                                    }
                                }
                            }
                            Any::BooleanExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'and."),
                                    }
                            }
                            _ => unreachable!(
                                "Errore di parsing: nodo non riconosciuto a destra dell'and."
                            ),
                        }
                    } else {
                        unreachable!("Errore di parsing: nessun token trovato a destra dell'and.");
                    };

                    //println!("printing the token vector after all");
                    //let mut j = 0;
                    // while j < tok_vec.nodes.len()
                    // {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j=j+1;
                    // }
                    // Crea l'oggetto And con left e right
                    let and_expr = And { left, right };

                    // Reinserisci l'oggetto And nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(and_expr)));

                    //elimino il token contenente l'operatore and
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Or => {
                    // Prima del or si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!("Errore di parsing: operando sinistro mancante per l'or.");
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::BooleanExpression(expr) => expr,
                        _ => unreachable!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra dell'or."
                        ),
                    };

                    // Dopo l'or, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!("Errore di parsing: operando destro mancante per l'or.");
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_bool_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'or."),
                                    }
                                }
                            }
                            Any::BooleanExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'or."),
                                    }
                            }
                            _ => {
                                unreachable!(
                                    "Errore di parsing: nodo non riconosciuto a destra dell'or."
                                )
                            }
                        }
                    } else {
                        unreachable!("Errore di parsing: nessun token trovato a destra dell'or.");
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len()
                    // {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j=j+1;
                    // }
                    // Crea l'oggetto Add con left e right
                    let or_expr = Or { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(or_expr)));

                    //elimino il token contenente l'operatore +
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Equal => {
                    // Prima del = si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano =."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano =."),
                    };

                    // Dopo l'=, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per l'op booleano =."
                        );
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano =."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano =."),
                                    }
                                },
                                _ => unreachable!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano =."),
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano =."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Equal con left e right
                    let eq_expr = Equal { left, right };

                    // Reinserisci l'oggetto Equal nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(eq_expr)));

                    //elimino il token contenente l'operatore =
                    tok_vec.nodes.remove(*index);
                }
                TokenType::LessEqual => {
                    // Prima del <= si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano <=."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano <=."),
                    };

                    // Dopo il <=, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per l'op booleano <=."
                        );
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <=."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <=."),
                                    }
                                },
                                _ => unreachable!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano <=."),
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano <=."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto LessEqual con left e right
                    let leq_expr = LessEqual { left, right };

                    // Reinserisci l'oggetto LessEqual nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(leq_expr)));

                    //elimino il token contenente l'operatore <=
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Less => {
                    // Prima del < si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano <."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano <."),
                    };

                    // Dopo il <, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per l'op booleano <."
                        );
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_boolean_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano <."),
                                    }
                                },
                                _ => unreachable!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano <."),
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano <."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Less con left e right
                    let less_expr = Less { left, right };

                    // Reinserisci l'oggetto Less nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(less_expr)));

                    //elimino il token contenente l'operatore <
                    tok_vec.nodes.remove(*index);
                }
                TokenType::GreatEqual => {
                    // Prima del >= si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano >=."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano >=."),
                    };

                    // Dopo il >=, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per l'op booleano >=."
                        );
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_arithmetic_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >=."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >=."),
                                    }
                                },
                                _ => unreachable!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano >=."),
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano >=."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto GreatEqual con left e right
                    let geq_expr = GreatEqual { left, right };

                    // Reinserisci l'oggetto Less nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(geq_expr)));

                    //elimino il token contenente l'operatore <
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Greater => {
                    // Prima del > si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per l'op booleano >."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a sinistra dell'op booleano >."),
                    };

                    // Dopo il >, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per l'op booleano >."
                        );
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_arithmetic_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >."),
                                    }
                                }
                            },
                            Any::ArithmeticExpression(_expr) =>{
                                let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra dell'op booleano >."),
                                    }
                                },
                                _ => unreachable!("Errore di parsing: nodo non riconosciuto a destra dell'op booleano >."),
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: nessun token trovato a destra dell'op booleano >."
                        );
                    };

                    //println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Grea con left e right
                    let great_expr = Great { left, right };

                    // Reinserisci l'oggetto Less nel vettore come BooleanExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(great_expr)));

                    //elimino il token contenente l'operatore <
                    tok_vec.nodes.remove(*index);
                }
                _ => {}
            }
        }
        // Incrementa l'indice per passare al prossimo token
        *index += 1;
    }
}

pub fn parse_arithmetic_unop(tok_vec: &mut AnyVec, index: &mut usize) {
    while *index < tok_vec.nodes.len() {
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Minus => {
                    // Assicurati che non ci sia un operando a sinistra
                    if *index > 0 {
                        // Controlla cosa c'è prima del segno meno
                        if let Some(node) = tok_vec.nodes.get(*index - 1) {
                            match node {
                                Any::ArithmeticExpression(_) => {
                                    // C'è un'espressione a sinistra, quindi il meno è binario
                                    // Puoi decidere di gestirlo come operatore binario
                                    return; // O interrompi qui, a seconda del caso
                                }
                                Any::Token(previous_token) => {
                                    // Controlla se il token precedente è un token che rappresenta
                                    // un operatore binario o un delimitatore (es. parentesi)
                                    match previous_token.token_ty {
                                        TokenType::Plus
                                        | TokenType::Minus
                                        | TokenType::Multiply
                                        | TokenType::Divide
                                        | TokenType::Bra
                                        | TokenType::Assign => {
                                            // Token valido per un operatore unario
                                        }
                                        _ => {
                                            // Se trovi un token non valido per un meno unario, interrompi
                                            unreachable!("Errore di parsing: operando sinistro non valido per il '-' unario.");
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    // Se non ci sono operandi a sinistra o il token precedente è valido
                    //rimuovo il token e continuo
                    tok_vec.nodes.remove(*index);

                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '-' unario."),
                                    }
                                }
                            }
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                    Any::ArithmeticExpression(expr) => expr,
                                    _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '-' unario."),
                                }
                            }
                            _ => unreachable!(
                                "Errore di parsing: nodo non riconosciuto a destra del '-' unario."
                            ),
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: nessun token trovato a destra del '-' unario."
                        );
                    };

                    let min_expr = Uminus { right };
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::ArithmeticExpression(Box::new(min_expr)));
                }
                _ => {}
            }
        }
        *index += 1;
    }
}

pub fn parse_bool_unop(tok_vec: &mut AnyVec, index: &mut usize) {
    while *index < tok_vec.nodes.len() {
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Not => {
                    // Assicurati che non ci sia un operando a sinistra
                    if *index > 0 {
                        // Controlla cosa c'è prima del segno meno
                        if let Some(node) = tok_vec.nodes.get(*index - 1) {
                            match node {
                                Any::BooleanExpression(_) => {
                                    // C'è un'espressione a sinistra, quindi il meno è binario
                                    // Puoi decidere di gestirlo come operatore binario
                                    return; // interrompo gestione delegata a parse_bool_expression
                                }
                                Any::Token(previous_token) => {
                                    // Controlla se il token precedente è un token che rappresenta
                                    // un operatore binario o un delimitatore (es. parentesi)
                                    match previous_token.token_ty {
                                        TokenType::And
                                        | TokenType::Or
                                        | TokenType::Bra
                                        | TokenType::Assign => {
                                            // Token valido per un operatore unario
                                        }
                                        _ => {
                                            // Se trovi un token non valido per un meno unario, interrompi
                                            unreachable!("Errore di parsing: operando sinistro non valido per il '-' unario.");
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    // Se non ci sono operandi a sinistra o il token precedente è valido
                    //rimuovo il token e continuo
                    tok_vec.nodes.remove(*index);

                    let expression = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    parse_bool_subexpression(tok_vec, index)
                                } else {
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::BooleanExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '-' unario."),
                                    }
                                }
                            }
                            Any::BooleanExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                    Any::BooleanExpression(expr) => expr,
                                    _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '-' unario."),
                                }
                            }
                            _ => unreachable!(
                                "Errore di parsing: nodo non riconosciuto a destra del '-' unario."
                            ),
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: nessun token trovato a destra del '-' unario."
                        );
                    };

                    let min_expr = Not { expression };
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::BooleanExpression(Box::new(min_expr)));
                }
                _ => {}
            }
        }
        *index += 1;
    }
}

pub fn remove_matching_braces(any_vec: &mut AnyVec, open_brace_index: usize) {
    // Assicuriamoci che l'indice `open_brace_index` sia valido e che corrisponda a una parentesi graffa aperta
    if let Some(Any::Token(ref open_token)) = any_vec.nodes.get(open_brace_index) {
        if open_token.token_ty == TokenType::CBra {
            // Scorriamo il vettore a partire dall'indice successivo per trovare la parentesi chiusa
            for i in open_brace_index + 1..any_vec.nodes.len() {
                if let Some(Any::Token(ref close_token)) = any_vec.nodes.get(i) {
                    if close_token.token_ty == TokenType::Cket {
                        // Rimuove la parentesi graffa aperta e chiusa
                        any_vec.nodes.remove(i); // Rimuove la parentesi graffa chiusa prima
                        any_vec.nodes.remove(open_brace_index); // Rimuove la parentesi graffa aperta
                        println!(
                            "Parentesi graffe aperta e chiusa trovate e rimosse agli indici {}, {}",
                            open_brace_index, i
                        );
                        return; // Esci dopo aver rimosso la coppia di parentesi
                    }
                }
            }
            unreachable!(
                "Errore: parentesi graffa chiusa non trovata dopo l'indice {}",
                open_brace_index
            );
        } else {
            unreachable!(
                "Errore: il nodo all'indice {} non è una parentesi graffa aperta",
                open_brace_index
            );
        }
    } else {
        unreachable!("Errore: indice non valido o il nodo non è un token");
    }
}

pub fn parse_arithmetic_expression(tok_vec: &mut AnyVec, index: &mut usize) {
    //println!("index:= {}", index);
    while *index < tok_vec.nodes.len() {
        // Controlla se il nodo attuale è un token
        if let Some(Any::Token(token)) = tok_vec.nodes.get(*index) {
            match token.token_ty {
                TokenType::Plus => {
                    // Prima del `+` si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per l'addizione."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra del '+'."
                        ),
                    };

                    // Dopo il `+`, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per l'addizione."
                        );
                    }
                    //println!("second print index:= {}", index);

                    // Se trovi una parentesi aperta, esegui parse_subexpression
                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsing subexpression");
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '+'."),
                                    }
                                }
                            }
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '+'."),
                                    }
                            }
                            _ => {
                                unreachable!(
                                    "Errore di parsing: nodo non riconosciuto a destra del '+'."
                                )
                            }
                        }
                    } else {
                        unreachable!("Errore di parsing: nessun token trovato a destra del '+'.");
                    };

                    // println!("printing the token vector after all");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }
                    // Crea l'oggetto Add con left e right
                    let add_expr = Add { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::ArithmeticExpression(Box::new(add_expr)));

                    //elimino il token contenente l'operatore +
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Multiply => {
                    // Prima del `*` si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per la moltiplicazione."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}", left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra del '*'."
                        ),
                    };

                    //println!("printing the token vector after the left elimination");
                    //let mut j = 0;
                    //while j < tok_vec.nodes.len()
                    //{
                    //  println!("{:?}", tok_vec.nodes[j]);
                    //   j=j+1;
                    //}

                    // Dopo il `*`, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per la moltiplicazione."
                        );
                    }
                    //println!("second print index:= {}" , index);

                    // Se trovi una parentesi aperta, esegui parse_subexpression

                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '*'."),
                                    }
                                }
                            }
                            //caso in cui ho già un ArithmeticExpression a dx
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '*'."),
                                    }
                            }
                            _ => {
                                unreachable!(
                                    "Errore di parsing: nodo non riconosciuto a destra del '*'."
                                )
                            }
                        }
                    } else {
                        unreachable!("Errore di parsing: nessun token trovato a destra del '*'.");
                    };

                    // Crea l'oggetto Product con left e right
                    let prod_expr = Product { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::ArithmeticExpression(Box::new(prod_expr)));

                    //elimino il token contenente l'operatore *
                    tok_vec.nodes.remove(*index);
                }
                TokenType::Minus => {
                    // Prima del `-` si trova l'operando sinistro (left)
                    if *index == 0 {
                        unreachable!(
                            "Errore di parsing: operando sinistro mancante per la sottrazione."
                        );
                    }
                    let left_node = tok_vec.nodes.remove(*index - 1);

                    //println!("left operand {:?}" , left_node);

                    let left = match left_node {
                        Any::ArithmeticExpression(expr) => expr,
                        _ => unreachable!(
                            "Errore di parsing: attesa espressione aritmetica a sinistra del '-'."
                        ),
                    };

                    // println!("printing the token vector after the left elimination");
                    // let mut j = 0;
                    // while j < tok_vec.nodes.len() {
                    //     println!("{:?}", tok_vec.nodes[j]);
                    //     j = j + 1;
                    // }

                    // Dopo il `-`, cerca l'operando destro
                    if *index >= tok_vec.nodes.len() {
                        unreachable!(
                            "Errore di parsing: operando destro mancante per la sottrazione."
                        );
                    }
                    //println!("second print index:= {}" , index);

                    // Se trovi una parentesi aperta, esegui parse_subexpression

                    let right = if let Some(node) = tok_vec.nodes.get(*index) {
                        match node {
                            Any::Token(token) => {
                                if let TokenType::Bra = token.token_ty {
                                    //println!("parsed by recursion right expression {:?}", parse_subexpression(tok_vec, index));
                                    parse_arithmetic_subexpression(tok_vec, index)
                                } else {
                                    // Token is not a parenthesis, check if it's a valid arithmetic expression
                                    let right_node = tok_vec.nodes.remove(*index);
                                    //println!("parsed right operand {:?}", right_node);
                                    match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '-'."),
                                    }
                                }
                            }
                            //caso in cui ho già un ArithmeticExpression a dx
                            Any::ArithmeticExpression(_expr) => {
                                let right_node = tok_vec.nodes.remove(*index);
                                //println!("parsed right operand {:?}", right_node);
                                match right_node {
                                        Any::ArithmeticExpression(expr) => expr,
                                        _ => unreachable!("Errore di parsing: attesa espressione aritmetica a destra del '-'."),
                                    }
                            }
                            _ => {
                                unreachable!(
                                    "Errore di parsing: nodo non riconosciuto a destra del '-'."
                                )
                            }
                        }
                    } else {
                        unreachable!("Errore di parsing: nessun token trovato a destra del '-'.");
                    };

                    // Crea l'oggetto Product con left e right
                    let diff_expr = Minus { left, right };

                    // Reinserisci l'oggetto Add nel vettore come ArithmeticExpression
                    tok_vec
                        .nodes
                        .insert(*index - 1, Any::ArithmeticExpression(Box::new(diff_expr)));

                    //elimino il token contenente l'operatore *
                    tok_vec.nodes.remove(*index);
                }

                _ => {}
            }
        }
        // Incrementa l'indice per passare al prossimo token
        *index += 1;
    }
}

//REMOVES ONLY THE CURLY BRACES WITHOUT ANYTHING IN THE MIDDLE
fn clean_curly_braces(any_vec: &mut AnyVec, index: &mut usize) {
       // Controlla se ci sono almeno due elementi per evitare accessi fuori dai limiti
       while *index < any_vec.nodes.len().saturating_sub(1) {
        if let (Any::Token(token_bra), Any::Token(token_ket)) = (&any_vec.nodes[*index], &any_vec.nodes[*index + 1]) {
            // Controlla se il primo token è `Bra` e il secondo è `Ket`
            if token_bra.token_ty == TokenType::CBra && token_ket.token_ty == TokenType::Cket {
                // Rimuovi entrambi i token
                any_vec.nodes.remove(*index);     // Rimuovi il token `Bra`
                any_vec.nodes.remove(*index);     // Rimuovi il token `Ket` (che ha preso il posto di Bra)
                // Non incrementare l'indice perché hai rimosso due elementi
                continue;
            }
        }
        // Incrementa l'indice solo se non hai rimosso nulla
        *index += 1;
    }
}

//RIMUOVE APPOSTA SOLO QUELLE ADIACENTI, A DIVERSI LIVELLI, QUINDI SOLO QUELLE RIGUARDANTI TOKEN GIA' PROCESSATI
fn clean_from_void(any_vec: &mut AnyVec) {
    let mut has_changes = true;

    // Continua a iterare fino a quando non ci sono più cambiamenti
    while has_changes {
        has_changes = false;
        let mut index = 0;

        while index < any_vec.nodes.len().saturating_sub(1) {
            if let (Any::Token(token_bra), Any::Token(token_ket)) =
                (&any_vec.nodes[index], &any_vec.nodes[index + 1])
            {
                // Controlla se il primo token è `Bra` e il secondo è `Ket`
                if (token_bra.token_ty == TokenType::Bra && token_ket.token_ty == TokenType::Ket)
                    || (token_bra.token_ty == TokenType::CBra
                        && token_ket.token_ty == TokenType::Cket)
                {
                    // Rimuovi entrambi i token e segna che ci sono stati cambiamenti
                    any_vec.nodes.remove(index); // Rimuovi `Bra`
                    any_vec.nodes.remove(index); // Rimuovi `Ket` (che ha preso il posto di Bra)
                    has_changes = true;

                    // Non incrementare l'indice perché hai rimosso due elementi
                    continue;
                }
            }
            // Incrementa l'indice solo se non hai rimosso nulla
            index += 1;
        }
    }
}

pub fn parse_substatement_block(
    any_vec: &mut AnyVec,
    index: &mut usize,
) -> Option<Box<dyn Statement>> 
{
    let start = *index;
    let mut depth = 0;

    // Scorrere fino alla parentesi graffa chiusa, incrementando la profondità
    while *index < any_vec.nodes.len() {
        match &any_vec.nodes[*index] {
            Any::Token(token) if token.token_ty == TokenType::CBra => {
                depth += 1;
            }
            Any::Token(token) if token.token_ty == TokenType::Cket => {
                depth -= 1;
                if depth == 0 {
                    *index += 1; // Include il token finale `}`
                    break;
                }
            }
            _ => {}
        }
        *index += 1;
    }

    // Debugging
    println!("PARSE BLOCK  POST CYCLE PRINTING");
    for (i, node) in any_vec.nodes.iter().enumerate() {
        println!("Indice: {}, Nodo: {:?}", i, node);
    }
    println!("DEPTH FINAL VALUE {:?}", depth);
    if depth != 0 {
        unreachable!("Errore di parsing: parentesi graffa chiusa mancante.");
    }

    // Drenare i token che compongono uno statement completo
    let mut sub_tok_vec = Vec::new();
    for i in start..*index {
        sub_tok_vec.push(any_vec.nodes.remove(start)); // Rimuovi da `any_vec` e aggiungi a `sub_tok_vec`
    }

    // Aggiorna l'indice principale per riflettere i token rimossi
    *index = start;

    // Crea un nuovo `AnyVec` per il blocco
    let mut sub_any_vec = AnyVec { nodes: sub_tok_vec };

    // Debugging
    println!("Contenuto di `sub_any_vec.nodes` prima del parse_statement da sub_block:");
    for (i, node) in sub_any_vec.nodes.iter().enumerate() {
        println!("Indice: {}, Nodo: {:?}", i, node);
    }


    // Richiama il parsing degli statement sulla sottoespressione
    let mut sub_index = 0; // Indice locale per la sottoespressione
    parse_statement(&mut sub_any_vec, &mut sub_index);

    println!("Contenuto di `any_vec.nodes` dopo del parse_statement da sub_block:");
    for (i, node) in sub_any_vec.nodes.iter().enumerate() {
        println!("Indice: {}, Nodo: {:?}", i, node);
    }
    // Controlla il risultato del parsing

    for node in sub_any_vec.nodes.iter() {
        if let Any::Statement(stmt) = node {
            return Some(stmt.clone_box()); // Trova e restituisce il primo `Statement`
        }
    }

    // Messaggio di errore nel caso non si trovi uno `Statement`
    eprintln!("Errore di parsing: nessuno Statement trovato nel blocco.");
    None
        
}

pub fn parse_statement(any_vec: &mut AnyVec, mut index: &mut usize) {
    while *index < any_vec.nodes.len() {
        println!("ANALYZED INDEXES: {:?}", *index);
        println!("ANALYZED ITEMS: {:?}", any_vec.nodes[*index]);
        if let Some(Any::Token(token)) = any_vec.nodes.get(*index) {
            // println!("printing current index: {:?}" ,index);
            // println!("printing current vector element: {:?}" , any_vec.nodes);
            match token.token_ty {
                //TODO Gestione dell'assegnazione: var := arith_expr
                TokenType::Assign => {
                    println!(
                        "TRYING TO REMOVE {:?} AT INDEX {:?}",
                        any_vec.nodes[*index], index
                    );
                    any_vec.nodes.remove(*index);
                    // Controlla che ci sia una variabile prima dell'assegnamento
                    // Stampa diagnostica iniziale per il contenuto di `any_vec`
                    println!("Contenuto di `any_vec.nodes` prima del parsing del `:=`:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    println!(
                        "TRYING TO REMOVE {:?} AT INDEX {:?}",
                        any_vec.nodes[*index - 1],
                        *index -1
                    );
                    let var_node = any_vec.nodes.remove(*index - 1); // Estrae il nodo della variabile
                    let var = match var_node.as_arithmetic_expr() {
                        Some(expr) => {
                            if let Some(variable) = expr.as_variable() {
                                variable
                            } else {
                                unreachable!(
                                    "Errore di parsing: attesa una variabile prima di ':='."
                                );
                            }
                        }
                        None => {
                            unreachable!("Errore di parsing: attesa una variabile prima di ':='.")
                        }
                    };

                    println!("Contenuto di `any_vec.nodes` dopo il parsing della variabile: ");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    // L’espressione aritmetica deve essere subito dopo l'assegnamento
                    println!(
                        "removing element {:?} at index {:?}",
                        any_vec.nodes[*index - 1],
                        *index -1
                    );
                    let expr_node = any_vec.nodes.remove(*index - 1); // Nessun incremento dell'indice qui
                    let expr = match expr_node.as_arithmetic_expr() {
                        Some(arith_expression) => arith_expression,
                        None => unreachable!(
                            "Errore di parsing: attesa un'espressione aritmetica a destra di ':='."
                        ),
                    };

                    // Creiamo e inseriamo lo statement di assegnamento
                    let assignment_stmt = Assign {
                        var_name: var.value.clone(),
                        expr: expr.clone_box(),
                    };
                    any_vec
                        .nodes
                        .insert(*index - 1, Any::Statement(Box::new(assignment_stmt))); // Inserisce lo statement

                    //Stampa diagnostica per confermare la situazione del vettore
                    println!("printing the vector after the assign insertion:");
                    for node in &any_vec.nodes {
                        println!("value {:?}", node);
                    }
                    *index -= 1;
                }
                //TODO Gestione della concatenazione: s1 ; s2 (s1 e s2 sono statements)
                TokenType::Semicolon => {
                    println!(
                        "Indice corrente prima di ogni operazione su `;`: {}",
                        *index
                    );
                    any_vec.nodes.remove(*index);

                    //Stampa diagnostica iniziale per il contenuto di `any_vec`
                    println!("Contenuto di `any_vec.nodes` prima del parsing del `;`:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    // Verifica che ci sia uno statement prima del `;`
                    if *index == 0 {
                        unreachable!("Errore di parsing: primo statement mancante prima di ';'.");
                    }

                    // Salviamo l'indice attuale come `start_index`
                    let start_index = *index;
                    //TODO BRACES CHECK
                    // Rimuove il primo statement (s1)
                    let s1_node = any_vec.nodes.remove(start_index - 1); // Rimuove subito s1
                    let s1 = match s1_node.as_statement() {
                        Some(stmt) => stmt,
                        None => {
                            unreachable!("Errore di parsing: atteso uno statement prima di ';'.")
                        }
                    };

                    println!("Primo statement trovato e rimosso: {:?}", s1);

                    println!("Contenuto di `any_vec.nodes` prima di parse_statement:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    // Chiamata a `parse_statement` per il prossimo statement
                    parse_statement(any_vec, index);

                    println!("Contenuto di `any_vec.nodes` dopo parse_statement:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    // Verifica del secondo statement
                    //println!("VECTOR ELEMENT: {:?} AT INDEX - 2 : {:?}" , any_vec.nodes[*index-2], *index-2);
                    println!(
                        "VECTOR ELEMENT: {:?} AT INDEX: {:?}",
                        any_vec.nodes[start_index-1],
                        start_index-1
                    );
                    if let Some(Any::Statement(_)) = any_vec.nodes.get(start_index-1) {
                        let s2_node = any_vec.nodes.remove(start_index-1);
                        let s2 = match s2_node.as_statement() {
                            Some(stmt) => stmt,
                            None => {
                                unreachable!("Errore di parsing: atteso uno statement dopo ';'.")
                            }
                        };

                        // Crea lo statement di concatenazione
                        let concat_stmt = Concat {
                            first: s1.clone_box(),
                            second: s2.clone_box(),
                        };

                        // Inserisce lo statement concatenato alla posizione corretta
                        any_vec
                            .nodes
                            .insert(start_index - 1, Any::Statement(Box::new(concat_stmt)));
                        // println!("--- Statement di concatenazione inserito ---");

                        // Stampa diagnostica del vettore per confermare la situazione
                        // println!("Contenuto di `any_vec.nodes` dopo la concatenazione:");
                        // for (i, node) in any_vec.nodes.iter().enumerate() {
                        //     println!("Indice: {}, Nodo: {:?}", i, node);
                        // }
                    } else {
                        unreachable!("Errore di parsing: atteso uno statement dopo ';'.");
                    }
                }

                //TODO Gestione del condizionale if then else
                TokenType::If => {
                    let startpos = index.clone();
                    println!("IF TOKEN FOUND");
                   

                    // Rimuove il token `If`
                    any_vec.nodes.remove(*index);
                    println!("IF PRINTING after removing if token:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    // Controlla che l'elemento in `any_vec.nodes[index]` sia una `BooleanExpression`
                    let guard = match any_vec.nodes.get(*index) {
                        Some(Any::BooleanExpression(expr)) => expr.clone_box(),
                        _ => unreachable!(
                            "Errore di parsing: attesa una espressione booleana dopo 'if'."
                        ),
                    };
                    any_vec.nodes.remove(*index); // Rimuove la BooleanExpression

                    println!("IF PRINTING after guard removing:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }
                    // Check della presenza del token `Then` in posizione `index`
                    let then_token = any_vec.nodes.remove(*index);
                    let branch = match then_token.as_token() {
                        Some(t) => t,
                        None => unreachable!(
                            "Errore di parsing: atteso token 'then' dopo condizionale 'if'."
                        ),
                    };
                    if branch.token_ty != TokenType::Then {
                        unreachable!("Errore di parsing: atteso token 'then' ma trovato altro.")
                    }
                    println!(
                        "ELEMENT BEFORE BLOCK CALL {:?} AT INDEX {:?}",
                        any_vec.nodes[*index], *index
                    );
                    println!("IF PRINTING before then block:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }
                    // Parsing del blocco `then` con `parse_statement_block`
                    println!("Parsing del blocco THEN...");
                    let then_expr = parse_substatement_block(any_vec, index).unwrap_or_else(|| {
                        unreachable!("Errore di parsing: atteso uno statement dopo il 'then'.")
                    });
                    
                    clean_curly_braces(any_vec, & mut 0);

                    println!("IF PRINTING after curly removement:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    println!(
                        "TRYING TO REMOVE IN ELSE BRANCH {:?} AT INDEX {:?}", any_vec.nodes[*index], *index);
                    // Controllo per il token `else` dopo il blocco `then`
                    if let Some(Any::Token(tok)) = any_vec.nodes.get(*index) {
                        if tok.token_ty == TokenType::Else {
                            // Rimuove il token `else`
                            any_vec.nodes.remove(*index);

                            // Parsing del blocco `else` con `parse_statement_block`
                            println!("Parsing del blocco ELSE...");
                            let else_expr = parse_substatement_block(any_vec, index)
                                .unwrap_or_else(|| {
                                    unreachable!(
                                        "Errore di parsing: atteso uno statement dopo 'else'."
                                    )
                                });

                            // Crea lo statement `IfThenElse` con entrambi i rami `then` ed `else`
                            let if_stmt = IfThenElse {
                                guard,
                                true_expr: then_expr.clone_box(),
                                false_expr: else_expr.clone_box(),
                            };

                            // Inserisce il risultato `IfThenElse` in `any_vec.nodes` alla posizione originale
                            any_vec
                                .nodes
                                .insert(startpos, Any::Statement(Box::new(if_stmt)));
                        } else {
                            // Caso in cui non c'è il token `else`, quindi inserisce uno statement `Skip`
                            let skip_stmt = Skip;
                            let if_stmt = IfThenElse {
                                guard,
                                true_expr: then_expr.clone_box(),
                                false_expr: Box::new(skip_stmt),
                            };

                            any_vec
                                .nodes
                                .insert(startpos, Any::Statement(Box::new(if_stmt)));
                        }
                    } else {
                        // Caso in cui non c'è il token `else`, quindi inserisce uno statement `Skip`
                        let skip_stmt = Skip;
                        let if_stmt = IfThenElse {
                            guard,
                            true_expr: then_expr.clone_box(),
                            false_expr: Box::new(skip_stmt),
                        };

                        any_vec
                            .nodes
                            .insert(startpos, Any::Statement(Box::new(if_stmt)));
                    }
                }

                //TODO Gestione del ciclo while
                TokenType::While => {
                    println!("WHILE PRINTING:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }
                    // Rimozione del token `While` dal vettore e check del token aperto `(`
                    println!(
                        "removing element IN WHILE {:?} at index {:?}",
                        any_vec.nodes[*index], *index
                    );
                    any_vec.nodes.remove(*index);
                    println!(
                        "should be open_bra IN WHILE {:?} at index {:?}",
                        any_vec.nodes[*index], *index
                    );
                    let open_paren = any_vec.nodes.get(*index);
                    if let Some(Any::Token(t)) = open_paren {
                        if t.token_ty != TokenType::Bra {
                            unreachable!(
                                "Errore di parsing: attesa una parentesi aperta '(' dopo 'while'."
                            );
                        } else {
                            any_vec.nodes.remove(*index);
                        }
                    } else {
                        unreachable!("Errore di parsing: atteso un token dopo 'while'.");
                    }
                    println!("WHILE PRINTING after bra:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }

                    // Parsing della guardia booleana del ciclo `while`
                    println!(
                        "should be guard IN WHILE {:?} at index {:?}",
                        any_vec.nodes[*index], *index
                    );
                    let guard = match any_vec.nodes.get(*index) {
                        Some(Any::BooleanExpression(expr)) => expr.clone_box(),
                        _ => unreachable!(
                            "Errore di parsing: attesa una espressione booleana dopo 'while'."
                        ),
                    };
                    any_vec.nodes.remove(*index);
                    clean_from_void(any_vec);
                    println!("WHILE PRINTING after guard:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }
                    // Check del token chiuso `)`
                    println!(
                        "should be close_ket IN WHILE {:?} at index {:?}",
                        any_vec.nodes[*index], *index
                    );
                    let close_paren = any_vec.nodes.get(*index);
                    if let Some(Any::Token(t)) = close_paren {
                        if t.token_ty != TokenType::Ket {
                            unreachable!("Errore di parsing: attesa una parentesi chiusa ')' dopo guardia while.");
                        } else {
                            any_vec.nodes.remove(*index);
                        }
                    } else {
                        unreachable!("Errore di parsing: atteso un token dopo 'while'.");
                    }

                    println!("WHILE PRINTING after ket:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }
                    // Avanza l'indice e controlla la parentesi graffa aperta `{`
                    let open_brace = any_vec.nodes.get(*index);
                    if let Some(Any::Token(t)) = open_brace {
                        if t.token_ty != TokenType::CBra {
                            unreachable!("Errore di parsing: attesa '{{' dopo la guardia.");
                        }
                    } else {
                        unreachable!("Errore di parsing: atteso '{{' dopo la guardia.");
                    }
                    println!("WHILE PRINTING after curly-bra:");
                    for (i, node) in any_vec.nodes.iter().enumerate() {
                        println!("Indice: {}, Nodo: {:?}", i, node);
                    }
                    println!(
                        "should be c_bra IN WHILE {:?} at index {:?}",
                        any_vec.nodes[*index], *index
                    );
                    let body_start_index = *index;
                    // Utilizza parse_statement_block per ottenere il body del ciclo `while`
                    let body = match parse_substatement_block(any_vec, index) {
                        Some(statement) => statement,
                        None => Box::new(Skip), // Se il body è vuoto, utilizza uno statement Skip come default
                    };

                    // Creazione dell'oggetto While
                    let while_stmt = While { guard, body };

                    // Inserimento del `while` statement nel vettore any_vec.nodes
                    
                    any_vec
                        .nodes
                        .insert(body_start_index, Any::Statement(Box::new(while_stmt)));
                }

                //TODO Gestione ciclo for
                TokenType::For => {
                    // Controlla la presenza di '(' dopo 'for'
                    *index += 1;
                    let open_paren = any_vec.nodes.get(*index);
                    if let Some(Any::Token(t)) = open_paren {
                        if t.token_ty != TokenType::Bra {
                            unreachable!(
                                "Errore di parsing: attesa una parentesi aperta '(' dopo 'for'."
                            );
                        }
                    } else {
                        unreachable!("Errore di parsing: atteso un token dopo 'for'.");
                    }

                    *index += 1;

                    // Parsing di init (deve essere uno Statement)
                    let parsed_init = if let Some(Any::Statement(stmt)) = any_vec.nodes.get(*index)
                    {
                        // Cast per verificare che lo statement sia un `Assign`
                        if let Some(assign_stmt) = stmt.as_any().downcast_ref::<Assign>() {
                            assign_stmt.clone_box() // Usa il clone del valore `Assign`
                        } else {
                            unreachable!("Errore di parsing: atteso un 'Assign' come prima condizione del 'for'.");
                        }
                    } else {
                        unreachable!("Errore di parsing: atteso uno statement come prima condizione del 'for'.");
                    };

                    // Parsing del guard (deve essere una BooleanExpression)
                    *index += 1;
                    let guard = match any_vec.nodes.get(*index) {
                        Some(Any::BooleanExpression(expr)) => expr.clone_box(),
                        _ => {
                            unreachable!("Errore di parsing: attesa un' espressione booleana come seconda condizione del 'for'.");
                        }
                    };

                    *index += 1;

                    // Parsing di increment (deve essere uno Statement)
                    parse_statement(any_vec, index);
                    //TODO FIX INDEX PROBLEM
                    let parsed_increment = if let Some(Any::Statement(stmt)) =
                        any_vec.nodes.get(*index)
                    {
                        // Cast per verificare che lo statement sia un `Assign`
                        if let Some(assign_stmt) = stmt.as_any().downcast_ref::<Assign>() {
                            assign_stmt.clone_box() // Usa il clone del valore `Assign`
                        } else {
                            unreachable!("Errore di parsing: atteso un 'Assign' come terza condizione del 'for'.");
                        }
                    } else {
                        unreachable!("Errore di parsing: atteso uno statement come terza condizione del 'for'.");
                    };

                    // Controlla la presenza di ')' dopo l'increment
                    *index += 1;
                    let close_paren = any_vec.nodes.get(*index);
                    if let Some(Any::Token(t)) = close_paren {
                        if t.token_ty != TokenType::Ket {
                            unreachable!("Errore di parsing: attesa una parentesi chiusa ')' dopo l'increment.");
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: atteso un token di chiusura dopo l'increment."
                        );
                    }

                    // Parsing del body
                    *index += 1;
                    let mut body: Option<Box<dyn Statement>> = None;

                    let open_brace = any_vec.nodes.get(*index);
                    match open_brace {
                        Some(Any::Token(t)) if t.token_ty == TokenType::CBra => {
                            // Trovata la parentesi aperta, procedi con il parsing del body
                            *index += 1;
                        }
                        _ => unreachable!(
                            "Errore di parsing: attesa '{{' dopo la condizione del for."
                        ),
                    }

                    while *index < any_vec.nodes.len() {
                        match any_vec.nodes.get(*index) {
                            Some(Any::Token(t)) if t.token_ty == TokenType::Cket => {
                                // Trovata la parentesi chiusa, fine del body
                                *index += 1;
                                break;
                            }
                            _ => {
                                // Parsiamo il prossimo statement nel body
                                parse_statement(any_vec, index);
                                //TODO FIX INDEX PROBLEM
                                if let Some(Any::Statement(stmt)) = any_vec.nodes.get(*index) {
                                    let parsed_stmt = stmt.clone_box();

                                    // Se già esiste uno statement nel body, concateno i nuovi statement
                                    body = match body {
                                        Some(existing_body) => {
                                            // Creazione di un nuovo `Concat` statement per concatenare
                                            Some(Box::new(Concat {
                                                first: existing_body,
                                                second: parsed_stmt,
                                            }))
                                        }
                                        None => Some(parsed_stmt), // Primo statement nel body
                                    };
                                } else {
                                    unreachable!("Errore di parsing: atteso uno statement nel body del ciclo for.");
                                }

                                *index += 1;
                            }
                        }
                    }

                    // Creazione dell'oggetto `For` utilizzando `parsed_init`, `guard`, `parsed_increment` e `body`
                    let for_stmt = For {
                        init: parsed_init, // Si può usare unwrap perché abbiamo già fatto il check
                        guard,
                        increment: parsed_increment,
                        body: body.unwrap_or_else(|| Box::new(Skip)), // Usa uno Skip se il body è vuoto
                    };

                    // Inserisci il ciclo for nel vettore any_vec
                    any_vec
                        .nodes
                        .insert(*index - 1, Any::Statement(Box::new(for_stmt)));

                    //any_vec.nodes.remove(*index);
                }
                //TODO Gestione repeat until
                TokenType::Repeat => {
                    // repeat-until: repeat {body} until guard
                    // guard: BooleanExpression

                    // Parsing del body
                    *index += 1;
                    let mut body: Option<Box<dyn Statement>> = None;

                    let open_brace = any_vec.nodes.get(*index);
                    match open_brace {
                        Some(Any::Token(t)) if t.token_ty == TokenType::CBra => {
                            // Trovata la parentesi aperta, procedi con il parsing del body
                            *index += 1;
                        }
                        _ => unreachable!(
                            "Errore di parsing: attesa '{{' dopo la condizione del repeat until."
                        ),
                    }

                    while *index < any_vec.nodes.len() {
                        match any_vec.nodes.get(*index) {
                            Some(Any::Token(t)) if t.token_ty == TokenType::Cket => {
                                // Trovata la parentesi chiusa, fine del body
                                *index += 1;
                                break;
                            }
                            _ => {
                                // Parsiamo il prossimo statement nel body
                                parse_statement(any_vec, index);

                                if let Some(Any::Statement(stmt)) = any_vec.nodes.get(*index) {
                                    let parsed_stmt = stmt.clone_box();

                                    // Se già esiste uno statement nel body, concateno i nuovi statement
                                    body = match body {
                                        Some(existing_body) => {
                                            // Creazione di un nuovo `Concat` statement per concatenare
                                            Some(Box::new(Concat {
                                                first: existing_body,
                                                second: parsed_stmt,
                                            }))
                                        }
                                        None => Some(parsed_stmt), // Primo statement nel body
                                    };
                                } else {
                                    unreachable!("Errore di parsing: atteso uno statement nel body del ciclo repeat until.");
                                }

                                *index += 1;
                            }
                        }
                    }
                    *index += 1;
                    //match su token until dopo il body
                    let until_node = any_vec.nodes.get(*index);
                    if let Some(Any::Token(t)) = until_node {
                        if t.token_ty != TokenType::Until {
                            unreachable!(
                                "Errore di parsing: atteso Token Until dopo repeat e body."
                            );
                        }
                    } else {
                        unreachable!(
                            "Errore di parsing: atteso un token dopo body del repeat until."
                        );
                    }
                    let guard = match any_vec.nodes.get(*index) {
                        Some(Any::BooleanExpression(expr)) => expr.clone_box(),
                        _ => {
                            unreachable!("Errore di parsing: attesa un' espressione booleana come seconda condizione del repeat until.");
                        }
                    };

                    let repeat_until_statement = RepeatUntil {
                        body: body.unwrap_or_else(|| Box::new(Skip)),
                        guard,
                    };

                    any_vec
                        .nodes
                        .insert(*index - 1, Any::Statement(Box::new(repeat_until_statement)));
                    //any_vec.nodes.remove(*index);
                }
                _ => {}
            }
        }
        *index += 1;
    }
}

//TODO GENERAL: IMPLEMENTA PARSE REPEAT, POI EVALUATION DELL'AST
pub fn analyze(program: String, initial_state: String) {
    //cleaning the input from whitespaces
    let cleanp = program.trim();
    //----------------------------------------------------------------------------------------------------------------------------------------------------
    //LEXING SECTION
    //----------------------------------------------------------------------------------------------------------------------------------------------------

    //let the lexer work (string->AnyVec)
    let tokens = Lexer::tokenize(cleanp.to_owned());
    let tokenized_program = TokenVec { tokens };
    let state_tokens = Lexer::tokenize(initial_state);
    let tokenized_state = TokenVec {
        tokens: state_tokens,
    };
    //print!("tokenized initial state: {}", parsed_state);
    //print!("tokenized program code: {}", pre_ast);

    //let's build the ast! (AnyVec->Statement)
    // building the any vector that contains tokens and expressions
    let mut any_vec = AnyVec::new();
    for token in tokenized_program.tokens {
        any_vec.push_token(token);
    }
    let mut state_vec = AnyVec::new();
    for token in tokenized_state.tokens {
        state_vec.push_token(token);
    }

    let mut index = 0 as usize;
    //----------------------------------------------------------------------------------------------------------------------------------------------------
    //PARSING SECTION
    //----------------------------------------------------------------------------------------------------------------------------------------------------

    parse_atomic(&mut state_vec, &mut index);
    index = 0;
    parse_atomic(&mut any_vec, &mut index);
    index = 0;

    // println!("atomic terms parsed: ");
    // let mut j = 0;
    // while j < any_vec.nodes.len() {
    //     println!("{:?}", state_vec.nodes[j]);
    //     j = j + 1;
    // }
    parse_arithmetic_unop(&mut state_vec, &mut index);
    index = 0;
    parse_arithmetic_unop(&mut any_vec, &mut index);
    index = 0;
    parse_bool_unop(&mut state_vec, &mut index);
    index = 0;
    parse_bool_unop(&mut any_vec, &mut index);
    index = 0;
    //arithmetic expressions
    parse_arithmetic_expression(&mut state_vec, &mut index);
    index = 0;
    parse_arithmetic_expression(&mut any_vec, &mut index);
    index = 0;
    parse_bool_expression(&mut state_vec, &mut index);
    index = 0;
    parse_bool_expression(&mut any_vec, &mut index);
    index = 0;
    // clean_from_void(&mut state_vec, &mut index);
    // index = 0;
    // clean_from_void(&mut any_vec, &mut index);
    // index = 0;

    println!("expressions parsed: ");
    let mut j = 0;
    while j < any_vec.nodes.len() {
        println!("{:?}", any_vec.nodes[j]);
        j = j + 1;
    }
    //statements
    parse_statement(&mut state_vec, &mut index);
    index = 0;
    println!("state parsed: ");
    let mut j = 0;
    while j < state_vec.nodes.len() {
        println!("{:?}", state_vec.nodes[j]);
        j = j + 1;
    }
    parse_statement(&mut any_vec, &mut index);
    index = 0;
    clean_from_void(&mut any_vec);
    index = 0;

    println!("statements parsed: ");
    let mut j = 0;
    while j < any_vec.nodes.len() {
        println!("{:?}", any_vec.nodes[j]);
        j = j + 1;
    }

    //----------------------------------------------------------------------------------------------------------------------------------------------------
    //EVALUATING SECTION
    //----------------------------------------------------------------------------------------------------------------------------------------------------
    // evaluate the final statement
    // if let Some(last_node) = any_vec.nodes.last(){
    //     if let Some(statement) = last_node.as_statement(){
    //         statement.evaluate();
    //     }
    // }
    //occhio al caso angeli degli spazi cancellati: 10- -10
}
