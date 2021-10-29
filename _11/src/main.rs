use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("nums.txt").unwrap();
    let mut contents = String::new();
    let mut numbers: Vec<u128> = Vec::new();
    file.read_to_string(&mut contents);

    let strings: Vec<&str> = contents.split(" ").collect();
    for string in strings {
        numbers.push(string.parse().unwrap());
    }

    let mut largest: u128 = 0;

    for i in 1..397 {
        let mut products: Vec<u128> = Vec::new();
        if i + 63 <= 400 && i % 20 < 17 {
            let mut product = 1;
            for j in 0..4 {
                println!("{}", j);
                product *= numbers.get(i + j * 20).unwrap();
            }
            products.push(product)
        }
        if i + 57 <= 400 && i % 20 > 3 {
            let mut product = 1;
            for j in 0..4 {
                product *= numbers.get(i + j * 19).unwrap();
            }
            products.push(product);
        }
        if i % 20 < 17 {
            let mut product = 1;
            for j in 0..4 {
                product *= numbers.get(i + j).unwrap();
            }
            products.push(product);
        }
        if i + 60 <= 399 {
            let mut product = 1;
            for j in 0..4 {
                product *= numbers.get(i + j * 20).unwrap();
            }
            products.push(product);
        }
        if (products.len() > 0){
            if largest < *products.iter().max().unwrap() {
                largest = *products.iter().max().unwrap();
            }
        }
    }
    print!("{}", largest);
}
