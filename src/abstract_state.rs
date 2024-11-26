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
    pub fn bottom(&self) -> AbstractState {
        AbstractState{
            is_bottom: true,
            variables: self.variables.clone()
        }
            
    }

    /// Controlla se lo stato è ⊥
    pub fn is_bottom(&self) -> bool {
        self.is_bottom
    }

    pub fn update_interval(
        &mut self,
        variable_name: &str,               // Nome della variabile
        new_interval: AbstractInterval<i64>, // Nuovo intervallo
    ) -> AbstractState {
        // Se lo stato è già bottom, restituire direttamente uno stato bottom
        if self.is_bottom() {
            return self.bottom();
        }

        // Recupera l'intervallo corrente della variabile, se esiste
        let current_interval = self
            .variables
            .get(variable_name)
            .cloned()
            .unwrap_or(AbstractInterval::Top);

        // Interseca l'intervallo corrente con quello nuovo
        let updated_interval = current_interval.intersect(&new_interval);

        // Se il risultato è Bottom, impostare lo stato a bottom
        if updated_interval.is_bottom() {
            return self.bottom();
        }

        // Aggiorna lo stato con il nuovo intervallo
        self.variables.insert(variable_name.to_string(), updated_interval);

        // Restituisce lo stato aggiornato
        self.clone()
    }

    fn is_top(&self) -> bool {
        // Se ci sono variabili, verificare se una di esse è Top
        self.variables.values().any(|interval| interval.is_top())
    }

    pub fn state_lub(&self, other: &AbstractState) -> AbstractState {
        // Se uno dei due stati è Bottom, ritorna l'altro stato (nessuna informazione aggiuntiva)
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }
        

        // Se entrambi sono Top, ritorniamo uno stato Top
        if self.is_top() || other.is_top() {
            return AbstractState {
                is_bottom: false,
                variables: HashMap::new(), // Top non ha variabili specifiche
            };
        }

        // Creiamo una nuova mappa per le variabili che contiene il lub di ogni intervallo
        let mut new_variables: HashMap<String, AbstractInterval<i64>> = HashMap::new();

        // Uniamo le variabili di entrambi gli stati
        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Calcoliamo il lub degli intervalli per ogni variabile
                new_variables.insert(key.clone(), left_interval.int_lub(right_interval));
            } else {
                // Se la variabile è presente solo in uno stato, la aggiungiamo comunque
                new_variables.insert(key.clone(), left_interval.clone());
            }
        }

        // Aggiungiamo le variabili che sono solo nell'altro stato
        for (key, right_interval) in &other.variables {
            if !self.variables.contains_key(key) {
                new_variables.insert(key.clone(), right_interval.clone());
            }
        }

        // Creiamo il nuovo stato con le variabili unite
        AbstractState {
            is_bottom: false, // Lo stato risultante non è Bottom
            variables: new_variables,
        }
    }
}
