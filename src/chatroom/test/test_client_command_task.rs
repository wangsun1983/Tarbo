use crate::chatroom::input::command;



//task list
pub fn test_task_command_parse_1() {
    let command_str = "list";
    let task_command = command::TaskCommand::from_string(command_str).unwrap();
    println!("command action is {}",task_command.get_action());
}

pub fn test_task_command_parse_2() {
    let command_str = "789 confirm Y";
    let task_command = command::TaskCommand::from_string(command_str).unwrap();
    println!("command action is {}",task_command.get_action());
    println!("pending task id is {}",task_command.get_pending_task_id());
    println!("confirm result is {}",task_command.get_confirm_result());

    let command_str = "789 confirm N";
    let task_command = command::TaskCommand::from_string(command_str).unwrap();
    println!("command2 action is {}",task_command.get_action());
    println!("pending2 task id is {}",task_command.get_pending_task_id());
    println!("confirm2 result is {}",task_command.get_confirm_result());

    let command_str = "  789   confirm    N  ";
    let task_command = command::TaskCommand::from_string(command_str).unwrap();
    println!("command3 action is {}",task_command.get_action());
    println!("pending3 task id is {}",task_command.get_pending_task_id());
    println!("confirm3 result is {}",task_command.get_confirm_result());
}

pub fn test_task_command_parse_3() {
    let command_str = "123 stop";
    let task_command = command::TaskCommand::from_string(command_str).unwrap();
    println!("command action is {}",task_command.get_action());
    println!("pending task id is {}",task_command.get_pending_task_id());

    let command_str = " 123   stop  ";
    let task_command = command::TaskCommand::from_string(command_str).unwrap();
    println!("command2 action is {}",task_command.get_action());
    println!("pending2 task id is {}",task_command.get_pending_task_id());
}