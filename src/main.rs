use std::env;
mod modules;

use modules::enigma::*;


fn main() {
    let args: Vec<_> = env::args().collect();

    // find idx of --pos, use next 3 numbers as initial settings for rotors
    let idx_pos: usize = match args.iter().position(|x| x == "--pos" || x == "-p") {
        None => panic!("--pos argument not found"),
        Some(n) => n,
    };

    let idx_rotor: usize = match args.iter().position(|x| x == "--rotors" || x == "-r") {
        None => panic!("--rotors argument not found"),
        Some(n) => n,
    };

    let idx_pb: usize = match args.iter().position(|x| x == "--plugboard" || x == "-g") {
        None => panic!("--plugboard argument not found"),
        Some(n) => n,
    };

    let ring_pos = Vec::from_iter(args[idx_pos+1..idx_pos+4].iter().map(|x| x.parse::<usize>().unwrap()));
    let rotors = Vec::from_iter(args[idx_rotor+1..idx_rotor+4].iter().map(|x| x.parse::<String>().unwrap()));
    let plugboard = Vec::from_iter(args[idx_pb+1..].iter().map(|x| x.parse::<String>().unwrap()));

    println!("{:?} {:?} {:?}", ring_pos, rotors, plugboard);
}