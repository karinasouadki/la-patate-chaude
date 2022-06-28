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
    // println!("{:?}", digest);
    let mut msg: String = String::new();

    for i in 0..16 {
        msg = format!("{}{}", msg, convertHexaToBinary(digest[i]));
    }

    count_first_zero_of_a_binary(&msg);



    // for i in 0..16 {
    //     msg = format!("{}{:x}", msg, digest[i]);
    // }


    // println!("{:?}", msg.as_bytes());

}

fn count_first_zero_of_a_binary(hashcode: &String) -> u8{
    let mut sumOfZero: u8 = 0;
    for i in 0..hashcode.len(){
        if hashcode.chars().nth(i).unwrap() != '0'{
            break;
        }
        sumOfZero += 1;
    }
    sumOfZero
    
}

fn convertHexaToBinary(hexa: u8) -> String {
    let x = format!("{:0>8}", format!("{:b}", hexa));
    x
}