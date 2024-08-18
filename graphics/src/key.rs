use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum KeyState {
    Press,
    Kept,
    Release,
}

impl KeyState {
    pub fn is_pressing(&self) -> bool {
        *self == KeyState::Press || *self == KeyState::Kept
    }
}

pub struct KeyStateMap {
    key_states: HashMap<String, KeyState>,
}

impl KeyStateMap {
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, state: KeyState) {
        self.key_states.insert(key, state);
    }

    pub fn get(&self, key: &str) -> Option<&KeyState> {
        self.key_states.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &KeyState)> {
        self.key_states.iter()
    }

    pub fn update(&mut self) {
        self.key_states.iter_mut().for_each(|(_, state)| {
            if *state == KeyState::Press {
                *state = KeyState::Kept;
            }
        });
        self.key_states
            .retain(|_, state| *state != KeyState::Release);
    }

    pub fn purge(&mut self) {
        self.key_states.iter_mut().for_each(|(_, state)| {
            *state = KeyState::Release;
        });
    }
}
