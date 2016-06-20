extern crate code;

use code::levenshtein;

fn main() {
    let u = "kangaroo";
    let v = "koala";

    let distance = levenshtein::recursive(u, v);

    println!("distance between \"{}\" and \"{}\" is {}", u, v, distance);
}
