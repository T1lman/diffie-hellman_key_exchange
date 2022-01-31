mod algo;

fn main() {
    let p: u128 = 149; //modulus
    let g: u128 = 79; //base
    let secret1: u128 = 1110234342;
    let secret2: u128 = 8721312;
    let public1 = algo::compute(secret1, g, p);
    let public2 = algo::compute(secret2, g, p);
    let key1 = algo::compute(secret1, public2, p);
    let key2 = algo::compute(secret2, public1, p);
    println!("key1: {}, key2: {}", key1, key2);
}
