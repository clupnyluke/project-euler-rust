use num_bigint::BigUint;

fn main() {
    let mut num: BigUint = BigUint::from_slice(&[2]);
    num = num.pow(1000);

    let mut sum = BigUint::from_slice(&[0]);
    while num > BigUint::from_slice(&[0]) {
        let n = num.clone() % 10 as u32;
        sum += n;
        num /= 10 as u32;
    }
    println!("{}", sum);
}
