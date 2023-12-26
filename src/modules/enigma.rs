use phf::phf_map;
use std::collections::HashMap;

use super::reflector::Reflector;
use super::rotor::Rotor;
use super::plugboard::Plugboard;


pub static ROTOR_SETTINGS: phf::Map<&'static str, (&'static str, &'static str)> = phf_map! {
    "I" => ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "Q"),
    "II" => ("AJDKSIRUXBLHWTMCQGZNPYFVOE", "E"),
    "III" => ("BDFHJLCPRTXVZNYEIWGAKMUSQO", "V"),
    "IV" => ("ESOVPZJAYQUIRHXLNFTGKDCMWB", "J"),
    "V" => ("VZBRGITYUPSDNHLXAWMJQOFECK", "Z"),
    "VI" => ("JPGVOUMFYQBENHZRDKASXLICTW", "ZM"),
    "VII" => ("NZJHGRCXMYSWBOUFAIVLPEKQDT", "ZM"),
    "VIII" => ("FKQHTLXOCBJSPDZRAMEWNIUYGV", "ZM"),
};

pub static REFLECTOR_SETTINGS: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "EJMZALYXVBWFCRQUONTSPIKHGD",
    "B" => "YRUHQSLDPXNGOKMIEBFZCWVJAT",
    "C" => "FVPJIAOYEDRZXWGCTKUQSBNMHL",
};


pub struct EnigmaM3<'a> {
    pub rotors: Vec<Rotor<'a>>,
    pub plugboard: Plugboard<'a>,
    pub reflector: Reflector<'a>,
}

impl<'a> EnigmaM3<'a> {
    pub fn new(
        first_rotor: &'a str, 
        second_rotor: &'a str, 
        third_rotor: &'a str, 
        starting_positions: [usize; 3],  
        reflector: &'a str,
        plugboard: &'a HashMap<char, char>) -> EnigmaM3<'a> {
        let mut rotors: Vec<Rotor> = Vec::with_capacity(3);

        let mut idx = 0;
        for i in [first_rotor, second_rotor, third_rotor] {
            let r = Rotor::new(
                i, 
                ROTOR_SETTINGS.get(i).unwrap().0, 
                starting_positions[idx], 
                ROTOR_SETTINGS.get(i).unwrap().1
            );
            idx = idx + 1;
            rotors.push(r);
        };
        let w = Vec::from_iter(REFLECTOR_SETTINGS.get(reflector).unwrap().chars());
        let refl = Reflector::new(reflector, w);
        let pb = Plugboard::new(&plugboard);
        return EnigmaM3{rotors: rotors, plugboard: pb, reflector: refl};
    }

    pub fn encode_decode(&mut self, msg: &str) -> String {
        /* This method is ued for both encoding and decoding of the message.
        To encode a plaintext message is simply passed as the argument.
        To decode (if using the same instance as for encoding) reset settings
        to default and then pass encoded message to the method.*/

        // create an empty vector to push data into
        let mut v: Vec<char> = Vec::with_capacity(msg.len());
        // iterate through characters of the message
        for c in msg.chars() {
            let pb = self.plugboard.encode(c);
            // for the first rotor we always set rotate tu true
            self.rotors[0].turn = true;
            let r1 = self.rotors[0].forward(pb);
            if self.rotors[0].wiring[0] == self.rotors[0].turnover[0] {
                self.rotors[1].turn = true;
            }
            else if self.rotors[0].turnover.len() == 2 && self.rotors[0].wiring[0] == self.rotors[0].turnover[1] {
                self.rotors[1].turn = true;
            }
            let r2 = self.rotors[1].forward(r1);
            if self.rotors[1].wiring[0] == self.rotors[1].turnover[0] {
                self.rotors[2].turn=true;
            }
            else if self.rotors[1].turnover.len() == 2 && self.rotors[1].wiring[0] == self.rotors[1].turnover[1] {
                self.rotors[2].turn=true;
            }
            let r3 = self.rotors[2].forward(r2);
            let refl = self.reflector.reflect(r3);
            let r3r = self.rotors[2].backward(refl);
            let r2r = self.rotors[1].backward(r3r);
            let r1r = self.rotors[0].backward(r2r);
            let res = self.plugboard.encode(r1r);
            v.push(res);
        }
        return v.iter().collect();
    }

    pub fn reset_to_start(&mut self) {
        self.rotors[0].reset_to_start();
        self.rotors[1].reset_to_start();
        self.rotors[2].reset_to_start();
    }

}


fn main() {
    let map = &HashMap::from_iter([('A', 'F'), ('B', 'Y'), ('C', 'Q'), ('N', 'D')]);
    let mut en = EnigmaM3::new("I", 
                            "II", 
                            "III", 
                            [2, 3, 5], 
                            "A",
                            map
    );
    let msg = "HELLOENIGMATICWORLD";
    let res = en.encode_decode(msg);
    println!("{}", res);
    en.reset_to_start();
    let dec = en.encode_decode(&res);
    println!("{}", dec);
}
