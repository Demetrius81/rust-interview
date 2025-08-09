fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn primes_in_range(start: u64, end: u64) -> Vec<u64> {
    (start..=end)
        .filter(|&x| is_prime(x))
        .collect()
}

pub fn run() {
    let primes = primes_in_range(10, 50);
    println!("Простые числа: {:?}", primes);
}
