use chrono::{DateTime, Utc};

enum Fixmsg {
    BeginString(String),
    BodyLength(u32),
    MsgType(char),
    SenderCompID(String),
    TargetCompID(String),
    TargetSubID(String),
    SenderSubID(String),
    MsgSeqNum(u64),
    SendingTime(DateTime<Utc>),
    CheckSum(u32),
}

fn msg_code(msg: &Fixmsg) -> i32 {
    match msg {
        Fixmsg::BeginString(_) => 8,
        Fixmsg::BodyLength(_) => 9,
        Fixmsg::MsgType(_) => 35,
        Fixmsg::SenderCompID(_) => 49,
        Fixmsg::TargetCompID(_) => 56,
        Fixmsg::TargetSubID(_) => 57,
        Fixmsg::SenderSubID(_) => 50,
        Fixmsg::MsgSeqNum(_) => 34,
        Fixmsg::SendingTime(_) => 52,
        Fixmsg::CheckSum(_) => 10,
    }
}

fn encode(msg: &Fixmsg) -> String {
    match msg {
        Fixmsg::BeginString(proto) => [msg_code(&msg).to_string(), proto.to_string()].join("="),
        Fixmsg::BodyLength(len) => [msg_code(&msg).to_string(), len.to_string()].join("="),
        Fixmsg::MsgType(mtype) => [msg_code(&msg).to_string(), mtype.to_string()].join("="),
        Fixmsg::SenderCompID(scid) => [msg_code(&msg).to_string(), scid.to_string()].join("="),
        Fixmsg::TargetCompID(tcid) => [msg_code(&msg).to_string(), tcid.to_string()].join("="),
        Fixmsg::TargetSubID(tsid) => [msg_code(&msg).to_string(), tsid.to_string()].join("="),
        Fixmsg::SenderSubID(ssid) => [msg_code(&msg).to_string(), ssid.to_string()].join("="),
        Fixmsg::MsgSeqNum(seqnum) => [msg_code(&msg).to_string(), seqnum.to_string()].join("="),
        Fixmsg::SendingTime(tm) => [msg_code(&msg).to_string(), tm.to_string()].join("="),
        Fixmsg::CheckSum(i) => [msg_code(&msg).to_string(), i.to_string()].join("="),
    }
}

fn main() {
    let fix_proto_string = "FIXT.1.1";
    let scid = "ArmstrongTrading";
    let tcid = "MorganStanley";
    let mut seqnum: u64 = 0;

    let begin_msg = Fixmsg::BeginString(fix_proto_string.to_string());
    let check_msg = Fixmsg::CheckSum(32);
    let time_msg = Fixmsg::SendingTime(Utc::now());
    println!("example msgpart: {}", encode(&begin_msg));
    println!("example msgpart: {}", encode(&check_msg));
    println!("example msgpart: {}", encode(&time_msg));

    let message_vec = vec![
        Fixmsg::BeginString(fix_proto_string.to_string()),
        Fixmsg::MsgType('A'),
        Fixmsg::SenderCompID(scid.to_string()),
        Fixmsg::TargetCompID(tcid.to_string()),
        Fixmsg::MsgSeqNum(seqnum),
        Fixmsg::BodyLength(108),
        Fixmsg::SendingTime(Utc::now()),
    ];
    seqnum += 1;
    let msg_string = message_vec
        .iter()
        .map(|msg: &Fixmsg| encode(msg))
        .collect::<Vec<_>>()
        .join("|");

    println!("final msg:\n{}", msg_string);
}
