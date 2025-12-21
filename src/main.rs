/// List of odd numbers where all multiples of 3, 5, 7, 11, 13, 17 are marked as false 
static ODD_WHEEL: [u8; 255255] = {
    let mut table = [1u8; 255255];
    let mut i = 0;
    while i < 255255 {
        let n = 2 * i + 1;
        if has_prime_proper_divisor(n) {
            table[i] = 0;
        }
        i += 1;
    }
    table
};

fn main() {
    let n: usize = 1_000_000_000;
    println!("Intialising table...");
    // Initialise from odd-only wheel
    let mut table: Vec<u8> = ODD_WHEEL
        .iter()
        .cycle()
        .take((n + 1) / 2)
        .copied()
        .collect();
    // 1 is not prime
    table[0] = 0;
    let mut p: usize = 19;
    println!("Sieving...");
    while p * p <= n {
        let mut i = (p * p) / 2;
        while i < table.len() {
            table[i] = 0;
            i += p;
        }
        println!("Eliminated all multiples of {}", p);
        p += 2;
        while p <= n && table[p / 2] == 0 {
            p += 2;
        }
    }
    println!("Collecting primes...");
    let mut primes = Vec::new();
    primes.push(2);
    for (i, &b) in table.iter().enumerate() {
        if b == 1 {
            primes.push(2 * i + 1);
        }
    }
}

/// Returns true if `num` has any prime proper divisors up to 17
const fn has_prime_proper_divisor(num: usize) -> bool {
    (num % 2 == 0 && num != 2)
        || (num % 3 == 0 && num != 3)
        || (num % 5 == 0 && num != 5)
        || (num % 7 == 0 && num != 7)
        || (num % 11 == 0 && num != 11)
        || (num % 13 == 0 && num != 13)
        || (num % 17 == 0 && num != 17)
}
