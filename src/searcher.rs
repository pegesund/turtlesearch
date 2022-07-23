use std::sync::Arc;
use threadpool::ThreadPool;
use async_std::task;
use std::thread;

use crate::web_glue::{SearchCommands, SearchCommand};


pub fn main_search(channel_receiver: async_std::sync::Receiver<SearchCommand>) {
    println!("Entering here");
    thread::spawn(move || {
        task::block_on(real_main(channel_receiver));    
    });
}

async fn real_main(channel_receiver: async_std::sync::Receiver<SearchCommand>) {
    // let (channel_sender, channel_receiver) = channel(1000);


    
    let channel_receiver_c = channel_receiver.clone();
/*

    let add_command = SearchCommand {
        command: SearchCommands::Update,
        person_id:  String::from("petter"),
        param:  String::from("book"),
        result: None,
        result_channel: result_sender.clone()
    };

    let read_command = SearchCommand {
        command: SearchCommands::Search,
        person_id:  String::from("petter"),
        param:  String::from("book"),
        result: None,
        result_channel: result_sender.clone()
    };

    let _die_command = SearchCommand {
        command: SearchCommands::Die,
        person_id:  String::from(""),
        param:  String::from(""),
        result: None,
        result_channel: result_sender.clone()
    };

    channel_sender.send(add_command.clone()).await;
    channel_sender.send(read_command.clone()).await;
    // channel_sender.send(die_command.clone()).await;

*/
    let task1 = executor_loop(channel_receiver_c);
    task::block_on(async move {
        let future1 = task::spawn(task1);
        // futures::join!(future1);
    });

    println!("Program ended!");
}


async fn executor_loop(receiver: async_std::sync::Receiver<SearchCommand>) {
    let pool = ThreadPool::with_name("worker".into(), 4);
    let bvec = Vec::new();
    let mut vec = Arc::new(bvec);
    loop {
        let execute_command = receiver.recv().await.unwrap();

        match execute_command.command {
            SearchCommands::Search => {
                let _handler = pool.execute(move || {
                    // println!("Here is the vector: {:?}", &cloned_vec);
                    let mut res =  execute_command.clone();
                    res.result = Some(String::from("hi there"));
                    res.result_channel.send(res.clone()).unwrap();   
                }); 
            },
            SearchCommands::Update => {
                pool.join();        
                let mut_vec = Arc::get_mut(&mut vec).unwrap();
                mut_vec.push(execute_command.param.clone());                    
                println!("Pushed: {:?}", &execute_command.param);
            },
            SearchCommands::Die => {    
                pool.join();
                execute_command.result_channel.clone().send(execute_command).unwrap();    
                break
            }
        }
    }
}
