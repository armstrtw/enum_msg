use chrono::{DateTime, Utc};

enum Fixmsg {
    BeginString,
    BodyLength,
    MsgType,
    SenderCompID,
    TargetCompID,
    TargetSubID,
    SenderSubID,
    MsgSeqNum,
    SendingTime(DateTime<Utc>),
    CheckSum(i32),
}

fn msg_code(msg: Fixmsg) -> i32 {
    match msg {
        Fixmsg::BeginString => 8,
        Fixmsg::BodyLength => 9,
        Fixmsg::MsgType => 35,
        Fixmsg::SenderCompID => 49,
        Fixmsg::TargetCompID => 56,
        Fixmsg::TargetSubID => 57,
        Fixmsg::SenderSubID => 50,
        Fixmsg::MsgSeqNum => 34,
        Fixmsg::SendingTime(_) => 52,
        Fixmsg::CheckSum(_) => 10,
    }
}

fn encode(msg: Fixmsg) {
    match msg {
        Fixmsg::BeginString => println!("8="),
        Fixmsg::BodyLength => println!("9="),
        Fixmsg::MsgType => println!("35="),
        Fixmsg::SenderCompID => println!("49="),
        Fixmsg::TargetCompID => println!("56="),
        Fixmsg::TargetSubID => println!("57="),
        Fixmsg::SenderSubID => println!("50="),
        Fixmsg::MsgSeqNum => println!("34="),
        Fixmsg::SendingTime(tm) => println!("52={}", tm.to_string()),
        //Fixmsg::CheckSum(i) => println!("10={}", i.to_string()),
        Fixmsg::CheckSum(i) => println!("{}={}", msg_code(msg).to_string(), i.to_string()),
    };
}

fn main() {
    let check_msg = Fixmsg::CheckSum(32);
    let time_msg = Fixmsg::SendingTime(Utc::now());
    //println!("msg: {}", encode(check_msg));
    encode(check_msg);
    encode(time_msg);
}
