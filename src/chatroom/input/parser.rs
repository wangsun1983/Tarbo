use crate::chatroom::message;
use crate::chatroom::message::message_id;
use crate::chatroom::input::command::ConnectCommand;
use crate::chatroom::input::command::MessageCommand;
use crate::chatroom::input::command::NameCommand;

use super::command;
use super::command::BroadCastCommand;
use super::command::FileCommand;
use super::command::GroupCommand;
use super::command::ListCommand;
use super::command::EnterCommand;
use super::command::TaskCommand;
use super::command::RoomCommand;
use super::command::HistoryCommand;

pub fn parse_command(cmd:&String)->(u32,&str) {
    if cmd.starts_with("connect") {
        return (message_id::Id_NewComer,&cmd.as_str()[7..]);
    } else if cmd.starts_with("msg") {
        return (message_id::Id_ChatMessage,&cmd.as_str()[3..]);
    } else if cmd.starts_with("name") {
        return (message_id::Id_NewComer,&cmd.as_str()[4..]);
    } else if cmd.starts_with("list") {
        if cmd.len() > 4 {
            return (message_id::LocalId_Client_List,&cmd.as_str()[4..]);
        } else {
            return (message_id::LocalId_Client_List,"");
        }
    } else if cmd.starts_with("file") {
        return (message_id::Id_FileTransfer,&cmd.as_str()[4..]);
    } else if cmd.starts_with("exit") {
        return (message_id::Id_UserExit,"");
    } else if cmd.starts_with("broadcast") {
        return (message_id::Id_BroadCast,&cmd.as_str()[9..]);
    } else if cmd.starts_with("group") {
        return (message_id::LocalId_Client_Group,&cmd.as_str()[5..]);
    } else if cmd.starts_with("enter") {
        return (message_id::LocalId_Client_Enter,&cmd.as_str()[5..]);
    } else if cmd.starts_with("back") {
        return (message_id::LocalId_Client_Back,"");
    } else if cmd.starts_with("task") {
        return (message_id::LocalId_Client_Task,&cmd.as_str()[4..]);
    } else if cmd.starts_with("room") {
        return (message_id::Id_RoomMessage,&cmd.as_str()[4..]);
    } else if cmd.starts_with("history") {
        return (message_id::LocalId_Client_History,&cmd.as_str()[7..]);
    }
    
    return (message_id::Id_Message_Undefined,"")
}

pub fn parse_connect_command(cmd:&str)->Option<ConnectCommand> {
    let command = ConnectCommand::from_string(cmd.trim());
    if let None = command {
        ConnectCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_message_command(cmd:&str)->Option<MessageCommand> {
    let command = MessageCommand::from_string(cmd.trim());
    if let None = command {
        MessageCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_name_command(cmd:&str)->Option<NameCommand> {
    let command = NameCommand::from_string(cmd.trim());
    if let None = command {
        NameCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_list_command(cmd:&str)->Option<ListCommand> {
    let command = ListCommand::from_string(cmd.trim());
    if let None = command {
        ListCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_file_command(cmd:&str)->Option<FileCommand> {
    let command = FileCommand::from_string(cmd.trim());
    if let None = command {
        FileCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_broadcast_command(cmd:&str)->Option<BroadCastCommand> {
    let command = BroadCastCommand::from_string(cmd.trim());
    if let None = command {
        BroadCastCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_group_command(cmd:&str)->Option<GroupCommand> {
    let command = GroupCommand::from_string(cmd.trim());
    if let None = command {
        GroupCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_enter_command(cmd:&str)->Option<EnterCommand> {
    let command = EnterCommand::from_string(cmd.trim());
    if let None = command {
        EnterCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_task_command(cmd:&str)->Option<TaskCommand> {
    let command = TaskCommand::from_string(cmd.trim());
    if let None = command {
        TaskCommand::print_standard_command();
        return None;
    }

    return command;
}


pub fn parse_room_command(cmd:&str)->Option<RoomCommand> {
    let command = RoomCommand::from_string(cmd.trim());
    if let None = command {
        RoomCommand::print_standard_command();
        return None;
    }

    return command;
}

pub fn parse_history_command(cmd:&str)->Option<HistoryCommand> {
    let command = HistoryCommand::from_string(cmd.trim());
    if let None = command {
        HistoryCommand::print_standard_command();
        return None;
    }

    return command;
}