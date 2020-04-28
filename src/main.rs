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
use std::sync::Arc;
use threadpool::ThreadPool;
// use std::time::Duration;
// use std::collections::BinaryHeap;
use async_std::sync::channel;
// use futures::executor::block_on;
// use futures::excutor::join;
// use futures::future::Future;
use async_std::task;
use std::thread;
// use futures::future::join;

// use async_std::task;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    main_search();
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

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
    result: Option<String>,
    result_channel: async_std::sync::Sender<SearchCommand>
}

fn main_search() {
    println!("Entering here");
    let t = thread::spawn(move || {
        task::block_on(real_main());    
    });
}

async fn real_main() {
    let (channel_sender, channel_receiver) = channel(1000);
    let (result_sender, result_receiver) = channel(1000);

    
    let channel_receiver_c = channel_receiver.clone();

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

    let die_command = SearchCommand {
        command: SearchCommands::Die,
        person_id:  String::from(""),
        param:  String::from(""),
        result: None,
        result_channel: result_sender.clone()
    };

    channel_sender.send(add_command.clone()).await;
    channel_sender.send(read_command.clone()).await;
    channel_sender.send(die_command.clone()).await;

    let task1 = executor_loop(channel_receiver_c);
    let task2 = result_loop(result_receiver);
    task::block_on(async move {
        let future1 = task::spawn(task1);
        let future2 = task::spawn(task2);
        futures::join!(future2, future1);
    });

    println!("Program ended!");
}

async fn result_loop(result_receiver: async_std::sync::Receiver<SearchCommand>) {
    loop 
    {
        let res = result_receiver.clone().recv().await.unwrap();
        match res.command {
            SearchCommands::Die => {
                println!("I died");
                break;
            },
            _ => println!("People alwayas say so much unrelevant")
        }
    }
}

async fn executor_loop(receiver: async_std::sync::Receiver<SearchCommand>) {
    let pool = ThreadPool::with_name("worker".into(), 2);
    let bvec = Vec::new();
    let mut vec = Arc::new(bvec);
    loop {
        let execute_command = receiver.recv().await.unwrap();

        match execute_command.command {
            SearchCommands::Search => {
                let cloned_vec = vec.clone();
                let _handler = pool.execute(move || {
                    println!("Here is the vector: {:?}", &cloned_vec);
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
                execute_command.result_channel.clone().send(execute_command).await;    
                break
            }
        }
    }
}

