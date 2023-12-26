use std::collections::HashMap;

pub struct Plugboard<'a> {
    first: Vec<&'a char>,
    second: Vec<&'a char>
}

impl<'a> Plugboard <'a>{
    pub fn new(pairings: &'a HashMap<char, char>) -> Plugboard<'a> {
        let first = Vec::from_iter(pairings.keys());
        let second = Vec::from_iter(pairings.values());
        return Plugboard{first: first, second: second};
    }

    pub fn encode(&self, letter: char) -> char {
        if self.first.contains(&&letter) {
            let idx = match self.first.iter().position(|&x| x == &letter) {
                Some(n) => n,
                None => 0
            };
            return *self.second[idx];
        }
        else if self.second.contains(&&letter) {
            let idx = match self.second.iter().position(|&x| x == &letter) {
                Some(n) => n,
                None => 0
            };
            return *self.first[idx];
        }
        else {
            return letter
        }
    }
}


fn main() {
}