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

fn encode(msg: Fixmsg) -> String {
    match msg {
        Fixmsg::BeginString => msg_code(msg).to_string(),
        Fixmsg::BodyLength => msg_code(msg).to_string(),
        Fixmsg::MsgType => msg_code(msg).to_string(),
        Fixmsg::SenderCompID => msg_code(msg).to_string(),
        Fixmsg::TargetCompID => msg_code(msg).to_string(),
        Fixmsg::TargetSubID => msg_code(msg).to_string(),
        Fixmsg::SenderSubID => msg_code(msg).to_string(),
        Fixmsg::MsgSeqNum => msg_code(msg).to_string(),
        Fixmsg::SendingTime(tm) => [msg_code(msg).to_string(), tm.to_string()].join("="),
        Fixmsg::CheckSum(i) => [msg_code(msg).to_string(), i.to_string()].join("="),
    }
}

fn main() {
    let begin_msg = Fixmsg::BeginString;
    let check_msg = Fixmsg::CheckSum(32);
    let time_msg = Fixmsg::SendingTime(Utc::now());
    println!("msg: {}", encode(begin_msg));
    println!("msg: {}", encode(check_msg));
    println!("msg: {}", encode(time_msg));
}
