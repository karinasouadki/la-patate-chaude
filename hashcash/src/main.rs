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
        let mut seed: u64 = random();
        let mut seed_with_message = format!("{}{}", seed.to_string(), input.message);
        let mut hashcode128 = u128::from_be_bytes(md5::compute(seed_with_message).0);
        let current_complexity = computeComplexity(hashcode128);
        let hashcode = format!("{:x}", hashcode128);
        if current_complexity >= input.complexity{
            return MD5HashCashOutput {
                seed,
                hashcode
            }
        }
    }
}

fn computeComplexity(hashcode: u128) -> u32 {
    let hashcode128 = u128::from_be_bytes(md5::compute(hashcode).0);
    hashcode128.leading_zeros()
}