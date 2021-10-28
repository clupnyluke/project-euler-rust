fn main() {
    let mut primes: Vec<u128> = Vec::new();
    let mut num: u128 = 2;
    loop {
        if num >= 2000000 {
            break;
        }
        let mut factor = false;
        for prime in primes.iter() {
            if num % prime == 0 {
                factor = true;
                break;
            }
        } 
        if !factor {
            println!("{}", num);
            primes.push(num);
        }
        num += 1;
    }
    let sum: u128 = primes.iter().sum();
    println!("{}", sum);
}
