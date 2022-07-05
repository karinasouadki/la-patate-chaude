use md5;
use rand::prelude::*;

struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}


fn main() {


    // Create a new input
    let input = MD5HashCashInput {
        complexity: 9,
        message: String::from("hello"),
    };

    let output = hashcash(input);

    println!("{}\n{}", output.seed, output.hashcode);
}

fn hashcash(input: MD5HashCashInput) -> MD5HashCashOutput {

    loop
    {
        let seed: u64 = random();
        let seed_with_message = format!("{}{}", seed.to_string(), input.message);
        let mut hashcode128 = compute_md5_to_u128(seed_with_message);
        let current_complexity = compute_complexity(hashcode128);
        let hashcode = format!("{:x}", hashcode128).to_string().to_uppercase();
        if current_complexity >= input.complexity{
            return MD5HashCashOutput {
                seed,
                hashcode
            }
        }
    }
}

fn compute_complexity(hashcode: u128) -> u32 {
    hashcode.leading_zeros()
}

fn compute_md5_to_u128(message: String) -> u128 {
    u128::from_be_bytes(md5::compute(message).0)
}

