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
    pub struct DocumentWordIndex {
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
        pub words: Vec<DocumentWordIndex>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct Word {
        pub id: u64,
        pub word: String,
        pub freq: u64,
        pub docs: Vec<DocumentIndex>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct WordIndex {
        pub id: u64,
        pub freq: u64,
        pub words: Vec<Word>
    }

pub trait HasID <E: Debug + Clone + Ord > {
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
        the_class sort_field;
        [ DocumentWordIndex ] [ id ];
        [ DocumentIndex ] [ id ];
        [ WordIndex ] [ id ];
        [ Word ] [ word ];
    )]
    impl PartialEq for the_class {
        fn eq(&self, other: &Self) -> bool {
            self.sort_field == other.sort_field
        }
    }

    #[duplicate(
        the_class;
        [ DocumentWordIndex ];
        [ DocumentIndex ];
        [ WordIndex ];
        [ Word ];
    )]
    impl PartialOrd for the_class {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    #[duplicate(
        the_class sort_field;
        [ DocumentWordIndex ] [ id ];
        [ DocumentIndex ] [ id ];
        [ WordIndex ] [ id ];
        [ Word ] [ word ];
    )]

    impl Ord for the_class {
        fn cmp(&self, other: &Self) -> Ordering {
            self.sort_field.cmp(&other.sort_field)
        }
    }
    
    
    impl HasID<u64> for DocumentWordIndex {
        fn get_vec_mut(&mut self) -> &mut Vec<u64> { &mut self.position }
        fn get_vec(&self) -> &Vec<u64> { &self.position }
    }

    impl HasID<DocumentWordIndex> for DocumentIndex  {
        fn get_vec_mut(&mut self) -> &mut Vec<DocumentWordIndex> { &mut self.words }
        fn get_vec(&self) -> &Vec<DocumentWordIndex> { &self.words }
    }

    impl HasID<DocumentIndex> for Word  {
        fn get_vec_mut(&mut self) -> &mut Vec<DocumentIndex> { &mut self.docs }
        fn get_vec(&self) -> &Vec<DocumentIndex> { &self.docs }
    }

/*
    impl HasID<Word> for WordIndex {
        fn get_vec_mut(&mut self) -> &mut Vec<Word> { &mut self.words }
        fn get_vec(&self) -> &Vec<Word> { &self.words }
    }
*/
