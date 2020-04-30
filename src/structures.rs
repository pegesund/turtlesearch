#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use duplicate::duplicate;

use std::sync::{RwLock};
    
    #[allow(dead_code)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Clone)]
    pub enum SearchCommands {
        Update,
        Search,
        Die
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct SearchCommand {
        pub command: SearchCommands,
        pub person_id: String,
        pub param: String,
        pub result: Option<String>,
        pub result_channel: std::sync::mpsc::Sender<SearchCommand>
    }

    #[derive(Debug)]
    pub struct AppState {
        pub send_channel: async_std::sync::Sender<SearchCommand>,
        pub counter: RwLock<u64>
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct WordIndex {
        pub id: u64,
        pub position: Vec<u64>,
        pub freq: u64
    }    
    
    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct DocumentIndex {
        pub id: u64,
        pub words: Vec<WordIndex>
    }
    
    pub trait HasID <E: Debug + Clone + Ord + Copy> {
        fn get_id(&self) -> u64;
        fn get_vec(&mut self) -> &mut Vec<E>;
        fn get_vec_immutable(&self) -> &Vec<E>;
        fn insert(&mut self, element: E) -> () {
            let insert_pos = match self.get_vec().binary_search(&element) {
                Ok(_) => panic!("tried to insert duplicate in non duplicate vector!"),
                Err(pos) => pos
            };
            self.get_vec().insert(insert_pos, element);
            println!("New value: {:?}", self.get_vec());
        }
        fn get_child_by_id(&self, id: E) -> &E {
            
            let res = match &self.get_vec_immutable().binary_search(&id) {
                Ok(pos) => &self.get_vec_immutable()[*pos],
                Err(_) => panic!("not found, should not happend")
            };
            res
        
        }
    }
    
    #[duplicate(
        [ all_classes [ WordIndex ]]
        [ all_classes [ DocumentIndex ]]
    )]
    impl PartialEq for all_classes {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
    
    #[duplicate(
        [ all_classes [ WordIndex ]]
        [ all_classes [ DocumentIndex ]]
    )]
    impl PartialOrd for all_classes {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    #[duplicate(
        [ all_classes [ WordIndex ]]
        [ all_classes [ DocumentIndex ]]
    )]
    impl Ord for all_classes {
        fn cmp(&self, other: &Self) -> Ordering {
            self.id.cmp(&other.id)
        }
    }
    
    
    impl HasID<u64> for WordIndex  {
        fn get_id(&self) -> u64 { self.id }
        fn get_vec(&mut self) -> &mut Vec<u64> { &mut self.position }
        fn get_vec_immutable(&self) -> &Vec<u64> { &self.position }
    }

    


