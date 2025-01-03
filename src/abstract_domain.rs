use num_traits::{Bounded, Zero, CheckedMul, CheckedAdd, CheckedDiv, CheckedSub};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialOrd;
use std::fmt;

use crate::M;
use crate::N;

#[derive(Debug, Clone, Copy)]
pub enum AbstractInterval<T> {
    Bottom,                  // Stuck configuration
    Top,                     // Lack of information
    Bounded {  lower: T,  upper: T }, // Regular Interval
}

impl<T: fmt::Display> fmt::Display for AbstractInterval<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbstractInterval::Bottom => write!( f, "Bottom ┴"),
            AbstractInterval::Top => write!(f, "Top ┬"),
            AbstractInterval::Bounded { lower, upper } => write!(f, "[ {}, {}]", lower, upper),
        }
    }
}


impl<T> AbstractInterval<T>
where
    T: PartialOrd + Copy + Ord + From<i64> + Bounded + Zero,
{
    /// Crea un intervallo con estremi definiti
    pub fn new(lower: T, upper: T) -> Self {
        if lower > upper {
            Self::Bottom
        } else {
            Self::Bounded { lower, upper }
        }
    }

    /// Unione di due intervalli
    pub fn int_lub(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                Self::Bounded {
                    lower: *l1.min(l2),
                    upper: *u1.max(u2),
                }
            }
        }
    }
   
    //widening di due intervalli
    pub fn int_widening( &self , other: &Self) -> Self{
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top , _) | (_, Self::Top) => Self::Top,
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                let new_lower = if l1<= &l2 {*l1} else if *l2<=T::zero() && l1< &l2 {T::zero()} else { <i64 as Into<T>>::into(*M.lock().unwrap())};
                let new_upper = if u1 >= &u2 {*u1} else if *u1<=T::zero() && u1 > &u2 {T::zero()} else {<i64 as Into<T>>::into(*N.lock().unwrap())};  
                Self::Bounded { lower: new_lower, upper: new_upper }
            }
        }
    }
    
    //narrowing di due intervalli s
    pub fn int_narrowing(&self, other: &Self) -> Self{
        match (self , other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, x) | (x, Self::Top)=> x.clone(),
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                let new_lower = if <i64 as Into<T>>::into(*M.lock().unwrap()) >= *l1 {*l2} else {*l1};
                let new_upper = if <i64 as Into<T>>::into(*N.lock().unwrap()) <= *u1 {*u2} else {*u1};
                Self::Bounded { lower: new_lower, upper: new_upper }
            }
        }
    }
    
    /// Intersezione di due intervalli
    pub fn intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, x) | (x, Self::Top) => x.clone(),
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                let new_lower = *l1.max(l2);
                let new_upper = *u1.min(u2);
                if new_lower <= new_upper {
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    Self::Bottom
                }
            }
        }
    }
    pub fn is_top(&self) -> bool {
        match self {
            Self::Top => true,
            _ =>  false,
        }
    }
    pub fn is_bottom(&self) -> bool{
        match self {
            Self::Top => true,
            _ => false,
        }
    }
}

impl<T: PartialEq> PartialEq for AbstractInterval<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bottom, Self::Bottom) => true,
            (Self::Top, Self::Top) => true,
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                l1 == l2 && u1 == u2
            }
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialOrd for AbstractInterval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Top) => Some(std::cmp::Ordering::Less),
            (_, Self::Bottom) | (Self::Top, _) => Some(std::cmp::Ordering::Greater),
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
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

impl<T> Add for AbstractInterval<T>
where
    T: PartialOrd + Copy + std::ops::Sub<Output = T> + std::cmp::Ord + From<i64> + Bounded + Zero + CheckedAdd,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                Self::Bounded { lower: checked_add(l1, l2), upper: checked_add(u1, u2) }
            }
        }
    }
}

fn checked_add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T> + std::cmp::Ord + CheckedAdd + Bounded + Copy + Zero + From<i64> ,
{
    match a.checked_add(&b){
        Some(result) => result,
        None if a > T::zero() => <i64 as Into<T>>::into(*N.lock().unwrap()),
        None => <i64 as Into<T>>::into(*M.lock().unwrap()),

    } 
}

impl<T> Sub for AbstractInterval<T>
where
    T: PartialOrd + Copy + std::ops::Sub<Output = T> + std::cmp::Ord + From<i64> + Bounded + Zero + CheckedSub,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
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
                    .unwrap_or(min_value::<T>()); // Se tutto fallisce, ritorna il valore minimo

                let new_upper = candidates
                    .iter()
                    .filter_map(|&x| Some(x)) // Filtra i risultati validi
                    .max() // Trova il valore massimo
                    .unwrap_or(max_value::<T>()); // Se tutto fallisce, ritorna il valore massimo

                Self::Bounded {
                    lower: new_lower,
                    upper: new_upper,
                }
            }
        }
    }
}

