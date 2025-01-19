fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1]; 
    is_prime[0] = false; 
    is_prime[1] = false; 

    for p in 2..=n {
        if is_prime[p] {
            // If p is prime, mark all its multiples as composite
            let mut j = p * p; 
            while j <= n {
                is_prime[j] = false;
                j += p;
            }
        }
    }

    (2..=n)
        .filter(|&x| is_prime[x]) 
        .collect() 
}

