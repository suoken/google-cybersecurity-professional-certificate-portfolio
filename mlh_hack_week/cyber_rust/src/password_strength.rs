// I shouldn't recreate the wheel so I'm using zxcvbn for password strength measuring
// "Through pattern matching and conservative estimation, it recognizes and weighs 30k common passwords, common names and surnames according to US census data, popular English words from Wikipedia and US television and movies, and other common patterns like dates, repeats (aaa), sequences (abcd), keyboard patterns (qwertyuiop), and l33t speak."

extern crate zxcvbn;

use zxcvbn::zxcvbn;

pub fn password_strength() {
    let entropy = zxcvbn("couchTreePlan#45", &[]).unwrap();
    println!("{}", entropy.score())
}