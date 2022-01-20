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
fn serialize_msg(seqnum: &mut u64, mut msg_vec: Vec<Fixmsg>) -> Result<String, &'static str> {
    let seqnum_this_msg = *seqnum;
    if seqnum_this_msg == u64::MAX {
        Err("Encountered u64::MAX. Reset sequence.")
    } else {
        *seqnum += 1;
        msg_vec.push(Fixmsg::MsgSeqNum(seqnum_this_msg));
        msg_vec.push(Fixmsg::SendingTime(Utc::now()));
        let msg_len: u32 = (msg_vec.len() * 10).try_into().unwrap();
        msg_vec.push(Fixmsg::BodyLength(msg_len));
        Ok(msg_vec
            .iter()
            .map(|msg: &Fixmsg| encode(msg))
            .collect::<Vec<_>>()
            .join("|"))
    }
}

fn main() {
    let fix_proto_string = "FIXT.1.1";
    let scid = "ArmstrongTrading";
    let ssid = "GlobalMacro";
    let tcid = "MorganStanley";
    let tsid = "FixedIncomeDesk";
    let mut seqnum: u64 = 0;

    let begin_msg = Fixmsg::BeginString(fix_proto_string.to_string());
    let check_msg = Fixmsg::CheckSum(32);
    let time_msg = Fixmsg::SendingTime(Utc::now());
    println!("example msgpart: {}", encode(&begin_msg));
    println!("example msgpart: {}", encode(&check_msg));
    println!("example msgpart: {}", encode(&time_msg));

    let msg1 = vec![
        Fixmsg::BeginString(fix_proto_string.to_string()),
        Fixmsg::MsgType('A'), // Logon
        Fixmsg::SenderCompID(scid.to_string()),
        Fixmsg::SenderSubID(ssid.to_string()),
        Fixmsg::TargetCompID(tcid.to_string()),
        Fixmsg::TargetSubID(tsid.to_string()),
    ];

    let msg2 = vec![
        Fixmsg::BeginString(fix_proto_string.to_string()),
        Fixmsg::MsgType('1'), // Test
        Fixmsg::SenderCompID(scid.to_string()),
        Fixmsg::SenderSubID(ssid.to_string()),
        Fixmsg::TargetCompID(tcid.to_string()),
        Fixmsg::TargetSubID(tsid.to_string()),
    ];

    let msg3 = vec![
        Fixmsg::BeginString(fix_proto_string.to_string()),
        Fixmsg::MsgType('5'), // Logout
        Fixmsg::SenderCompID(scid.to_string()),
        Fixmsg::SenderSubID(ssid.to_string()),
        Fixmsg::TargetCompID(tcid.to_string()),
        Fixmsg::TargetSubID(tsid.to_string()),
    ];
    println!("msg1:\n{}", serialize_msg(&mut seqnum, msg1).unwrap());
    println!("msg2:\n{}", serialize_msg(&mut seqnum, msg2).unwrap());
    println!("intentional captured panic:");
    seqnum = u64::MAX;
    match serialize_msg(&mut seqnum, msg3) {
        Ok(msg_str) => println!("{}", msg_str),
        Err(e) => println!("{}", e),
    }
}
