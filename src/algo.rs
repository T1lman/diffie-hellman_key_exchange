pub fn check_prime(prime: u64) -> bool {
    for i in 2..prime / 2 {
        if prime % i == 0 {
            return false;
        }
    }
    true
}

pub fn compute(exp: u32, base: u128, modulus: u128) -> u128 {
    let pow = base.pow(exp);
    let computed = pow % modulus;
    computed
}
