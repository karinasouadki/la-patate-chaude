use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;


enum Messages {
    Hello(String),
    Subscribe,
    ChallengeResult,
}

struct Subscribe {
    name:String
}
enum ChallengeResult{
    ChallengeName(ChallengeInput)
}


fn sendMessageToServer(message: String) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    for _ in 0..1000 {
        let mut input = message;
      //  let mut input_hello = "Hello".to_string();
        io::stdin().read_line(&mut input).expect("Failed to read");
        stream.write(input.as_bytes()).expect("failed to write");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_until(b'\n', &mut buffer)?;

        println!("read from server:{}", str::from_utf8(&buffer).unwrap());
        println!("");
    }
    Ok(())
}

