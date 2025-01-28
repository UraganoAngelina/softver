use num_traits::Zero;
use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::M;
use crate::N;

pub trait AbstractDomainOps {
    fn partial_order(&self, other: &Self) -> bool;
    fn bottom() -> Self;
    fn top() -> Self;

    // Operazioni di unione (lub) e intersezione (glb)
    fn lub(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;

    fn widening(&self, other: &Self) -> Self;
    fn narrowing(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum AbstractInterval {
    Bottom,                             // Stuck configuration
    Top,                                // Lack of information
    Bounded { lower: i64, upper: i64 }, // Regular Interval
}

impl AbstractDomainOps for AbstractInterval {
    // Operazione di ordine parziale
    fn partial_order(&self, other: &Self) -> bool {
        match (self, other) {
            (
                AbstractInterval::Bounded {
                    lower: a1,
                    upper: b1,
                },
                AbstractInterval::Bounded {
                    lower: a2,
                    upper: b2,
                },
            ) => a1 <= a2 && b1 >= b2,
            (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => true, // Empty è sempre sotto
            (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => true, // Top è sempre sopra
        }
    }

    // Elemento bottom
    fn bottom() -> Self {
        AbstractInterval::Bottom
    }

    // Elemento top
    fn top() -> Self {
        AbstractInterval::Top
    }

    // Unione (lub)
    fn lub(&self, other: &Self) -> Self {
        // match (self, other) {
        //     (
        //         AbstractInterval::Bounded {
        //             lower: a1,
        //             upper: b1,
        //         },
        //         AbstractInterval::Bounded {
        //             lower: a2,
        //             upper: b2,
        //         },
        //     ) => AbstractInterval::Bounded {
        //         lower: a1.clone().min(a2.clone()),
        //         upper: b1.clone().max(b2.clone()),
        //     },
        //     (AbstractInterval::Bottom, other) | (other, AbstractInterval::Bottom) => other.clone(),
        //     (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => AbstractInterval::Top,
        // }
        self.int_lub(other)
    }

    // Intersezione (glb)
    fn glb(&self, other: &Self) -> Self {
        // match (self, other) {
        //     (
        //         AbstractInterval::Bounded {
        //             lower: a1,
        //             upper: b1,
        //         },
        //         AbstractInterval::Bounded {
        //             lower: a2,
        //             upper: b2,
        //         },
        //     ) => AbstractInterval::Bounded {
        //         lower: a1.clone().max(a2.clone()),
        //         upper: b1.clone().min(b2.clone()),
        //     },
        //     (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
        //         AbstractInterval::Bottom
        //     }
        //     (AbstractInterval::Top, other) | (other, AbstractInterval::Top) => other.clone(),
        // }
        self.intersect(other)
    }

    fn widening(&self, other: &Self) -> Self {
        // Esegui una logica di widening, ad esempio un'unione di intervalli
        self.int_widening(other)
    }

    // Narrowing
    fn narrowing(&self, other: &Self) -> Self {
        // Esegui una logica di narrowing, ad esempio una intersezione di intervalli
        self.int_narrowing(other)
    }
}

impl fmt::Display for AbstractInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbstractInterval::Bottom => write!(f, "Bottom ┴"),
            AbstractInterval::Top => write!(f, "Top ┬"),
            AbstractInterval::Bounded { lower, upper } => write!(f, "[ {}, {}]", lower, upper),
        }
    }
}

impl AbstractInterval {
    pub fn new_top() -> Self {
        Self::Top
    }
    /// Crea un intervallo con estremi definiti
    pub fn new(lower: i64, upper: i64) -> Self {
        if lower > upper {
            Self::Bottom
        } else {
            Self::Bounded { lower, upper }
        }
    }

    /// Interval Least Upper Bound
    pub fn int_lub(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    Self::Bounded {
                        lower: *l1.min(l2),
                        upper: *u1.max(u2),
                    }
                }
            }
        }
    }

    // Interval widening
    pub fn int_widening(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let new_lower = if l1 <= &l2 {
                        *l1
                    } else if *l2 <= i64::zero() && l1 < &l2 {
                        i64::zero()
                    } else {
                        m
                    };
                    let new_upper = if u1 >= &u2 {
                        *u1
                    } else if *u1 <= i64::zero() && u1 > &u2 {
                        i64::zero()
                    } else {
                        n
                    };
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }

    // Interval narrowing
    pub fn int_narrowing(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, x) | (x, Self::Top) => x.clone(),
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                //dovrei controllare il caso in cui il mio M sia == N
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let new_lower = if m >= *l1 { *l2 } else { *l1 };
                    let new_upper = if n <= *u1 { *u2 } else { *u1 };
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }

    /// Intersezione di due intervalli
    pub fn intersect(&self, other: &Self) -> Self {
        // Pattern matching per gestire i casi
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => {
                //println!("Returning Bottom due to Self::Bottom or other::Bottom");
                Self::Bottom
            }
            (Self::Top, x) | (x, Self::Top) => {
                //println!("Returning clone of other (Top case)");
                x.clone()
            }
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                //println!("Handling bounded case");

                // Acquisisci i valori dei mutex una sola volta
                let m_val = *M.lock().expect("Failed to lock M");
                let n_val = *N.lock().expect("Failed to lock N");

                //println!("m_val: {}, n_val: {}", m_val, n_val);

                // Confronta e calcola il nuovo intervallo
                if m_val == n_val {
                    //println!("Bounds are equal, returning new bounded interval with single value");
                    Self::Bounded {
                        lower: m_val,
                        upper: m_val,
                    }
                } else {
                    let new_lower = *l1.max(l2);
                    let new_upper = *u1.min(u2);

                    // println!(
                    //     "Calculated new bounds: new_lower = {}, new_upper = {}",
                    //     new_lower, new_upper
                    // );

                    if new_lower <= new_upper {
                        //println!("Valid bounded interval, returning it");
                        Self::Bounded {
                            lower: new_lower,
                            upper: new_upper,
                        }
                    } else {
                        //println!("Invalid bounds, returning Bottom");
                        Self::Bottom
                    }
                }
            }
        }
    }
    pub fn is_top(&self) -> bool {
        match self {
            Self::Top => true,
            _ => false,
        }
    }
    pub fn is_bottom(&self) -> bool {
        match self {
            Self::Top => true,
            _ => false,
        }
    }
}

