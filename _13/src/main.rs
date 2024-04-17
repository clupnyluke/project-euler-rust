use num_bigint::BigUint;
use num_traits::Zero;
use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("nums.txt").unwrap();
    let mut contents = String::new();
    let res = file.read_to_string(&mut contents);
    res.unwrap();

    let strings: Vec<&str> = contents.split(" ").collect();
    let mut sum: BigUint = Zero::zero();
    for string in strings {
        sum += string.parse::<BigUint>().unwrap();
    }
    print!("{}", sum.to_string().get(0..10).unwrap())
}
