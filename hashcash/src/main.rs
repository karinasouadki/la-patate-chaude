use md5;
use std::str;

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
    
    pub struct Digest(pub [u8; 16]);

    // Create a new input
    let input = MD5HashCashInput {
        complexity: 9,
        message: String::from("hello"),
    };

    // output = hashcash(input);

    let digest: [u8; 16] = *md5::compute("000000000000034Chello");


    println!("{:?}", digest[0]);
}

// fn hashcash(input: MD5HashCashInput) -> MD5HashCashOutput {
//     let seed = String::from("000000000000034C");
//     let digest = md5::compute(b"000000000000034Chello");

//     let output = MD5HashCashOutput {
//         seed: seed,
//         hashcode: digest,
//     };

//     println!("{:?}", digest);

//     output
// }
