mod structures;
mod searcher;
mod server;
use crate::server::start_server;

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

// use async_std::task;
// use actix_web::{web, App, HttpServer};


#[actix_rt::main]
async fn main() {
    start_server().await.unwrap();
}





