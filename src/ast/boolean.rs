use crate::abstract_domain::AbstractDomain;
use crate::abstract_interval::AbstractInterval;
use crate::abstract_state::AbstractState;
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use std::collections::HashMap;
use std::fmt::Debug;

use super::arithmetic::{Numeral, Add};

pub trait BooleanExpression: Debug {
    fn clone_box(&self) -> Box<dyn BooleanExpression>;
    fn evaluate(&self, state: &mut State) -> bool;
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState;
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct Boolean(pub bool);

impl BooleanExpression for Boolean {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Boolean(self.0)) // Crea un nuovo Box con una copia di Numeral
    }
    fn evaluate(&self, _state: &mut State) -> bool {
        self.0
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        if !flag {
            if self.0 {
                state.clone()
            } else {
                AbstractState::bottom(state)
            }
        } else {
            if self.0 {
                AbstractState::bottom(state)
            } else {
                state.clone()
            }
        }
    }
    fn to_string(&self) -> String {
        self.0.to_string()
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
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) == self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        // devo trasformare x = y in (x <= y) && (y <= x)
        // let left_op = LessEqual{left: self.left.clone_box(), right:self.right.clone_box()};
        // let right_op = LessEqual{left: self.right.clone_box(), right: self.right.clone_box()};
        // let disjunction = Box::new(And{left: Box::new(left_op), right: Box::new(right_op)});
        // disjunction.abs_evaluate(state, flag)

        if !flag {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
                (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
                    if left_eval == right_eval =>
                {
                    if let Some(var_name) = self.left.as_variable() {
                        state.update_interval(&var_name.value, left_eval.clone());
                        state.clone()
                    } else {
                        unreachable!("Left operand of == must be a variable!");
                    }
                }
                _ => AbstractState::bottom(state),
            }
        } else {
            //eseguo operatore != se flag=true
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
                (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
                    if left_eval != right_eval =>
                {
                    // Se gli intervalli sono diversi, non restringiamo ulteriormente lo stato
                    state.clone()
                }
                _ => AbstractState::bottom(state), // Se gli intervalli sono uguali, lo stato diventa Bottom
            }
        }
    }
    fn to_string(&self) -> String {
        format!("{} = {}", self.left.to_string(), self.right.to_string())
    }
}
#[derive(Debug)]
pub struct NotEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for NotEqual {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(NotEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) != self.right.evaluate(state)
    }

    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
         // devo trasformare x != y in (x < y) || (y < x)
        // let left_op = Less{left : self.left.clone_box(), right:self.right.clone_box()};
        // let right_op = Less{left: self.right.clone_box(), right: self.left.clone_box()};
        // let union = Box::new(Or{left: Box::new(left_op), right: Box::new(right_op)});
        // union.abs_evaluate(state, flag)

        if !flag {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
                (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
                    if left_eval != right_eval =>
                {
                    // Se gli intervalli sono diversi, non restringiamo ulteriormente lo stato
                    state.clone()
                }
                _ => AbstractState::bottom(state), // Se gli intervalli sono uguali, lo stato diventa Bottom
            }
        } else {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
                (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
                    if left_eval == right_eval =>
                {
                    if let Some(var_name) = self.left.as_variable() {
                        state.update_interval(&var_name.value, left_eval.clone());
                        state.clone()
                    } else {
                        unreachable!("Left operand of == must be a variable!");
                    }
                }
                _ => AbstractState::bottom(state),
            }
        }
    }
    fn to_string(&self) -> String {
        format!("{} != {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct GreatEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for GreatEqual {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(GreatEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) >= self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        // devo trasformare x >= y in y <= x
        //  let leq = LessEqual{left: self.left.clone_box(), right:self.right.clone_box()};
        //  leq.abs_evaluate(state, flag)
        if !flag {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: l2,
                        upper: _,
                    },
                ) => {
                    if let Some(var_name) = self.left.as_variable() {
                        if u1 >= l2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &var_name.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::max(l1, l2),
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
                        unreachable!("Left operand of >= must be a variable!");
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            }
        } else {
            //eseguo operatore <
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: _,
                        upper: u2,
                    },
                ) => {
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
    fn to_string(&self) -> String {
        format!("{} >= {}", self.left.to_string(), self.right.to_string())
    }
}
#[derive(Debug)]
pub struct Great {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}
impl BooleanExpression for Great {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Great {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) > self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        // transforming x > y into y+1 <= x
        // let left_op = Box::new(Add{left: self.right.clone_box(), right: Box::new(Numeral(1))});
        // let leq = LessEqual{left: left_op.clone_box(), right:self.left.clone_box()};
        // leq.abs_evaluate(state, flag)

