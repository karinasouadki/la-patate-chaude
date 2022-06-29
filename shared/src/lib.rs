use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Welcome {
    version: u8
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Hello,
    Welcome(Welcome)
}




// Les types additionnels
#[derive(Debug, Deserialize, Serialize)]
pub enum SubscribeError {
    AlreadyRegistered, 
    InvalidName
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChallengeAnswer {
 //   ChallengeName(ChallengeOutput)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChallengeResult {
    name: ChallengeAnswer,
    next_target: String
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }  
 }

 #[derive(Debug, Deserialize, Serialize)]
pub struct ReportedChallengeResult {
    name: String,
  //  value: JobValue,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublicLeaderBoard {
  
}


//from server to client

#[derive(Debug, Deserialize, Serialize)]
pub enum Messages {
    Welcome,
    SubscribeResult,
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge,
    RoundSummary,
    EndOfGame,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SubscribeResult {
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Challenge {
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoundSummary {
    Challenge: String,
    chain: Vec<ReportedChallengeResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EndOfGame {
    leader_board:PublicLeaderBoard,
}



//from client to server
