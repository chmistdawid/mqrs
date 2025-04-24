use bin_proto::{BitDecode, BitEncode, BitCodec};

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
#[codec(discriminant_type = u8)]
#[codec(bits = 2)]
enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}
#[derive(Debug, BitDecode, BitEncode, PartialEq)]
#[codec(discriminant_type = u8)]
#[codec(bits = 4)]
enum MqttPacketType {
    Connect = 1,
    ConnAck = 2,
    Publish = 3,
    PubAck = 4,
    PubRec = 5,
    PubRel = 6,
    PubComp = 7,
    Subscribe = 8,
    SubAck = 9,
    Unsubscribe = 10,
    UnsubAck = 11,
    PingReq = 12,
    PingResp = 13,
    Disconnect = 14,
    Auth = 15,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
struct ConnectFlags {
    #[codec(bits = 1)]
    reserved: bool,
    #[codec(bits = 1)]
    clean_session: bool,
    #[codec(bits = 1)]
    will_flag: bool,
    will_qos: QoS,
    #[codec(bits = 1)]
    will_retain: bool,
    #[codec(bits = 1)]
    password_flag: bool,
    #[codec(bits = 1)]
    username_flag: bool,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
struct MqttFixedHeader {
    #[codec(bits = 1)]
    duplication_flag: bool,
    qos: QoS,
    #[codec(bits = 1)]
    retain: bool,
    packet_type: MqttPacketType,
}

#[derive(Debug, BitDecode, BitEncode, PartialEq)]
struct MqttConnect {
    mqtt_header: MqttFixedHeader,
    #[codec(bits = 8)]
    length: u8,
    #[codec(bits = 16)]
    protocol_name_len: u16,
    #[codec(tag = protocol_name_len as usize)]
    protocol_name: Vec<u8>,
    #[codec(bits = 8)]
    protocol_version: u8,
    
    connect_flags: ConnectFlags,
    #[codec(bits = 16)]
    keep_alive: u16,
    #[codec(bits = 16)]
    client_id_len: u16,
    #[codec(tag = client_id_len as usize)]
    client_id: Vec<u8>,
    #[codec(bits = 16)]
    username_len: u16,
    #[codec(tag = username_len as usize)]
    username: Vec<u8>,
    #[codec(bits = 16)]
    password_len: u16,
    #[codec(tag = password_len as usize)]
    password: Vec<u8>,
}

fn main() {
    let header = MqttFixedHeader {
        duplication_flag: false,
        qos: QoS::AtMostOnce,
        retain: false,
        packet_type: MqttPacketType::Connect,
    };
    let encoded = header.encode_bytes(bin_proto::BigEndian).unwrap();

    let connect_header = MqttConnect {
        mqtt_header: header,
        length: 32,
        protocol_name_len: 4,
        protocol_name: "MQTT".as_bytes().to_vec(),
        protocol_version: 4,
        connect_flags: ConnectFlags {
            reserved: false,
            clean_session: true,
            will_flag: false,
            will_qos: QoS::AtMostOnce,
            will_retain: false,
            password_flag: true,
            username_flag: true,
        },
        keep_alive: 60,
        client_id_len: 10,
        client_id: "client_id".as_bytes().to_vec(),
        username_len: 8,
        username: "username".as_bytes().to_vec(),
        password_len: 8,
        password: "password".as_bytes().to_vec(),
    };

    println!("encoded: {:?}", encoded);
    let connect = connect_header.encode_bytes(bin_proto::BigEndian).unwrap();
    println!("connect: {:?}", connect);
}