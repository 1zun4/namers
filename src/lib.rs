use rand::prelude::SliceRandom;

const NAMES: &str = include_str!("../wordlists/wordlists/names/names.txt");

pub fn random() -> Option<String> {
    let names: Vec<&str> = NAMES.lines().collect();
    names.choose(&mut rand::thread_rng()).map(|&name| name.to_owned())
}