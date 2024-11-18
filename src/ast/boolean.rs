use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use std::fmt::Debug;
//use std::any::Any;

pub trait BooleanExpression: Debug {
    fn clone_box(&self) -> Box<dyn BooleanExpression>;
    fn evaluate(&self, state: & mut State) -> bool;
    //fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct Boolean(pub bool);

impl BooleanExpression for Boolean {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Boolean(self.0)) // Crea un nuovo Box con una copia di Numeral
    }
    fn evaluate(&self, _state: & mut State) -> bool {
        self.0
    }
}

#[derive(Debug)]
pub struct Equal {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for Equal {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Equal {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) == self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct GreatEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for GreatEqual {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(GreatEqual{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) >= self.right.evaluate(state)
    }
}
#[derive(Debug)]
pub struct Great {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}
impl BooleanExpression for Great {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Great{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) > self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct LessEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for LessEqual {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(LessEqual{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) <= self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Less {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for Less {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Less{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) < self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct And {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for And {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(And{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Or {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Or {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Or{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) || self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Not {
    pub expression: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Not {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Not{
            expression:self.expression.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        !(self.expression.evaluate(state))
    }
}
