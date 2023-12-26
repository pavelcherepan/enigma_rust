use std::collections::VecDeque;

pub struct Rotor<'a> {
    pub name: &'a str,
    pub wiring: VecDeque<char>,
    pub turnover: Vec<char>,
    total_turns: usize,
    pub turn: bool,
}

impl<'a> Rotor<'a>{    
    const ALPHABET: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
                                  'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
                                  'X', 'Y', 'Z'];
    pub fn new(
        name: &'a str, 
        wiring: &'a str, 
        starting_pos: usize, 
        turnover: &'a str) -> Rotor<'a> {
        // create a deque from keys of the 
        let mut current_pos = VecDeque::from_iter(wiring.chars());
        // create a vector if turnovers since for some rotors we can have more than one notch
        let turn = Vec::from_iter(turnover.chars());
        // rotate current_pos to set it to desired starting position
        current_pos.rotate_left(starting_pos);
        return Rotor{name: name, wiring: current_pos, turnover: turn, total_turns: 0, turn: false};
    }

    pub fn rotate(&mut self, n: usize) {
        self.wiring.rotate_left(n);
    }

    pub fn forward(&mut self, letter: char) -> char {
        /* Encode a letter using the defined wiring on forward pass.
        Before encoding starts, rotate the rotor by 1 position*/
        if self.turn {
            self.rotate(1);
            self.total_turns = (self.total_turns + 1) % 26; // store total turns as mod 26
        }
        // capitalize the letter
        let uppercase: char = letter.to_ascii_uppercase();
        // find index of a letter to be encoded
        let idx = Self::ALPHABET.iter().position(|&x| x == uppercase).unwrap();
        return self.wiring[idx]
    }

    pub fn backward(&mut self, letter: char) -> char {
        /* Encode letter on backward pass after reflector 
        Unlike forward pass here we don't rotate the rotor. */
        let uppercase: char = letter.to_ascii_uppercase();
        let idx: usize = self.wiring.iter().position(|&x| x == uppercase).unwrap();
        return Self::ALPHABET[idx]
    }

    pub fn reset_to_start(&mut self) {
        self.wiring.rotate_right(self.total_turns);
        self.total_turns = 0;
        self.turn = false;
    }

    pub fn print(&self) {
        println!("Rotor {}\ncurrent position: {:?}\n", self.name, self.wiring);
    }

    pub fn is_turnover(self) -> bool {
        for t in self.turnover {
            if self.wiring[0] == t {
                return true;
            }
        }  
        return false;      
    }
}

fn main() {
}

