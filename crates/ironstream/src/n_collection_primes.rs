// FILE: n_collection_primes.rs
// occt: NCollection_Primes

/// Prime number utilities for hash table sizing.
pub struct Primes;

impl Primes {
    /// Standard prime numbers for hash table sizes.
    const PRIME_SEQUENCE: &'static [i32] = &[
        17, 37, 53, 89, 127, 181, 241, 337, 433, 577, 757, 947, 1259, 1597, 2053, 2713, 3371,
        4373, 5623, 7213, 9293, 11987, 15461, 19997, 25811, 33317, 43103, 55703, 71989, 93089,
        120463, 156011, 201977, 261973, 340079, 441263, 573107, 744137, 966197, 1255003,
    ];

    /// Find a suitable prime for a given size.
    pub fn nearest(size: i32) -> i32 {
        for &p in Self::PRIME_SEQUENCE {
            if p >= size {
                return p;
            }
        }
        Self::PRIME_SEQUENCE[Self::PRIME_SEQUENCE.len() - 1]
    }

    /// Check if a number is prime.
    pub fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let limit = (n as f64).sqrt() as i32;
        for i in (3..=limit).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest() {
        assert_eq!(Primes::nearest(10), 17);
        assert_eq!(Primes::nearest(50), 53);
        assert_eq!(Primes::nearest(100), 127);
    }

    #[test]
    fn test_is_prime() {
        assert!(Primes::is_prime(2));
        assert!(Primes::is_prime(17));
        assert!(Primes::is_prime(53));
        assert!(!Primes::is_prime(1));
        assert!(!Primes::is_prime(4));
        assert!(!Primes::is_prime(15));
    }
}
