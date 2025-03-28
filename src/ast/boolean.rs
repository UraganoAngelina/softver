use crate::abstract_domain::AbstractDomainOps;
use crate::abstract_interval::AbstractInterval;
use crate::abstract_state::AbstractState;
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use std::fmt::Debug;

use super::arithmetic::{Minus, Numeral};

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
    // fn build_ast(&self , state: &mut AbstractState<Self::Q>) ->
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
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Equal {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) == self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        // if !flag {
        //     if state.is_bottom() {
        //         return AbstractState::bottom(state);
        //     }
        //     let left_eval = self.left.abs_evaluate(state);
        //     let right_eval = self.right.abs_evaluate(state);

        //     match (left_eval, right_eval) {
        //         (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
        //             AbstractState::bottom(state)
        //         }
        //         (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        //         (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
        //             if left_eval == right_eval =>
        //         {
        //             // Se gli intervalli sono diversi, non restringiamo ulteriormente lo stato
        //             state.clone()
        //         }
        //         _ => AbstractState::bottom(state), // Se gli intervalli sono uguali, lo stato diventa Bottom
        //     }
        // } else {
        //     if state.is_bottom() {
        //         return AbstractState::bottom(state);
        //     }

        //     let left_eval = self.left.abs_evaluate(state);
        //     let right_eval = self.right.abs_evaluate(state);

        //     match (left_eval, right_eval) {
        //         (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
        //             AbstractState::bottom(state)
        //         }
        //         (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        //         (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
        //             if left_eval != right_eval =>
        //         {
        //             if let Some(var_name) = self.left.as_variable() {
        //                 state.update_interval(&var_name.value, left_eval.clone());
        //                 state.clone()
        //             } else {
        //                 unreachable!("Left operand of == must be a variable!");
        //             }
        //         }
        //         _ => AbstractState::bottom(state),
        //     }
        // }

        // if self.left.abs_evaluate(&mut state.clone()).is_bottom() || self.right.abs_evaluate(&mut state.clone()).is_bottom(){
        //     AbstractState::bottom(state);
        // }
        // let left_op = LessEqual {
        //     left: self.left.clone_box(),
        //     right: self.right.clone_box(),
        // };
        // let right_op = LessEqual {
        //     left: self.right.clone_box(),
        //     right: self.right.clone_box(),
        // };
        // let disjunction = Box::new(And {
        //     left: Box::new(left_op),
        //     right: Box::new(right_op),
        // });
        // let retstate=disjunction.abs_evaluate(state, flag);
        // println!("return state in equal test {}", retstate);
        // retstate
        // (a - b <= -1) && (b - a <= -1)
        let lleft_op = Box::new(Minus {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        });
        let rleft_op = Box::new(Minus {
            left: self.right.clone_box(),
            right: self.left.clone_box(),
        });
        let right_op = Box::new(Numeral(0));
        let lleq = Box::new(LessEqual {
            left: lleft_op.clone_box(),
            right: right_op.clone_box(),
        });
        let rleq = Box::new(LessEqual {
            left: rleft_op.clone_box(),
            right: right_op.clone_box(),
        });
        let disjunction = Box::new(And {
            left: lleq.clone_box(),
            right: rleq.clone_box(),
        });
        let ret: AbstractState<AbstractInterval> = disjunction.abs_evaluate(state, flag);
        println!("not equal return state {}", ret);
        ret
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
        // if !flag {
        //     if state.is_bottom() {
        //         return AbstractState::bottom(state);
        //     }

        //     let left_eval = self.left.abs_evaluate(state);
        //     let right_eval = self.right.abs_evaluate(state);

        //     match (left_eval, right_eval) {
        //         (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
        //             AbstractState::bottom(state)
        //         }
        //         (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        //         (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
        //             if left_eval != right_eval =>
        //         {
        //             // Se gli intervalli sono diversi, non restringiamo ulteriormente lo stato
        //             state.clone()
        //         }
        //         _ => AbstractState::bottom(state), // Se gli intervalli sono uguali, lo stato diventa Bottom
        //     }
        // } else {
        //     if state.is_bottom() {
        //         return AbstractState::bottom(state);
        //     }

        //     let left_eval = self.left.abs_evaluate(state);
        //     let right_eval = self.right.abs_evaluate(state);

        //     match (left_eval, right_eval) {
        //         (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
        //             AbstractState::bottom(state)
        //         }
        //         (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
        //         (AbstractInterval::Bounded { .. }, AbstractInterval::Bounded { .. })
        //             if left_eval == right_eval =>
        //         {
        //             if let Some(var_name) = self.left.as_variable() {
        //                 state.update_interval(&var_name.value, left_eval.clone());
        //                 state.clone()
        //             } else {
        //                 unreachable!("Left operand of != must be a variable!");
        //             }
        //         }
        //         _ => AbstractState::bottom(state),
        //     }
        // }
        // if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
        //     AbstractState::bottom(state);
        // }
        // let left_op = Less {
        //     left: self.left.clone_box(),
        //     right: self.right.clone_box(),
        // };
        // let right_op = Less {
        //     left: self.right.clone_box(),
        //     right: self.left.clone_box(),
        // };
        // let union = Box::new(Or {
        //     left: Box::new(left_op),
        //     right: Box::new(right_op),
        // });
        // let retstate = union.abs_evaluate(state, flag);
        // println!("return state in not equal test {}", retstate);
        // retstate

        // (a - b <= 0) && (b - a <= 0)
        let lleft_op = Box::new(Minus {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        });
        let rleft_op = Box::new(Minus {
            left: self.right.clone_box(),
            right: self.left.clone_box(),
        });
        let right_op = Box::new(Numeral(-1));
        let lleq = Box::new(LessEqual {
            left: lleft_op.clone_box(),
            right: right_op.clone_box(),
        });
        let rleq = Box::new(LessEqual {
            left: rleft_op.clone_box(),
            right: right_op.clone_box(),
        });
        let union = Box::new(Or {
            left: lleq.clone_box(),
            right: rleq.clone_box(),
        });
        let ret = union.abs_evaluate(state, flag);
        println!("ret expression {:?}", union);
        println!("not equal return state {}", ret);
        ret
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
        if !flag {
            println!("great equal normal eval in state {}", state);
            if self.left.abs_evaluate(state).is_bottom()
                || self.right.abs_evaluate(state).is_bottom()
            {
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
                        upper: u2,
                    },
                ) => {
                    // caso x >= y (con x variabile)
                    if let (Some(left_var), Some(_right_var)) =
                        (self.left.as_variable(), self.right.as_variable())
                    {
                        if u1 >= l2 {
                            // Aggiorniamo l'intervallo della variabile destra (speculare al caso <=)
                            state.update_interval(
                                &_right_var.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::max(l2, l1),
                                    upper: u2,
                                },
                            );
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
                            state.update_interval(&_right_var.value, AbstractInterval::Bottom);
                            AbstractState::bottom(state)
                        }
                    } else if let (Some(_left_var), Some(right_num)) = (
                        self.left.as_any().downcast_ref::<Numeral>(),
                        self.right.as_variable(),
                    ) {
                        if u1 >= l2 {
                            // Aggiorniamo l'intervallo della variabile destra
                            state.update_interval(
                                &right_num.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::max(l2, l1),
                                    upper: u2,
                                },
                            );
                            println!("returning state in geq eval {}", state);
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom
                            state.update_interval(&right_num.value, AbstractInterval::Bottom);
                            println!("returning bottom case in geq eval {}", state);
                            AbstractState::bottom(state)
                        }
                    } else {
                        println!(
                            "evaluating {} >= {} in state {}",
                            left_eval, right_eval, state
                        );
                        println!("returning top case in geq eval {}", state);
                        // Se il lato destro non è una variabile, ritorno lo stato iniziale
                        state.clone()
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            }
        } else {
            println!("filtering with !guard in geq in state {}", state);
            // if self.left.abs_evaluate(state).is_bottom()
            //     || self.right.abs_evaluate(state).is_bottom()
            // {
            //     AbstractState::bottom(state);
            // }
            if state.is_bottom() {
                return AbstractState::bottom(&state);
            }
            let leq = LessEqual {
                left: self.right.clone_box(),
                right: self.left.clone_box(),
            };
            //devo trasformare x >= y in y<=x
            // let leq = LessEqual {
            //     left: self.right.clone_box(),
            //     right: self.left.clone_box(),
            // };
            let my_flag = !flag;
            leq.abs_evaluate(state, my_flag)
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
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state);
        }
        // b - a <= -1
        let lhs = Box::new(Minus {
            left: self.right.clone_box(),
            right: self.left.clone_box(),
        });
        let rhs = Box::new(Numeral(-1));
        let leq = Box::new(LessEqual {
            left: lhs,
            right: rhs,
        });
        let ret = leq.abs_evaluate(state, flag);
        println!("great return state {}", ret);
        ret
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
            println!("less equal normal eval in state {}", state);
            if self.left.abs_evaluate(state).is_bottom()
                || self.right.abs_evaluate(state).is_bottom()
            {
                AbstractState::bottom(state);
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
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom
                            state.update_interval(&left_var.value, AbstractInterval::Bottom);
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
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom
                            state.update_interval(&left_var.value, AbstractInterval::Bottom);
                            AbstractState::bottom(state)
                        }
                    } else {
                        if let (Some(lhs), Some(rhs)) = (
                            self.left.as_any().downcast_ref::<Minus>(),
                            self.right.as_any().downcast_ref::<Numeral>(),
                        ) {
                            let c = rhs.0; // ad esempio -1

                            // Prova a estrarre le variabili dalla sottostruttura di Minus in maniera ricorsiva
                            let vars = lhs.extract_variables();
                            match vars.len() {
                                2 => {
                                    if l1 <= u2 {
                                        // Caso in cui entrambe sono variabili, gestisci come prima
                                        let var_x = vars[0];
                                        let var_y = vars[1];
                                        let new_x_upper = std::cmp::min(u1, u2 + c);
                                        // y deve essere maggiore o uguale a x_low + 1
                                        let new_y_lower = std::cmp::max(l2, l1 - c);
                                        state.update_interval(
                                            &var_x.value,
                                            AbstractInterval::Bounded {
                                                lower: l1,
                                                upper: new_x_upper,
                                            },
                                        );
                                        state.update_interval(
                                            &var_y.value,
                                            AbstractInterval::Bounded {
                                                lower: new_y_lower,
                                                upper: u1,
                                            },
                                        );
                                        return state.clone();
                                    } else {
                                        let var_x = vars[0];
                                        state.update_interval(
                                            &var_x.value,
                                            AbstractInterval::Bottom,
                                        );
                                        return AbstractState::bottom(state);
                                    }
                                }
                                1 => {
                                    if l1 <= u2 {
                                        // Caso in cui entrambe sono variabili, gestisci come prima
                                        let var_x = vars[0];
                                        let new_x_upper = std::cmp::min(u1, u2 + c);
                                        state.update_interval(
                                            &var_x.value,
                                            AbstractInterval::Bounded {
                                                lower: l1,
                                                upper: new_x_upper,
                                            },
                                        );
                                        
                                        return state.clone();
                                    } else {
                                        let var_x = vars[0];
                                        state.update_interval(
                                            &var_x.value,
                                            AbstractInterval::Bottom,
                                        );
                                        return AbstractState::bottom(state);
                                    }
                                }
                                _ => {
                                    //top case no variables found in lhs
                                    return state.clone()
                                }
                            }
                        }
                        // Se il lato sinistro non è una variabile, ritorno lo stato iniziale
                        state.clone()
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            }
        } else {
            println!("filtering with !guard in leq in state {}", state);
            // if self.left.abs_evaluate(state).is_bottom()
            //     || self.right.abs_evaluate(state).is_bottom()
            // {
            //     AbstractState::bottom(state);
            // }
            println!(
                "lhs  rhs {:?} >= {:?} in final filtering",
                self.left, self.right
            );
            if state.is_bottom() {
                return AbstractState::bottom(&state);
            }
            let geq = GreatEqual {
                left: self.left.clone_box(),
                right: self.right.clone_box(),
            };
            //devo trasformare x >= y in y<=x
            // let leq = LessEqual {
            //     left: self.right.clone_box(),
            //     right: self.left.clone_box(),
            // };
            let my_flag = !flag;
            geq.abs_evaluate(state, my_flag)
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
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Less {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) < self.right.evaluate(state)
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
        // a - b <= -1
        let lhs = Box::new(Minus {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        });
        let rhs = Box::new(Numeral(-1));
        let leq = Box::new(LessEqual {
            left: lhs,
            right: rhs,
        });
        let ret = leq.abs_evaluate(state, flag);
        println!("less return state {}", ret);
        ret
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
            // Caso And: combinazione tramite glb (intersezione)
            let mut fixpoint = false;
            let mut x = state.clone();
            let mut j = 0;
            while !fixpoint {
                j += 1;
                println!("j counter {}", j);
                // Valutiamo le due sotto-espressioni con lo stato corrente x
                let left_eval = self.left.abs_evaluate(&mut x, false);
                let right_eval = self.right.abs_evaluate(&mut x, false);
                // Combiniamo gli stati usando la funzione glb_var_wise (che fa l'intersezione variabile per variabile)
                let current = left_eval.state_glb(&right_eval);
                // Il fixpoint è raggiunto se lo stato non cambia oppure se si arriva a bottom
                fixpoint = (current == x) || (current == AbstractState::bottom(state));
                x = current;
            }
            println!("state after and evaluation {}", x);
            x
        } else {
            // Caso in cui flag è true: qui ipotizziamo l'uso di lub (unione) come nel tuo codice originale
            let mut fixpoint = false;
            let mut x = state.clone();
            while !fixpoint {
                let left_eval = self.left.abs_evaluate(&mut x, false);
                let right_eval = self.right.abs_evaluate(&mut x, false);
                let current = left_eval.state_lub(&right_eval);
                fixpoint = (current == x) || (current == AbstractState::bottom(state));
                x = current;
            }
            x
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
            // Caso in cui flag è true: qui ipotizziamo l'uso di lub (unione) come nel tuo codice originale
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let ret = left_eval.state_lub(&right_eval);

            println!("or return state {}", ret);
            ret
        } else {
            // Valutiamo le due sotto-espressioni con lo stato corrente x
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            // Combiniamo gli stati usando la funzione glb_var_wise (che fa l'intersezione variabile per variabile)
            let ret = left_eval.state_glb(&right_eval);
            // Il fixpoint è raggiunto se lo stato non cambia oppure se si arriva a bottom
            println!("negated or return state {}", ret);
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
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Not {
            expression: self.expression.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        !(self.expression.evaluate(state))
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.expression.abs_evaluate(state, flag).is_bottom() {
            AbstractState::bottom(state);
        }
        let expr_eval = self.expression.abs_evaluate(state, false);
        expr_eval
    }

    fn to_string(&self) -> String {
        format!("! {}", self.expression.to_string())
    }
}
