use std::collections::HashMap;
use std::fmt;

use crate::abstract_domain::AbstractInterval;

#[derive(Debug, PartialEq)]
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
        // println!("LUB STATE 1 {}", self);
        // println!("LUB STATE 2 {}", other);

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
        let newstate=AbstractState {
            is_bottom: false, // Lo stato risultante non è Bottom
            variables: new_variables,
        };
        //println!("LUB RETURN  {}", newstate);
        newstate
    }

    pub fn state_widening(&self, other : &AbstractState) -> AbstractState {
         // Se uno dei due stati è Bottom, ritorna l'altro stato
         if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }
        // println!("WIDENING STATE 1 {}", self);
        // println!("WIDENING STATE 2 {}", other);

        // Se uno e' Top, ritorniamo uno stato Top
        if self.is_top() || other.is_top() {
            return AbstractState {
                is_bottom: false,
                variables: HashMap::new(), 
            };
        }

        let mut new_variables: HashMap<String, AbstractInterval<i64>> = HashMap::new();

        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Calcoliamo il widening degli intervalli per ogni variabile
                new_variables.insert(key.clone(), left_interval.int_widening(right_interval));
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
        let newstate=AbstractState {
            is_bottom: false, // Lo stato risultante non è Bottom
            variables: new_variables,
        };
        //println!("WIDENING RETURN {}", newstate);
        newstate
    }

    pub fn state_narrowing(&self, other : &AbstractState) -> AbstractState{
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }
        // println!("NARROWING STATE 1 {}", self);
        // println!("NARROWING STATE 2 {}", other);

        // Se uno e' Top, ritorniamo uno stato Top
        if self.is_top() || other.is_top() {
            return AbstractState {
                is_bottom: false,
                variables: HashMap::new(), 
            };
        }

        let mut new_variables: HashMap<String, AbstractInterval<i64>> = HashMap::new();

        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Calcoliamo il narrowing degli intervalli per ogni variabile
                new_variables.insert(key.clone(), left_interval.int_narrowing(right_interval));
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
        let newstate= AbstractState {
            is_bottom: false, // Lo stato risultante non è Bottom
            variables: new_variables,
        };
        //println!("NARROWING RETURN {}", newstate);
        newstate
    }
}

impl fmt::Display for AbstractState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_bottom {
            // Stato è ⊥ (bottom)
            write!(f, "Bottom ⊥")
        } else {
            // Stato normale: stampiamo le variabili con i loro intervalli
            let variables_str: Vec<String> = self
                .variables
                .iter()
                .map(|(var, interval)| format!("{}: {}", var, interval))
                .collect();
            write!(f, "{{ {} }}", variables_str.join(", "))
        }
    }
}

impl Clone for AbstractState {
    fn clone(&self) -> Self {
        Self {
            is_bottom: self.is_bottom,
            variables: self.variables.clone(),
        }
    }
}