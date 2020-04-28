extern crate rand;

// use hashbrown::{HashMap, HashSet};
// use std::collections::HashMap;
// use std::sync::RwLock;
// use std::sync::Arc;
// use fnv::FnvHashMap;
// use im::hashmap::HashMap;
// use std::time::Instant;
// use rand::Rng;
// use evmap;
use std::thread;
use std::sync::Arc;
use threadpool::ThreadPool;
use crossbeam_channel::bounded;
use std::time::Duration;
// use std::collections::BinaryHeap;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum SearchCommands {
    Update,
    Search,
    Die
}

#[derive(Debug)]
#[derive(Clone)]
struct SearchCommand {
    command: SearchCommands,
    person_id: String,
    param: String,
    result: Option<String>
}



fn main() {

    let (channel_sender, channel_receiver) = bounded(1000);
    let add_command = SearchCommand {
        command: SearchCommands::Update,
        person_id:  String::from("petter"),
        param:  String::from("book"),
        result: None
    };
    thread::spawn(move || 
        executor_loop(channel_receiver)
    );
    for _ in 0..10 {
        channel_sender.send(add_command.clone()).unwrap();
    }
    let read_command = SearchCommand {
        command: SearchCommands::Search,
        person_id:  String::from("petter"),
        param:  String::from("book"),
        result: None
    };

    

    thread::sleep(Duration::from_secs(2));
    channel_sender.send(read_command).unwrap();
    loop {}
}


fn executor_loop(receiver: crossbeam_channel::Receiver<SearchCommand>) {

    let pool = ThreadPool::with_name("worker".into(), 2);
    let bvec = Vec::new();
    let mut vec = Arc::new(bvec);
    loop {
        let mut commands: Vec<SearchCommand>;
        loop {
            commands = receiver.try_iter().collect();
            if commands.len() > 0 { break };    
        }
        
        println!("Got something: {:?}", commands);
        let read_commands: Vec<SearchCommand> = commands.iter().filter(|c| c.command == SearchCommands::Search).cloned().collect();
        let write_commands: Vec<SearchCommand> = commands.iter().filter(|c| c.command != SearchCommands::Search).cloned().collect();

        for read_command in read_commands {
            let cloned_vec = vec.clone();
            match read_command.command {
                SearchCommands::Search => {
                    let _handler = pool.execute(move || {
                        println!("Here is the vector: {:?}", &cloned_vec);
                    });      
                },
                _ => println!("Should never happend, non read command in read")
            }
        }
        pool.join();
        
        let mut_vec = Arc::get_mut(&mut vec).unwrap();
        for write_command in write_commands {
            match write_command.command {
                SearchCommands::Update => {
                    mut_vec.push(write_command.param.clone());                    
                    println!("Pushed: {:?}", &write_command.param);
                },
                SearchCommands::Die => break,
                _ => println!("Should never happend, 
                non read command in read")
            }
        }
    }
}

