use crate::abstract_state::AbstractState;
use crate::abstract_domain::AbstractInterval;
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait BooleanExpression: Debug {
    fn clone_box(&self) -> Box<dyn BooleanExpression>;
    fn evaluate(&self, state: & mut State) -> bool;
    fn abs_evaluate(&self, state: & mut AbstractState) -> AbstractState;
}

#[derive(Debug)]
pub struct Boolean(pub bool);

impl BooleanExpression for Boolean {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Boolean(self.0)) // Crea un nuovo Box con una copia di Numeral
    }
    fn evaluate(&self, _state: & mut State) -> bool {
        self.0
    }
    fn abs_evaluate(&self, state: & mut AbstractState) -> AbstractState
    {
        if self.0  {
            state.clone()
        } else {
            AbstractState::bottom(state)
        }
    }
}

#[derive(Debug)]
pub struct Equal {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for Equal {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Equal {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) == self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState) -> AbstractState {
        if state.is_bottom() {
            return AbstractState::bottom(state);
        }

        let left_eval = self.left.abs_evaluate(state);
        let right_eval = self.right.abs_evaluate(state);

        match (left_eval, right_eval) {
            (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => AbstractState::bottom(state),
            (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. }) if left_eval == right_eval => {
                if let Some(var_name) = self.left.as_variable() {
                    state.update_interval(&var_name.value, left_eval.clone());
                    state.clone()
                } else {
                    unreachable!("Left operand of == must be a variable!");
                }
            }
            _ => AbstractState::bottom(state), //caso in cui i due intervalli non sono uguali o bounded == bottom
        }
    }
}

#[derive(Debug)]
pub struct GreatEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for GreatEqual {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(GreatEqual{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) >= self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState) -> AbstractState {
        if state.is_bottom() {
            return AbstractState::bottom(state);
        }
    
        let left_eval = self.left.abs_evaluate(state);
        let right_eval = self.right.abs_evaluate(state);
    
        match (left_eval, right_eval) {
            // Caso base: uno dei due è Bottom
            (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => AbstractState::bottom(state),
    
            // Caso concreto: Entrambi gli intervalli sono bounded
            (AbstractInterval::Bounded { lower: l1, upper: u1 }, AbstractInterval::Bounded { lower: l2, upper: _ }) => {
                if let Some(var_name) = self.left.as_variable() {
                    if u1 >= l2 {
                        // Aggiorniamo l'intervallo della variabile sinistra
                        state.update_interval(
                            &var_name.value,
                            AbstractInterval::Bounded {
                                lower: std::cmp::max(l1,l2),
                                upper: u1,
                            },
                        );
                        state.clone()
                    } else {
                        // Non soddisfatto: Bottom
                        AbstractState::bottom(state)
                    }
                } else {
                    // Se il lato sinistro non è una variabile, errore logico
                    unreachable!("Left operand of > must be a variable!");
                }
            }
            (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        }
    }
}
#[derive(Debug)]
pub struct Great {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}
impl BooleanExpression for Great {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Great{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) > self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState) -> AbstractState {
        if state.is_bottom() {
            return AbstractState::bottom(state);
        }
    
        let left_eval = self.left.abs_evaluate(state);
        let right_eval = self.right.abs_evaluate(state);
    
        match (left_eval, right_eval) {
            // Caso base: uno dei due è Bottom
            (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => AbstractState::bottom(state),
    
            // Caso concreto: Entrambi gli intervalli sono bounded
            (AbstractInterval::Bounded { lower: l1, upper: u1 }, AbstractInterval::Bounded { lower: l2, upper: _ }) => {
                if let Some(var_name) = self.left.as_variable() {
                    if u1 > l2 {
                        // Aggiorniamo l'intervallo della variabile sinistra
                        state.update_interval(
                            &var_name.value,
                            AbstractInterval::Bounded {
                                lower: std::cmp::max(l1,l2),
                                upper: u1,
                            },
                        );
                        state.clone()
                    } else {
                        // Non soddisfatto: Bottom
                        AbstractState::bottom(state)
                    }
                } else {
                    // Se il lato sinistro non è una variabile, errore logico
                    unreachable!("Left operand of > must be a variable!");
                }
            }
            (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        }
    }
    
}

#[derive(Debug)]
pub struct LessEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for LessEqual {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(LessEqual{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) <= self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState) -> AbstractState {
        if state.is_bottom() {
            return AbstractState::bottom(state);
        }
    
        let left_eval = self.left.abs_evaluate(state);
        let right_eval = self.right.abs_evaluate(state);
    
