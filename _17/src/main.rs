fn count_number(num: usize) -> usize {
    let digits = vec![3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 9, 8, 7, 7, 9, 8, 8];
    let tens_digits = vec![6, 6, 5, 5, 5, 7, 6, 6];
    let mut remainder: usize;
    let mut sum = 0usize;
    if num >= 100 {
        let hundreds_digit = num / 100;
        remainder = num - (hundreds_digit * 100);
        sum += digits[hundreds_digit - 1] + 7;
        if remainder != 0 {
            sum += 3
        }
    } else {
        remainder = num;
    }
    if remainder >= 20 {
        let tens_digit = remainder / 10;
        remainder -= tens_digit * 10;
        sum += tens_digits[tens_digit - 2];
    }
    if remainder > 0 {
        sum += digits[remainder - 1];
    }
    sum
}
fn main() {
    let mut sum = 0usize;
    for i in 1..999usize {
        sum += count_number(i)
    }
    sum += 11;
    println!("{}", count_number(115));
    println!("{}", sum);
}
