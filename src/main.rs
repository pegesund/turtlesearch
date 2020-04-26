extern crate rand;

// use hashbrown::{HashMap, HashSet};
// use std::collections::HashMap;
// use std::sync::RwLock;
// use std::sync::Arc;
// use fnv::FnvHashMap;
// use im::hashmap::HashMap;
use std::time::Instant;
use rand::Rng;
use evmap;
use std::thread;
use std::sync::Arc;


fn main() {
    let mut bvec = Vec::new();
    bvec.push(88);
    let mut vec = Arc::new(bvec);
    loop {
        for _i in 0..10 {
            let x = vec.clone();
            let handler = thread::spawn(move || {
                println!("Here is the vector: {:?}", &x);
                
            });
            handler.join().unwrap();
        }

        let n = Arc::get_mut(&mut vec).unwrap();
        n.push(99);

        println!("The clone: {:?}", vec.clone());
    }
}

#[allow(dead_code)]
fn main2() {

    let (reader, mut writer) = evmap::new();
    for i in 0..10000000i64 {
        writer.insert(i.to_string(), i);
    }
    let mut rng = rand::thread_rng();
    writer.refresh();
    let start = Instant::now();
    for j in 0..10 {
        println!("Number: {0}", j);
        for i in 0..10000000i64 {
            let the_clone = reader.clone();
            let r = rng.gen_range(0, 10000000i64);
            let res = the_clone.get(&r.to_string());
            if i == 500 {
                println!("Value is: {:?}", res);
            }}
    }
    let elapsed = start.elapsed();
    println!("Debug: {:?}", elapsed);
    println!("Done");
    
}

