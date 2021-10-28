use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("num.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut digits: Vec<i128> = Vec::new();

    for char in contents.chars() {
        digits.push(char.to_digit(10).unwrap().into());
    }

    let mut largest = 0;

    for i in 0..digits.len()-13 {
        let product = digits.get(i..i+13).unwrap().iter().fold(1, |a, b|  {a * b} );
        if product > largest {
            largest = product;
        }
    }

    println!("{}", largest);
}
