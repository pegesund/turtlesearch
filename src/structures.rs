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
    
    pub trait HasID <E: Debug + Clone + Ord> {
        fn get_id(&self) -> u64;
        fn get_vec_mut(&mut self) -> &mut Vec<E>;
        fn get_vec(&self) -> &Vec<E>;
        fn insert(&mut self, element: E) -> () {
            let insert_pos = match self.get_vec_mut().binary_search(&element) {
                Ok(_) => panic!("tried to insert duplicate in non duplicate vector!"),
                Err(pos) => pos
            };
            self.get_vec_mut().insert(insert_pos, element);
        }
        fn get_child_by_id(&self, id: E) -> Option<&E> {
            let res = match &self.get_vec().binary_search(&id) {
                Ok(pos) => Some (&self.get_vec()[*pos]),
                Err(_) => None
            };
            res
        }
        fn get_child_by_id_mut(&mut self, id: E) -> Option<&mut E> {
            let res = match self.get_vec_mut().binary_search(&id) {
                Ok(pos) => Some (&mut self.get_vec_mut()[pos]),
                Err(_) => None
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
        fn get_vec_mut(&mut self) -> &mut Vec<u64> { &mut self.position }
        fn get_vec(&self) -> &Vec<u64> { &self.position }
    }

    impl HasID<WordIndex> for DocumentIndex  {
        fn get_id(&self) -> u64 { self.id }
        fn get_vec_mut(&mut self) -> &mut Vec<WordIndex> { &mut self.words }
        fn get_vec(&self) -> &Vec<WordIndex> { &self.words }
    }


    


