use md5;


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

    // output = hashcash(input);

    let mut digest: [u8; 16] = *md5::compute("000000000000034Chello");
    println!("{:?}", digest);
    let mut msg: String = "0".to_string();

    for i in 0..16 {
        msg = format!("{}{:x}", msg, digest[i]);
    }

    count_first_zero_of_a_hexa(&msg);

    println!("{:?}", msg.as_bytes());

}

fn count_first_zero_of_a_hexa(hashcode: &String){
    let current_char: char = hashcode.chars().next().unwrap();
    let mut found_zero: bool = false;
    let sum_zero: u32;

    println!("{:?}", (current_char.to_string()).parse::<i32>().unwrap());
}
