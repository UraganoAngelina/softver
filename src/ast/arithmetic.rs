use crate::ast::State;
use std::fmt::Debug;
//use std::any::Any;

pub trait ArithmeticExpression: Debug {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression>;
    fn as_variable(&self) -> Option<&Variable>;
    fn evaluate(&self, state: &State) -> i64;
    //fn as_any(&self) -> &dyn Any;

    // Aggiungi to_string al trait
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct Numeral(pub i64);

impl ArithmeticExpression for Numeral {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Numeral(self.0))
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, _state: &State) -> i64 {
        self.0
    }
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug)]
pub struct Variable {
    pub value: String,
}

impl ArithmeticExpression for Variable {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Variable {
            value: self.value.clone(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        Some(self)
    }
    fn evaluate(&self, state: &State) -> i64 {
        *state.get(&self.value).expect("Variabile non trovata!")
    }
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct Add {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Add {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Add {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &State) -> i64 {
        self.left.evaluate(state) + self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("({} + {})", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Product {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Product {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Product {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &State) -> i64 {
        self.left.evaluate(state) * self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("({} * {})", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Minus {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Minus {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Minus {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &State) -> i64 {
        self.left.evaluate(state) - self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("({} - {})", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Uminus {
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Uminus {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Uminus {
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &State) -> i64 {
        -self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("-{}", self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Divide {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Divide {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Divide {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &State) -> i64 {
        self.left.evaluate(state) / self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("({} / {})", self.left.to_string(), self.right.to_string())
    }
}
