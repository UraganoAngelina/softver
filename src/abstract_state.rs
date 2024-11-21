use std::collections::HashMap;

use crate::abstract_domain::AbstractInterval;

#[derive(Clone, Debug)]
pub struct AbstractState {
    pub is_bottom: bool, // Indica se lo stato è ⊥ (false)
    pub variables: HashMap<String, AbstractInterval<i64>>, // Variabili astratte
}

impl AbstractState {
    /// Crea uno stato vuoto (non ⊥)
    pub fn new() -> Self {
        Self {
            is_bottom: false,
            variables: HashMap::new(),
        }
    }

    /// Crea lo stato ⊥
    pub fn bottom(mut self) -> Self {
            self.is_bottom = true;
            self
    }

    /// Controlla se lo stato è ⊥
    pub fn is_bottom(&self) -> bool {
        self.is_bottom
    }

}