        if !flag {
             println!("> FOUND");
             println!("> INPUT STATE : {}", state);
            if state.is_bottom() {
                println!("mi blocco qui?");
                return AbstractState::bottom(state);
            }
            //println!("dio mega merda");
            let left_eval = self.left.abs_evaluate(&mut state.clone());
            print!("speriamo che non si incastri qui ");
            let right_eval = self.right.abs_evaluate(&mut state.clone());
            print!("speriamo che non si incastri nemmeno qui ");
            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: l2,
                        upper: _,
                    },
                ) => {
                    if let Some(var_name) = self.left.as_variable() {
                        if u1 > l2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &var_name.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::max(l1, l2-1),
                                    upper: u1,
                                },
                            );
                            println!("returning state in > {}", state);
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
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => {println!("returning state {}", state ); state.clone()},
            }
        } else {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: _,
                        upper: u2,
                    },
                ) => {
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
    fn to_string(&self) -> String {
        format!("{} > {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct LessEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for LessEqual {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(LessEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) <= self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        if !flag {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: _,
                        upper: u2,
                    },
                ) => {
                    // caso x <= y (y var) 
                    if let (Some(left_var), Some(_right_var)) = (self.left.as_variable(), self.right.as_variable()) {
                        if l1 <= u2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
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
                    } else if let (Some(left_var), Some(_right_num)) = (self.left.as_variable(), self.right.as_any().downcast_ref::<Numeral>()) {
                        if l1 <= u2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
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
                        // Se il lato sinistro non è una variabile, ritorno lo stato iniziale
                        state.clone()
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            }
        } else {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: l2,
                        upper: _,
                    },
                ) => {
                     // caso x <= y (y var) 
                     if let (Some(left_var), Some(_right_var)) = (self.left.as_variable(), self.right.as_variable()) {
                        if u1 > l2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::max(l1, l2),
                                    upper: u1,
                                },
                            );
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom
                            AbstractState::bottom(state)
                        }
                    } else if let (Some(left_var), Some(_right_num)) = (self.left.as_variable(), self.right.as_any().downcast_ref::<Numeral>()) {
                        if u1 > l2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::max(l1, l2),
                                    upper: u1,
                                },
                            );
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom
                            AbstractState::bottom(state)
                        }
                    } else {
                        // Se il lato sinistro non è una variabile, ritorno lo stato iniziale
                        state.clone()
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            }
        }
    }
    fn to_string(&self) -> String {
        format!("{} <= {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Less {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for Less {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Less {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) < self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        // transforming x < y into x+1 <= y
        // let  left_op =Box::new(Add{left: self.left.clone_box(), right :Box::new(Numeral(1))});
        // let  leq = LessEqual{left: left_op, right: self.right.clone_box()};
        // leq.abs_evaluate(state, flag)


        if !flag {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: _,
                        upper: u2,
                    },
                ) => {
                    if let Some(var_name) = self.left.as_variable() {
                        if l1 < u2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &var_name.value,
                                AbstractInterval::Bounded {
                                    lower: l1,
                                    upper: std::cmp::min(u1, u2-1),
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
        } else {
            if state.is_bottom() {
                return AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: l2,
                        upper: _,
                    },
                ) => {
                    if let Some(var_name) = self.left.as_variable() {
                        if u1 >= l2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &var_name.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::max(l1, l2),
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
                        unreachable!("Left operand of >= must be a variable!");
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            }
        }
    }
    fn to_string(&self) -> String {
        format!("{} < {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct And {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for And {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(And {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        if !flag {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);

            if left_eval.is_bottom || right_eval.is_bottom {
                return AbstractState::bottom(state);
            }
            let mut new_variables = HashMap::new();

            for key in left_eval
                .variables
                .keys()
                .chain(right_eval.variables.keys())
            {
                // retrieve the left and right interval from the state
                let left_interval = left_eval.variables.get(key);
                let right_interval = right_eval.variables.get(key);
                //combine them using the interval intersection
                let intersec_interval = match (left_interval, right_interval) {
                    (Some(l), Some(r)) => l.value.intersect(&r.value),
                    (Some(l), None) => l.value.clone(),
                    (None, Some(r)) => r.value.clone(),
                    (None, None) => AbstractInterval::Top,
                };

                new_variables.insert(key.clone(), AbstractDomain::new(intersec_interval));
            }

            AbstractState {
                is_bottom: false,
                variables: new_variables,
            }
        } else {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);

            if left_eval.is_bottom || right_eval.is_bottom {
                return AbstractState::bottom(state);
            }
            let mut new_variables = HashMap::new();
            for key in left_eval
                .variables
                .keys()
                .chain(right_eval.variables.keys())
            {
                let left_interval = left_eval.variables.get(key);
                let right_interval = right_eval.variables.get(key);

                let union_interval = match (left_interval, right_interval) {
                    (Some(l), Some(r)) => l.value.int_lub(&r.value),
                    (Some(l), None) => l.value.clone(),
                    (None, Some(r)) => r.value.clone(),
                    (None, None) => AbstractInterval::Top,
                };
                new_variables.insert(key.clone(), AbstractDomain::new(union_interval));
            }
            AbstractState {
                is_bottom: false,
                variables: new_variables,
            }
        }
    }
    fn to_string(&self) -> String {
        format!("{} && {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Or {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Or {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Or {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) || self.right.evaluate(state)
    }
    fn abs_evaluate(&self, state: &mut AbstractState, flag: bool) -> AbstractState {
        if !flag {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);

            if left_eval.is_bottom || right_eval.is_bottom {
                return AbstractState::bottom(state);
            }
            let mut new_variables = HashMap::new();
            for key in left_eval
                .variables
                .keys()
                .chain(right_eval.variables.keys())
            {
                let left_interval = left_eval.variables.get(key);
                let right_interval = right_eval.variables.get(key);

                let union_interval = match (left_interval, right_interval) {
                    (Some(l), Some(r)) => l.value.int_lub(&r.value),
                    (Some(l), None) => l.value.clone(),
                    (None, Some(r)) => r.value.clone(),
                    (None, None) => AbstractInterval::Top,
                };
                new_variables.insert(key.clone(), AbstractDomain::new(union_interval));
            }
            AbstractState {
                is_bottom: false,
                variables: new_variables,
            }
        }
        else {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);

            if left_eval.is_bottom || right_eval.is_bottom {
                return AbstractState::bottom(state);
            }
            let mut new_variables = HashMap::new();

            for key in left_eval
                .variables
                .keys()
                .chain(right_eval.variables.keys())
            {
                // retrieve the left and right interval from the state
                let left_interval = left_eval.variables.get(key);
                let right_interval = right_eval.variables.get(key);
                //combine them using the interval intersection
                let intersec_interval = match (left_interval, right_interval) {
                    (Some(l), Some(r)) => l.value.intersect(&r.value),
                    (Some(l), None) => l.value.clone(),
                    (None, Some(r)) => r.value.clone(),
                    (None, None) => AbstractInterval::Top,
                };

                new_variables.insert(key.clone(), AbstractDomain::new(intersec_interval));
            }

            AbstractState {
                is_bottom: false,
                variables: new_variables,
            }
        }
    }
    fn to_string(&self) -> String {
        format!("{} || {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Not {
    pub expression: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Not {
    fn clone_box(&self) -> Box<dyn BooleanExpression> {
        Box::new(Not {
            expression: self.expression.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        !(self.expression.evaluate(state))
    }
    fn abs_evaluate(&self, state: &mut AbstractState, _flag:bool) -> AbstractState {
        let expr_eval = self.expression.abs_evaluate(state, false);

        if expr_eval.is_bottom() {
            return AbstractState::bottom(state);
        }

        let mut new_variables = HashMap::new();
        for key in expr_eval.variables.keys() {
            let expr_interval = expr_eval.variables.get(key);

            let neg_interval = match expr_interval {
                Some(i) => -(i.value),
                None => AbstractInterval::Top,
            };

            new_variables.insert(key.clone(), AbstractDomain::new(neg_interval));
        }
    
        let negated_state = AbstractState {
            is_bottom: false,
            variables: new_variables,
        };
        state.state_lub(&negated_state)
    }

    fn to_string(&self) -> String {
        format!("! {}", self.expression.to_string())
    }
}