        match (left_eval, right_eval) {
            // Caso base: uno dei due è Bottom
            (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => AbstractState::bottom(state),
    
            // Caso concreto: Entrambi gli intervalli sono bounded
            (AbstractInterval::Bounded { lower: l1, upper: u1 }, AbstractInterval::Bounded { lower: _, upper: u2 }) => {
                if let Some(var_name) = self.left.as_variable() {
                    if l1 <= u2 {
                        // Aggiorniamo l'intervallo della variabile sinistra
                        state.update_interval(
                            &var_name.value,
                            AbstractInterval::Bounded {
                                lower: l1,
                                upper: std::cmp::min(u1, u2),
                            },
                        );
                        state.clone()
                    } else {
                        // Non soddisfatto: Bottom
                        AbstractState::bottom(state)
                    }
                } else {
                    // Se il lato sinistro non è una variabile, errore logico
                    unreachable!("Left operand of <= must be a variable!");
                }
            }
            (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        }
    }
    
}  

#[derive(Debug)]
pub struct Less {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for Less {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Less{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: & mut State) -> bool {
        self.left.evaluate(state) < self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState) -> AbstractState {
        if state.is_bottom() {
            return AbstractState::bottom(state);
        }
    
        let left_eval = self.left.abs_evaluate(state);
        let right_eval = self.right.abs_evaluate(state);
    
        match (left_eval, right_eval) {
            // Caso base: uno dei due è Bottom
            (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => AbstractState::bottom(state),
    
            // Caso concreto: Entrambi gli intervalli sono bounded
            (AbstractInterval::Bounded { lower: l1, upper: u1 }, AbstractInterval::Bounded { lower: _, upper: u2 }) => {
                if let Some(var_name) = self.left.as_variable() {
                    if l1 < u2 {
                        // Aggiorniamo l'intervallo della variabile sinistra
                        state.update_interval(
                            &var_name.value,
                            AbstractInterval::Bounded {
                                lower: l1,
                                upper: std::cmp::min(u1, u2),
                            },
                        );
                        state.clone()
                    } else {
                        // Non soddisfatto: Bottom
                        AbstractState::bottom(state)
                    }
                } else {
                    // Se il lato sinistro non è una variabile, errore logico
                    unreachable!("Left operand of < must be a variable");
                }
            }
            (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        }
    }
    
}

#[derive(Debug)]
pub struct And {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for And {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(And{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: & mut AbstractState) -> AbstractState {
       let left_eval = self.left.abs_evaluate(state);
       let right_eval = self.right.abs_evaluate(state);

       if left_eval.is_bottom || right_eval.is_bottom 
        {
            return AbstractState::bottom(state);
        }
        let mut new_variables = HashMap::new();

        for key in left_eval.variables.keys().chain(right_eval.variables.keys()) {
            // retrieve the left and right interval from the state
            let left_interval = left_eval.variables.get(key);
            let right_interval = right_eval.variables.get(key);
            //combine them using the interval intersection
            let intersec_interval = match (left_interval, right_interval) {
                (Some(l), Some(r)) => l.intersect(r),
                (Some(l), None) => l.clone(),
                (None, Some(r)) => r.clone(),
                (None, None) => AbstractInterval::Top, 
            };

            new_variables.insert(key.clone(), intersec_interval);
        }

        AbstractState {
            is_bottom: false,
            variables: new_variables,
        }
    }
}


#[derive(Debug)]
pub struct Or {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Or {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Or{
            left:self.left.clone_box(),
            right:self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) || self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: & mut AbstractState) -> AbstractState {
        let left_eval = self.left.abs_evaluate(state);
        let right_eval = self.right.abs_evaluate(state);

        if left_eval.is_bottom || right_eval.is_bottom 
        {
            return AbstractState::bottom(state);
        }
        let mut new_variables = HashMap::new();
        for key in left_eval.variables.keys().chain(right_eval.variables.keys())
        {
            let left_interval = left_eval.variables.get(key);
            let right_interval = right_eval.variables.get(key);

            let union_interval= match (left_interval, right_interval){
                (Some(l), Some(r)) => l.int_lub(r),
                (Some(l) , None) => l.clone(),
                (None, Some(r)) => r.clone(),
                (None, None) => AbstractInterval::Top
            };
            new_variables.insert(key.clone(), union_interval);
        }
        AbstractState{
            is_bottom: false,
            variables: new_variables,
        }
    }
}

#[derive(Debug)]
pub struct Not {
    pub expression: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Not {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Not{
            expression:self.expression.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        !(self.expression.evaluate(state))
    }
    fn abs_evaluate(&self, state: & mut AbstractState) -> AbstractState {
        let expr_eval=self.expression.abs_evaluate(state);

        if expr_eval.is_bottom()
        {
            return AbstractState::bottom(state);
        }

        let mut new_variables=HashMap::new();
        for key in expr_eval.variables.keys(){
            let expr_interval = expr_eval.variables.get(key);

            let neg_interval = match expr_interval
            {
                Some(i) => -(*i),
                None => AbstractInterval::Top,
            };

            new_variables.insert(key.clone(), neg_interval);
        }
        let negated_state= AbstractState{is_bottom: false, variables:new_variables};
        state.state_lub(&negated_state)
    }
}
