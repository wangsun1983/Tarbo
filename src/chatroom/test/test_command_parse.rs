use crate::chatroom::input;

//sampel command:connect ip=127.0.0.1,port=1234,name=hh
pub fn test_connect_command_parse() {
    let command = String::from("connect ip=127.0.0.1,port=1234,name=test");
    let (id,param) =  input::parser::parse_command(&command);
    println!("id is {},param is {}",id,param);

    let connect_command = input::parser::parse_connect_command(param);
    match connect_command {
        Some(cmd)=> {
            println!("ip is {},port is {},name is {}",cmd.get_ip(),cmd.get_port(),cmd.get_name());
        },
        None=> {
            println!("parse connect command failed");
        }
    }
}

pub fn test_msg_command_parse() {
    let command = String::from("msg wang,hhhhh");
    let (id,param) =  input::parser::parse_command(&command);
    println!("id is {},param is {}",id,param);

    let msg_command = input::parser::parse_message_command(param);
    match msg_command {
        Some(cmd)=> {
            println!("name is {},content is {}",cmd.get_name(),cmd.get_text());
        },
        None=> {
            println!("parse msg command failed");
        }
    }
}