use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("nums.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let strings: Vec<&str> = contents.split(" ").collect();
    let mut sum: u128 = 0;
    for string in strings {
        println!("{}", sum);
        sum += string.parse::<u128>().unwrap();
    }
    print!("{}", sum.to_string().get(0..10).unwrap())
}
