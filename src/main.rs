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
    // let segment_size = (n as f32).sqrt() as usize;
    let segment_size = n / 5;
    println!("Initialising table...");
    // Initialise from odd-only wheel
    let mut table: Vec<u8> = repeated_range(&ODD_WHEEL, 0, (segment_size + 1) / 2);
    // 1 is not prime
    table[0] = 0;
    let mut p: usize = 19;
    // First segement sieve (normal)
    println!("Sieving chunk 1...");
    while p * p <= segment_size {
        let mut i = (p * p) / 2;
        while i < table.len() {
            table[i] = 0;
            i += p;
        }
        p += 2;
        while p <= n && table[p / 2] == 0 {
            p += 2;
        }
    }
    // Collect primes in first segement
    println!("Collecting primes in chunk 1...");
    let mut primes = Vec::new();
    for (i, &b) in table.iter().enumerate() {
        if b == 1 {
            primes.push(2 * i + 1);
        }
    }
    println!("Processing chunks...");
    let mut i = 0;
    while i < n / segment_size {
        let start = 2 * (i + 1) * segment_size / 2 + 1;
        let odd_segment_size = if start + segment_size > n {
            (n - start) / 2 + 1
        } else {
            segment_size / 2
        };
        if start > n {
            break;
        }
        // Get segment
        let mut segment = repeated_range(&ODD_WHEEL, start / 2, start / 2 + odd_segment_size);
        sieve_segment(&mut primes, &mut segment, start);
        i += 1;
    }
    primes.insert(0, 2);
    // println!("Primes: {:?}", primes);
    println!("Number of primes: {}", primes.len());
}

fn sieve_segment(primes: &mut Vec<usize>, segment: &mut Vec<u8>, start: usize) {
    println!("Sieving chunk {start}...");
    let mut i = 0;
    let mut p = primes[0];
    let up = start + 2 * segment.len();
    // Sieve
    while p * p <= up {
        i += 1;
        if let Some(pr) = primes.get(i - 1) {
            p = *pr;
            let mut j = p * p;
            while j < start {
                j += 2 * p;
            }
            while j < up {
                segment[(j - start) / 2] = 0;
                j += 2 * p;
            }
        } else {
            break;
        }
    }
    println!("Collecting primes in chunk {start}...");
    // Collect primes
    for (i, &b) in segment.iter().enumerate() {
        if b == 1 {
            primes.push(2 * i + start);
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

/// Helper function to repeat range
#[inline(always)]
fn repeated_range<'a>(
    list: &'a [u8],
    n: usize,
    x: usize,
) -> Vec<u8> {
    (n..x)
    .map(move |i| unsafe {
        *list.get_unchecked(i % list.len())
    })
    .collect()
}