impl PartialEq for AbstractInterval {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bottom, Self::Bottom) => true,
            (Self::Top, Self::Top) => true,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => l1 == l2 && u1 == u2,
            _ => false,
        }
    }
}

impl PartialOrd for AbstractInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Top) => Some(std::cmp::Ordering::Less),
            (_, Self::Bottom) | (Self::Top, _) => Some(std::cmp::Ordering::Greater),
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                if l1 >= l2 && u1 <= u2 {
                    Some(std::cmp::Ordering::Less) // `self` è contenuto in `other`
                } else if l1 <= l2 && u1 >= u2 {
                    Some(std::cmp::Ordering::Greater) // `other` è contenuto in `self`
                } else {
                    None
                }
            }
        }
    }
}

impl Add for AbstractInterval {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    Self::Bounded {
                        lower: checked_add(l1, l2),
                        upper: checked_add(u1, u2),
                    }
                }
            }
        }
    }
}

fn checked_add(a: i64, b: i64) -> i64 {
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_add(b) {
        Some(result) => result,
        None if a > i64::zero() => n,
        None => m,
    }
}

impl Sub for AbstractInterval {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let candidates = [
                        checked_sub(l1, l2),
                        checked_sub(l1, u2),
                        checked_sub(u1, l2),
                        checked_sub(u1, u2),
                    ];

                    let new_lower = candidates
                        .iter()
                        .filter_map(|&x| Some(x)) // Filtra i risultati validi
                        .min() // Trova il valore minimo
                        .unwrap_or(min_value()); // Se tutto fallisce, ritorna il valore minimo

                    let new_upper = candidates
                        .iter()
                        .filter_map(|&x| Some(x)) // Filtra i risultati validi
                        .max() // Trova il valore massimo
                        .unwrap_or(max_value()); // Se tutto fallisce, ritorna il valore massimo

                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }
}

fn checked_sub(a: i64, b: i64) -> i64 {
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_sub(b) {
        Some(result) => result,
        None if a > i64::zero() => n,
        None => m,
    }
}

