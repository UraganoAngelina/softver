use crate::ast::State;
use std::fmt::Debug;

pub trait ArithmeticExpression: Debug {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression>;
    fn as_variable(&self) -> Option<&Variable>;
    fn evaluate(&self, state: &State) -> i32;
}

#[derive(Debug)]
pub struct Numeral(pub i32);

impl ArithmeticExpression for Numeral {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Numeral(self.0)) // Crea un nuovo Box con una copia di Numeral
    }
    fn as_variable(&self) -> Option<&Variable> {
        // Restituisce Some(self) se è una variabile
        None
    }
    fn evaluate(&self, _state: &State) -> i32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Variable {
    pub value: String,
}

impl ArithmeticExpression for Variable {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Variable {
            value: self.value.clone(),
        }) // Crea un nuovo Box con una copia di Variable
    }
    fn as_variable(&self) -> Option<&Variable> {
        // Restituisce Some(self) se è una variabile
        Some(self)
    }
    fn evaluate(&self, state: &State) -> i32 {
        match state.get(&self.value) {
            Some(&val) => val,
            None => panic!("Variabile '{}' non trovata nello stato!", self.value),
        }
    }
}

#[derive(Debug)]
pub struct Add {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Add {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Add {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        }) // Crea un nuovo Box con una copia di Product
    }
    fn as_variable(&self) -> Option<&Variable> {
        // Restituisce Some(self) se è una variabile
        None
    }
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) + self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Product {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Product {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Product {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        }) // Crea un nuovo Box con una copia di Product
    }
    fn as_variable(&self) -> Option<&Variable> {
        // Restituisce Some(self) se è una variabile
        None
    }
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) * self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Minus {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Minus {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Minus {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        }) // Crea un nuovo Box con una copia di Minus
    }

    fn as_variable(&self) -> Option<&Variable> {
        // Restituisce Some(self) se è una variabile
        None
    }
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) - self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Uminus {
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Uminus {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Uminus {
            right: self.right.clone_box(),
        }) // Crea un nuovo Box con una copia di Uminus
    }
    fn as_variable(&self) -> Option<&Variable> {
        // Restituisce Some(self) se è una variabile
        None
    }
    fn evaluate(&self, state: &State) -> i32 {
        -self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Divide {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Divide {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Divide {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        }) // Crea un nuovo Box con una copia di Divide
    }
    fn as_variable(&self) -> Option<&Variable> {
        // Restituisce Some(self) se è una variabile
        None
    }
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) / self.right.evaluate(state)
    }
}
