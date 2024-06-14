use crate::chatroom::client::manager;
use crate::chatroom::input::command;


pub fn test_group_json_to_string() {
    // let mut group = manager::Group::new();
    // group.add_new_group(&String::from("group_name"));
    // group.add_new_user_to_group(&String::from("h1"), &String::from("group_name"));
    // group.add_new_user_to_group(&String::from("h2"), &String::from("group_name"));
    // group.add_new_user_to_group(&String::from("h3"), &String::from("group_name"));

    // group.add_new_group(&String::from("group_name1"));
    // group.add_new_user_to_group(&String::from("h11"), &String::from("group_name1"));
    // group.add_new_user_to_group(&String::from("h21"), &String::from("group_name1"));
    // group.add_new_user_to_group(&String::from("h31"), &String::from("group_name1"));
    // group.export(String::from("test.json"));

    // let group2 = manager::Group::import(String::from("test.json"));
    // group2.dump();
}

pub fn test_group_command_parse_1() {
    //group new abc
    {
        let command = " new abc";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace1 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
    }

    {
        let command = " new    abc   ";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace2 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
    }

    {
        let command = " new    a b c   ";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace3 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
    }

    //group erase abc
    {
        let command = " erase abc";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace4 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
    }

    {
        let command = " erase    abc   ";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace5 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
    }

    {
        let command = " erase    a b c   ";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace6 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
    }
}

pub fn test_group_command_parse_2() {
    {
        let command: &str = " myabc,add abc,abc2";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace1 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
        for user in group.get_user_names() {
            println!("user is {}",user);
        }
    }

    {
        let command: &str = " my abc , add abc";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace2 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
        for user in group.get_user_names() {
            println!("user is {}",user);
        }
    }

    {
        let command: &str = " my abc , add abc, abc2, dd,";
        let group = command::GroupCommand::from_string(command).unwrap();
        println!("trace3 group command is {},group name is |{}|",group.get_action(),group.get_group_name());
        for user in group.get_user_names() {
            println!("user is {}",user);
        }
    }
}

pub fn test_group_command_parse_invalid() {
/*
    {
        let command: &str = " new";
        let result = command::GroupCommand::from_string(command);
        match result {
            Some(command) => {
                println!("trace1 group command is {},group name is |{}|",command.get_action(),command.get_group_name());
            }
            None => {

            }
        }
    }
    
    {
        let command: &str = " afd";
        let result = command::GroupCommand::from_string(command);
        match result {
            Some(command) => {
                println!("trace2 group command is {},group name is |{}|",command.get_action(),command.get_group_name());
            }
            None => {

            }
        }
    }

    {
        let command: &str = " myabc,";
        let result = command::GroupCommand::from_string(command);
        match result {
            Some(command) => {
                println!("trace3 group command is {},group name is |{}|",command.get_action(),command.get_group_name());
            }
            None => {

            }
        }
    }

    {
        let command: &str = " myabc,add";
        let result = command::GroupCommand::from_string(command);
        match result {
            Some(command) => {
                println!("trace4 group command is {},group name is |{}|",command.get_action(),command.get_group_name());
            }
            None => {

            }
        }
    }
  
    {
        let command: &str = " myabc add";
        let result = command::GroupCommand::from_string(command);
        match result {
            Some(command) => {
                println!("trace4 group command is {},group name is |{}|",command.get_action(),command.get_group_name());
            }
            None => {

            }
        }
    }
 */  
    {
        let command: &str = " add ,abc";
        let result = command::GroupCommand::from_string(command);
        match result {
            Some(command) => {
                println!("trace4 group command is {},group name is |{}|",command.get_action(),command.get_group_name());
            }
            None => {

            }
        }
    }
}