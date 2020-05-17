#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod structures;
mod searcher;
mod server;
mod serializing;
mod test_structures;
use crate::server::start_server;
use structures::DocumentWordIndex;
use crate::structures::HasChildren;
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
}





