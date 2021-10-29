fn main() {
    let mut triangle_number = 1;
    let mut triangle_number_increase = 2;
    loop {
        let mut i = 2;
        let mut max = triangle_number;
        let mut divisors_num = 2;
        loop {
            if i >= max {
                break;
            }
            if triangle_number % i == 0 {
                divisors_num += 2;
                max = triangle_number / i;
            }
            i += 1;
        }
        if divisors_num > 500 {
            break;
        }
        triangle_number += triangle_number_increase;
        triangle_number_increase += 1;
    }
    println!("{}", triangle_number);
}
