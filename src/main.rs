mod structures;
mod searcher;
mod server;
mod serializing;
use crate::server::start_server;
use structures::WordIndex;
use crate::structures::HasID;
use byte_array::{
    BinaryBuilder,
    ByteArray,
 };

 


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
    if false {start_server().await.unwrap() }
    let mut wi = WordIndex {
        id: 99,
        position: Vec::new(),
        freq: 0
    };
    
    wi.insert(22);
    wi.insert(23);
    wi.insert(21);
    wi.insert(18);
    wi.insert(33);
    let mut found = wi.get_child_by_id(21);
    match found.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    let mut ba = &mut ByteArray::new();
    println!("wi: {:?} {:?}", wi, found);
    let raw = wi.to_raw(ba);
    let wi2 = WordIndex::from_raw(ba);
    println!("Here is wi2: {:?}", wi2);
}





