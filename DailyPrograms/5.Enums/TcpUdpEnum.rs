/*
1) Structure
    - Defining structure
    - Creating instance
    - Writing member function
2) Box: malloc
    - How to allocate memory
    - Idea on how the memory is deallocated
3) Enum : Option
    - Defining Enums
    - Using "match"
4) Exmaple  to implement Linked List (Block chain)
    - Creating blocks
    - Linking
*/

#[derive(Debug)]
struct TcpMsg {
    msg:String,
    len:i8
}

#[derive(Debug)]
struct UdpMsg {
    msg:String,
    len:i8
}

#[derive(Debug)]
enum Message {
    Invalid,
    Empty,
    udp_msg(UdpMsg),
    tcp_msg(TcpMsg)
}

fn get_msg(num:i8) -> Message {
    if num ==0 {
        return Message::udp_msg(UdpMsg{msg:String::from("udp message"), len:10});
    } else if num == 1 {
        return Message::tcp_msg(TcpMsg{msg:String::from("tcp message"), len:10});
    } else if num < 0 {
        return Message::Invalid;
    } else {
        return Message::Empty;
    }
}


fn main() {
    // println!("Return : {:?}", get_msg(1));
    // println!("Return : {:?}", get_msg(2));

    match get_msg(1) {
        Message::None => {println!("Invalud message")},
        Message::tcp_msg(msg) => {println!("TCP: Message = {}, Lngth = {}", msg.msg, msg.len)},
        Message::udp_msg(msg) => {println!("TCP: Message = {}, Lngth = {}", msg.msg, msg.len)},
    }
}
