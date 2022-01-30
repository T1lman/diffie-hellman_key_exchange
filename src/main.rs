mod algo;

fn main() {
    let p: u128 = 53; //modulus
    let g: u128 = 17; //base
    let secret1: u32 = 23;
    let secret2: u32 = 11;
    let public1 = algo::compute(secret1, g, p);
    let public2 = algo::compute(secret2, g, p);
    let key1 = algo::compute(secret1, public2, p);
    let key2 = algo::compute(secret2, public1, p);
    println!("key1: {}, key2: {}", key1, key2);
}