impl Mul for AbstractInterval {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let candidates = [
                        checked_mul(l1, l2),
                        checked_mul(l1, u2),
                        checked_mul(u1, l2),
                        checked_mul(u1, u2),
                    ];

                    let new_lower = candidates
                        .iter()
                        .filter_map(|&x| Some(x)) // Filtra i risultati validi
                        .min() // Trova il valore minimo
                        .unwrap_or(min_value()); // Se tutto fallisce, ritorna il valore minimo

                    let new_upper = candidates
                        .iter()
                        .filter_map(|&x| Some(x)) // Filtra i risultati validi
                        .max() // Trova il valore massimo
                        .unwrap_or(max_value()); // Se tutto fallisce, ritorna il valore massimo

                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }
}

/// Funzione helper per effettuare una moltiplicazione controllata
fn checked_mul(a: i64, b: i64) -> i64 {
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_mul(b) {
        Some(result) => result,
        None if a > i64::zero() => n,
        None => m,
    } // Usa il metodo built-in per tipi numerici nativi
}

/// Function to obtain the global values M (⊥) and N (⊤)
fn min_value() -> i64 {
    let m = *M.lock().unwrap();
    m
}

fn max_value() -> i64 {
    let n = *N.lock().unwrap();
    n
}

impl Div for AbstractInterval {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let m =*M.lock().unwrap();
        let n = *N.lock().unwrap();
        if m==n {
            Self::Bounded { lower: m, upper: m }
        }
        else {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom, // ⊥ / qualsiasi = ⊥
            (Self::Top, Self::Top) => Self::Top, // ⊤ / ⊤ = ⊤
            (Self::Top, Self::Bounded { lower, upper }) if lower == 0 && upper == 0 => Self::Bottom, // ⊤ / [0,0] = ⊥ ✅
            (Self::Top, _) => Self::Top, // ⊤ / qualsiasi ≠ [0,0] = ⊤ ✅
            (_, Self::Top) => Self::Top, // qualsiasi / ⊤ = ⊤ ✅
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                if l2 == 0 && u2 == 0 {
                    return Self::Bottom; // [l,u] / [0,0] = ⊥ ✅
                }

                if l2 < 0 && u2 > 0 {
                    return Self::Top; // [l,u] / [l',u'] con l' < 0 < u' = ⊤ ✅
                }

                let candidates = [
                    checked_div(l1, l2),
                    checked_div(l1, u2),
                    checked_div(u1, l2),
                    checked_div(u1, u2),
                ];

                let new_lower = candidates
                    .iter()
                    .filter_map(|&x| Some(x)) // Rimuove gli Option::None
                    .min()
                    .unwrap_or(m);

                let new_upper = candidates
                    .iter()
                    .filter_map(|&x| Some(x))
                    .max()
                    .unwrap_or(n);

                Self::Bounded {
                    lower: new_lower,
                    upper: new_upper,
                }
            }
        }
    }
    }
}
fn checked_div(a: i64, b: i64) -> i64 {
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_div(b) {
        Some(result) => result,
        None if a > i64::zero() => n,
        None => m,
    } // Usa il metodo built-in per tipi numerici nativi
}

impl Neg for AbstractInterval {
    type Output = Self;

    fn neg(self) -> AbstractInterval {
        match self {
            AbstractInterval::Bottom => AbstractInterval::Bottom,
            AbstractInterval::Top => AbstractInterval::Top,
            AbstractInterval::Bounded { lower, upper } => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    AbstractInterval::Bounded {
                        lower: -upper,
                        upper: -lower,
                    }
                }
            }
        }
    }
}

impl Ord for AbstractInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (
                AbstractInterval::Bounded {
                    lower: a1,
                    upper: b1,
                },
                AbstractInterval::Bounded {
                    lower: a2,
                    upper: b2,
                },
            ) => a1.cmp(a2).then(b1.cmp(b2)),
            // `Bottom` è sempre considerato più piccolo di qualsiasi altro intervallo
            (AbstractInterval::Bottom, _) => Ordering::Less,
            (_, AbstractInterval::Bottom) => Ordering::Greater,

            // `Top` è sempre considerato più grande di qualsiasi altro intervallo
            (AbstractInterval::Top, _) => Ordering::Greater,
            (_, AbstractInterval::Top) => Ordering::Less,
        }
    }
}