fn checked_sub<T>(a: T, b: T) -> T
where
    T: std::ops::Sub<Output = T> + std::cmp::Ord + CheckedSub + Bounded + Copy + Zero + From<i64>,
{
    match a.checked_sub(&b){
        Some(result) => result,
        None if a > T::zero() => <i64 as Into<T>>::into(*N.lock().unwrap()),
        None => <i64 as Into<T>>::into(*M.lock().unwrap()),

    } 
}

impl<T> Mul for AbstractInterval<T>
where
    T: PartialOrd + Copy + std::ops::Mul<Output = T> + std::cmp::Ord + From<i64> + Bounded + Zero + CheckedMul,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
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
                    .unwrap_or(min_value::<T>()); // Se tutto fallisce, ritorna il valore minimo

                let new_upper = candidates
                    .iter()
                    .filter_map(|&x| Some(x)) // Filtra i risultati validi
                    .max() // Trova il valore massimo
                    .unwrap_or(max_value::<T>()); // Se tutto fallisce, ritorna il valore massimo

                Self::Bounded {
                    lower: new_lower,
                    upper: new_upper,
                }
            }
        }
    }
}

/// Funzione helper per effettuare una moltiplicazione controllata
fn checked_mul<T>(a: T, b: T) -> T
where
    T: std::ops::Mul<Output = T> + std::cmp::Ord + CheckedMul + Bounded + Copy + Zero + From<i64>,
{
     match a.checked_mul(&b) {
        Some(result) => result,
        None if a > T::zero() => <i64 as Into<T>>::into(*N.lock().unwrap()), 
        None => <i64 as Into<T>>::into(*M.lock().unwrap()),
     } // Usa il metodo built-in per tipi numerici nativi
}

/// Funzioni per ottenere i valori minimi e massimi di un tipo
fn min_value<T>() -> T
where
    T: std::cmp::Ord + Bounded + From<i64>,
{
    <i64 as Into<T>>::into(*M.lock().unwrap())
}

fn max_value<T>() -> T
where
    T: std::cmp::Ord + Bounded + From<i64>,
{
    <i64 as Into<T>>::into(*N.lock().unwrap())
}

impl<T> Div for AbstractInterval<T>
where
    T: PartialOrd + Copy + std::ops::Div<Output = T> + From<i64>  + std::cmp::Ord  + Bounded + Zero + CheckedDiv,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (Self::Bounded { lower: l1, upper: u1 }, Self::Bounded { lower: l2, upper: u2 }) => {
                if l2 <= T::from(0) && T::from(0) <= u2 {
                    return Self::Bottom; // Errore runtime
                }
                let candidates = [
                    checked_div(l1, l2),
                    checked_div(l1 , u2),
                    checked_div(u1, l2),
                    checked_div(u1, u2), 
                ];

                let new_lower = candidates
                    .iter()
                    .filter_map(|&x| Some(x)) // Filtra i risultati validi
                    .min() // Trova il valore minimo
                    .unwrap_or(min_value::<T>()); // Se tutto fallisce, ritorna il valore minimo

                let new_upper = candidates
                    .iter()
                    .filter_map(|&x| Some(x)) // Filtra i risultati validi
                    .max() // Trova il valore massimo
                    .unwrap_or(max_value::<T>()); // Se tutto fallisce, ritorna il valore massimo

                Self::Bounded {
                    lower: new_lower,
                    upper: new_upper,
                }
            }
        }
    }
}

fn checked_div<T>(a: T, b: T) -> T
where
    T: std::ops::Div<Output = T> + std::cmp::Ord + CheckedDiv + Bounded + Copy + Zero + From<i64>,
{
     match a.checked_div(&b) {
        Some(result) => result,
        None if a > T::zero() => <i64 as Into<T>>::into(*N.lock().unwrap()), 
        None => <i64 as Into<T>>::into(*M.lock().unwrap()),
     } // Usa il metodo built-in per tipi numerici nativi
}

impl<T> Neg for AbstractInterval<T>
where
    T: PartialOrd + Copy + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> AbstractInterval<T> {
        match self {
            AbstractInterval::Bottom => AbstractInterval::Bottom,
            AbstractInterval::Top => AbstractInterval::Top,
            AbstractInterval::Bounded { lower, upper } => {
                AbstractInterval::Bounded {
                    lower: -upper,
                    upper: -lower,
                }
            }
        }
    }
}
