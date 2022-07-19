use crate::searcher::main_search;
use crate::structures::*;
use crate::web_glue::*;
use actix_web::{web, App, HttpServer};
use async_std::sync::channel;
use std::sync::mpsc::channel as sync_channel;
use std::sync::{RwLock};



async fn index(data: web::Data<AppState>) -> String {
    let sender = &data.send_channel; // <- get app_name
    let (result_sender, result_receiver) = sync_channel();
    let read_command = SearchCommand {
        command: SearchCommands::Search,
        person_id:  String::from("petter"),
        param:  String::from("book"),
        result: None,
        result_channel: result_sender.clone()
    };
    sender.send(read_command).await;
    let from_search = result_receiver.recv().unwrap();

    format!("Hello {}!", from_search.result.unwrap()) // <- response with app_name
}

async fn index2(data: web::Data<AppState>) -> String {
    let read_lock = &data.counter;
    let val = read_lock.read().unwrap();
    format!("Hello {}!", *val) 
}

async fn index3(data: web::Data<AppState>) -> String {
    let write_lock = &data.counter;
    let val = write_lock.write();
    match val {
        Ok(mut n) => {*n = *n + 1},
        Err(_) => println!("It was an err"),
    };
    format!("Hello updated") 
}


pub async fn start_server() -> std::io::Result<()> {

    let (sender, receiver) = channel(1000);
    main_search(receiver);

    let state = web::Data::new(AppState {
        send_channel: sender.clone(),
        counter: RwLock::new(0),
    });


    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(state.clone()) // <- register the created data
            .route("/", web::get().to(index2))
            .route("/write", web::get().to(index3))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

