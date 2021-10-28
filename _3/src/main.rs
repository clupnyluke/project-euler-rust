fn main() {
    let mut num: u128 = 600851475143;
    let mut factor = 2;
    let mut largest = 1;
    loop {
        if factor > num / 2 {
            break;
        }
        if num % factor == 0 {
            num /= factor;
            largest = factor;
        }
        else {
            factor += 1;
        }
    }

    if num > largest {
        largest = num;
    }
    println!("{}", largest)
}
