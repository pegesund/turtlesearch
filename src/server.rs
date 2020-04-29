use crate::searcher::main_search;
use crate::structures::*;
use actix_web::{web, App, HttpServer};
use async_std::sync::channel;
use std::sync::mpsc::channel as sync_channel;



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


pub async fn start_server() -> std::io::Result<()> {
    let (sender, receiver) = channel(1000);
    main_search(receiver);
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                send_channel: sender.clone()
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}