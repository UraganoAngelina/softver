use crate::abstract_domain::AbstractDomainOps;
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use crate::{abstract_interval::AbstractInterval, abstract_state::AbstractState};
use std::collections::HashMap;
use std::fmt::Debug;

use super::arithmetic::{Add, Minus, Numeral};
use crate::M;
// use super::{BooleanAST, RelOp};

pub trait BooleanExpression: Debug {
    type Q: AbstractDomainOps + PartialEq + Clone + Debug;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>>;
    fn evaluate(&self, state: &mut State) -> bool;
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q>;
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct Boolean(pub bool);

impl BooleanExpression for Boolean {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Boolean(self.0)) // Crea un nuovo Box con una copia di Numeral
    }
    fn evaluate(&self, _state: &mut State) -> bool {
        self.0
    }
    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>, flag: bool) -> AbstractState<Self::Q>
    where
        Self::Q: AbstractDomainOps + Clone + PartialEq,
    {
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
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Equal {
    type Q = AbstractInterval;
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) == self.right.evaluate(state)
    }

    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Equal {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }

    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state)
        } else {
            //(a - b <= 0) and (b - a <=0)
            let left = Box::new(LessEqual {
                left: Box::new(Minus {
                    left: self.left.clone_box(),
                    right: self.right.clone_box(),
                }),
                right: Box::new(Numeral(0)),
            });
            let right = Box::new(LessEqual {
                left: Box::new(Minus {
                    left: self.right.clone_box(),
                    right: self.left.clone_box(),
                }),
                right: Box::new(Numeral(0)),
            });
            let canonical = Box::new(And { left, right });
            canonical.abs_evaluate(state, flag)
        }
    }
    fn to_string(&self) -> String {
        format!("{} = {}", self.left.to_string(), self.right.to_string())
    }
}
#[derive(Debug)]
pub struct NotEqual {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for NotEqual {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(NotEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) != self.right.evaluate(state)
    }

    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        // (b - a + 1 <=0) or (a - b + 1 <=0)
        let left = Box::new(LessEqual {
            left: Box::new(Add {
                left: Box::new(Minus {
                    left: self.right.clone_box(),
                    right: self.left.clone_box(),
                }),
                right: Box::new(Numeral(1)),
            }),
            right: Box::new(Numeral(0)),
        });
        let right = Box::new(LessEqual {
            left: Box::new(Add {
                left: Box::new(Minus {
                    left: self.left.clone_box(),
                    right: self.right.clone_box(),
                }),
                right: Box::new(Numeral(1)),
            }),
            right: Box::new(Numeral(0)),
        });
        let canonical = Box::new(Or { left, right });
        canonical.abs_evaluate(state, flag)
    }
    fn to_string(&self) -> String {
        format!("{} != {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct GreatEqual {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for GreatEqual {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(GreatEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) >= self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state);
        }

       if ! flag{
         // b - a  <= 0
         let lhs = Box::new(Minus {
            left: self.right.clone_box(),
            right: self.left.clone_box(),
        });

        let rhs = Box::new(Numeral(0));
        let canonical = Box::new(LessEqual {
            left: lhs,
            right: rhs,
        });
        canonical.abs_evaluate(state, flag)
       }
       else {
           let lth = Box::new(Less{left: self.left.clone_box(), right: self.right.clone_box()});
           lth.abs_evaluate(state, !flag)
       }
    }
    fn to_string(&self) -> String {
        format!("{} >= {}", self.left.to_string(), self.right.to_string())
    }
}
#[derive(Debug)]
pub struct Great {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}
impl BooleanExpression for Great {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Great {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) > self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        _flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state);
        }

        if !_flag {
            // qui voglio che sia evaluato come un > in senso stretto e quindi trasformato in <=
            let lhs = Box::new(Add {
                left: Box::new(Minus {
                    left: self.right.clone_box(),
                    right: self.left.clone_box(),
                }),
                right: Box::new(Numeral(1)),
            });
            let rhs = Box::new(Numeral(0));
            let canonical = Box::new(LessEqual {
                left: lhs,
                right: rhs,
            });

            canonical.abs_evaluate(state, false)
        } else {
           // println!("great true flag");
            // qui voglio che sia evaluato come un minore
            let lth = Box::new(LessEqual {
                left: self.left.clone_box(),
                right: self.right.clone_box(),
            });
            lth.abs_evaluate(state, ! _flag)
        }
    }
    fn to_string(&self) -> String {
        format!("{} > {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct LessEqual {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for LessEqual {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(LessEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) <= self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if !flag {
            // println!(
            //     "less equal {} <= {} normal eval in state {}",
            //     self.left.to_string(),
            //     self.right.to_string(),
            //     state
            // );
            if self.left.abs_evaluate(state).is_bottom()
                || self.right.abs_evaluate(state).is_bottom()
            {
                AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);
            // println!(
            //     "evaluation lhs: {} rhs:{}",
            //     left_eval.to_string(),
            //     right_eval.to_string()
            // );

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    // println!("bottom case of leq");
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
                        upper: u2,
                    },
                ) => {
                    // caso x <= y (y var)
                    if let (Some(left_var), Some(_right_var)) =
                        (self.left.as_variable(), self.right.as_variable())
                    {
                        if l1 <= u2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
                                AbstractInterval::Bounded {
                                    lower: l1,
                                    upper: std::cmp::min(u1, u2),
                                },
                            );
                            state.update_interval(
                                &_right_var.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::min(l1, l2),
                                    upper: u1,
                                },
                            );
                            // println!(" leq result {}", state.clone());
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom

                            state.update_interval(&left_var.value, AbstractInterval::Bottom);
                            // println!(" leq result {}", state.clone());
                            AbstractState::bottom(state)
                        }
                    } else if let (Some(left_var), Some(_right_num)) = (
                        self.left.as_variable(),
                        self.right.as_any().downcast_ref::<Numeral>(),
                    ) {
                        if l1 <= u2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
                                AbstractInterval::Bounded {
                                    lower: l1,
                                    upper: std::cmp::min(u1, u2),
                                },
                            );
                            // println!(" leq result {}", state.clone());
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom
                            state.update_interval(&left_var.value, AbstractInterval::Bottom);
                            // println!(" leq result {}", state.clone());
                            AbstractState::bottom(state)
                        }
                    } else {
                        // If the lhs is not a straight variable execute the propagation algorithm
                        let m: i64 = *M.lock().expect("failed to lock m mutex");
                        let mut var_leaves = HashMap::new();
                        // Build the AST representation of the test (canonical form lhs <=0)
                        let tree = self.left.to_ast(state, &mut var_leaves);
                        // Find the refinement intersecting with [-∞; 0]
                        let refinement = tree
                            .get_value()
                            .intersect(&AbstractInterval::Bounded { lower: m, upper: 0 });
                        // Propagate the refinement found with the backward operators
                        let sat = tree.backward_analysis(refinement);
                        if !sat {
                            return AbstractState::bottom(&state);
                        }
                        //Update the real state
                        var_leaves.iter().for_each(|(var, node)| {
                            state.update_interval(var, *node);
                        });
                        
                        let new_state = state.clone();
                        new_state
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => {
                    // println!(" top case res{}", state.clone());
                    state.clone()
                }
            }
        } else {
            // println!("filtering with !guard in leq in state {}", state);
            if self.left.abs_evaluate(state).is_bottom()
                || self.right.abs_evaluate(state).is_bottom()
            {
                // println!(" bottom case in leq negated");
                AbstractState::bottom(state);
            }
            // println!(
            //     "lhs  rhs {} > {} in final filtering",
            //     self.left.to_string(),
            //     self.right.to_string()
            // );
            if state.is_bottom() {
                return AbstractState::bottom(&state);
            }
            let gr = Great {
                left: self.left.clone_box(),
                right: self.right.clone_box(),
            };
            let my_flag = !flag;
            gr.abs_evaluate(state, my_flag)
        }
    }
    fn to_string(&self) -> String {
        format!("{} <= {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Less {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Less {
    type Q = AbstractInterval;
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) < self.right.evaluate(state)
    }

    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Less {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }

    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state);
        }
        // println!("Interpreting {:?}", self);
        // println!("In the abstract state {}", state);
        let zero_int = AbstractInterval::Bounded { lower: 0, upper: 0 };
        if !flag {
            if self.right.abs_evaluate(state) == zero_int {
                let lhs = Box::new(Add {
                    left: self.left.clone_box(),
                    right: Box::new(Numeral(1)),
                });
                let rhs = Box::new(Numeral(0));
                let canonical = Box::new(LessEqual {
                    left: lhs,
                    right: rhs,
                });
                canonical.abs_evaluate(state, flag)
            } else {
                let lhs = Box::new(Add {
                    left: Box::new(Minus {
                        left: self.left.clone_box(),
                        right: self.right.clone_box(),
                    }),
                    right: Box::new(Numeral(1)),
                });
                let rhs = Box::new(Numeral(0));
                let canonical = Box::new(LessEqual {
                    left: lhs,
                    right: rhs,
                });
                canonical.abs_evaluate(state, flag)
            }
        }
        else {
            let geq = Box::new(GreatEqual{left: self.left.clone_box(), right: self.right.clone_box()});
            geq.abs_evaluate(state, !flag)
        }
    }
    fn to_string(&self) -> String {
        format!("{} < {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct And {
    pub left: Box<dyn BooleanExpression<Q = AbstractInterval>>,
    pub right: Box<dyn BooleanExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for And {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(And {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if !flag {
            let mut x = state.clone();
            let left_eval = self.left.abs_evaluate(&mut x, false);
            let right_eval = self.right.abs_evaluate(&mut x, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let current = left_eval.state_glb(&right_eval);
            state.variables.extend(current.variables.clone());
            current
        } else {
            let mut x = state.clone();

            let left_eval = self.left.abs_evaluate(&mut x, false);
            let right_eval = self.right.abs_evaluate(&mut x, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let current = left_eval.state_lub(&right_eval);
            state.variables.extend(current.variables.clone());
            current
        }
    }
    fn to_string(&self) -> String {
        format!("{} && {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Or {
    pub left: Box<dyn BooleanExpression<Q = AbstractInterval>>,
    pub right: Box<dyn BooleanExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Or {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Or {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) || self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if !flag {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let ret = left_eval.state_lub(&right_eval);

            state.variables.extend(ret.variables.clone());
            ret
        } else {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let ret = left_eval.state_glb(&right_eval);
            state.variables.extend(ret.variables.clone());
            ret
        }
    }
    fn to_string(&self) -> String {
        format!("{} || {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Not {
    pub expression: Box<dyn BooleanExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Not {
    type Q = AbstractInterval;
    fn evaluate(&self, state: &mut State) -> bool {
        !(self.expression.evaluate(state))
    }

    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Not {
            expression: self.expression.clone_box(),
        })
    }

    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.expression.abs_evaluate(state, flag).is_bottom() {
            AbstractState::bottom(state);
        }
        let myflag = !flag;
        let expr_eval = self.expression.abs_evaluate(state, myflag);
        expr_eval
    }

    fn to_string(&self) -> String {
        format!("! {}", self.expression.to_string())
    }
}
