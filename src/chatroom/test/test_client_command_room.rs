use crate::chatroom::input::command;

pub fn test_room_command_parse_1() {
    let cmd = "new abc";
    let command = command::RoomCommand::from_string(&cmd).unwrap();
    println!("trace1 event is {},room name is {}", command.get_event(),command.get_room_name());

    let cmd = " new   abc  ";
    let command = command::RoomCommand::from_string(&cmd).unwrap();
    println!("trace2 event is {},room name is {}", command.get_event(),command.get_room_name());

    let cmd = " new     ";
    let result = command::RoomCommand::from_string(&cmd);
    match result {
        Some(_)=> {

        },
        None=> {
            println!("trace3 no event!!");
        }
    }

    let cmd = "new ";
    let result = command::RoomCommand::from_string(&cmd);
    match result {
        Some(_)=> {

        },
        None=> {
            println!("trace4 no event!!");
        }
    }

    

}