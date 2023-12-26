
pub struct Reflector<'a> {
    name: &'a str,
    wiring: Vec<char>,
}

impl<'a> Reflector<'a>{
    const ALPHABET: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
                                  'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
                                  'X', 'Y', 'Z'];

    pub fn new(name: &str, wiring: Vec<char>) -> Reflector {
        return Reflector{name: name, wiring: wiring};
    }

    pub fn reflect(&self, letter: char) -> char {
        let upper = letter.to_ascii_uppercase();
        let idx = Self::ALPHABET.iter().position(|&x| x == upper).unwrap();
        return self.wiring[idx]
    }
}


fn main() {

}