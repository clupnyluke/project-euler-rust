fn palindrome_check(num: u128) -> bool {
    let mut str = num.to_string();
    let mut out = true;
    loop {
        if str.chars().count() < 2 {
            break;
        }
        if str.pop() == str.chars().next() {
            str.remove(0);
        }
        else {
            out = false;
            break;
        }
    }
    out
}

fn main() {
    let mut num1 = 100;
    let mut largest = 0;
    loop {
        if num1 >= 1000 {
            break;
        }
        let mut num2 = num1;
        loop {
            if num2 >= 1000 {
                break;
            }
            let product = num1 * num2;
            if palindrome_check(product) {
                if product > largest {
                    largest = product;
                }
            }
            num2 += 1;
        }
        num1 += 1;
    }
    println!("{}", largest)
}
