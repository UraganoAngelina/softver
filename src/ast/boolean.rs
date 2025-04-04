use crate::abstract_domain::AbstractDomainOps;
use crate::{abstract_interval::AbstractInterval, abstract_state::AbstractState};
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use std::fmt::Debug;

use super::arithmetic::{Add, Minus, Numeral};

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

        // b - a  <= 0
        let lhs = Box::new(Minus{left: self.right.clone_box(), right: self.left.clone_box()});
         
        let rhs = Box::new(Numeral(0));
        let canonical = Box::new(LessEqual {
            left: lhs,
            right: rhs,
        });
        canonical.abs_evaluate(state, flag)
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

        // b - a + 1 <= 0
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
        canonical.abs_evaluate(state, flag)
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
                        // Se il lato sinistro non è una variabile, ritorno lo stato iniziale
                        state.clone()
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => state.clone(),
            }
        } else {
            println!("filtering with !guard in leq in state {}", state);
            if self.left.abs_evaluate(state).is_bottom()
                || self.right.abs_evaluate(state).is_bottom()
            {
                AbstractState::bottom(state);
            }
            println!(
                "lhs  rhs {:?} >= {:?} in final filtering",
                self.left, self.right
            );
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
        let zero_int = AbstractInterval::Bounded { lower: 0, upper: 0 };
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
