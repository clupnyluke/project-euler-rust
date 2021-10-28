fn main() {
    let mut primes: Vec<u128> = Vec::new();
    let mut num: u128 = 2;
    loop {
        if primes.len() > 10000 {
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
            primes.push(num);
            println!("{}: {}", primes.len(), primes.last().unwrap())
        }
        num += 1;
    }
    println!("10001 prime: {}", primes.last().unwrap());
}
