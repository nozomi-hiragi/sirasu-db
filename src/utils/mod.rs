pub mod jwt;
pub mod mongodb;
pub mod params;

use rand::{self, Rng};

pub fn gen_random_string() -> String {
    let mut alphabet_vec: Vec<char> = vec![];
    for _ in 1..13 {
        let rand_num = rand::thread_rng().gen_range(65..91);
        if let Some(rand_num) = std::char::from_u32(rand_num) {
            alphabet_vec.push(rand_num);
        }
    }
    alphabet_vec.iter().collect::<String>()
}
