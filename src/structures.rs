#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use duplicate::duplicate;

use std::sync::{RwLock};
use float_cmp::ApproxEq;


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
    pub struct WordSorted<'a> {
        pub value:  &'a str,
        pub freq: u64,
        pub docs: Vec<DocumentIndex>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    #[derive(Copy)]
    pub struct IntegerSorted<'a> {
        pub value: u64,
        pub doc_ids: &'a Vec<u64>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct FloatSorted {
        pub value: f64,
        pub doc_ids: Vec<u64>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct DateSorted {
        pub value: u64,
        pub doc_ids: Vec<u64>
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct WordIndex<'a> {
        pub id: u64,
        pub freq: u64,
        pub words: Vec<WordSorted<'a>>
    }



    pub trait HasChildren<E: Debug + Clone + Ord > {
        fn get_vec_mut(&mut self) -> &mut Vec<E>;
        fn get_vec(&self) -> &Vec<E>;
        fn insert(&mut self, element: E) -> () {
            let insert_pos = match self.get_vec_mut().binary_search(&element) {
                Ok(pos) => pos,
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
        [ IntegerSorted<'_> ] [ value ];
        [ DateSorted ] [ value ];

    )]
    impl PartialEq for the_class {
        fn eq(&self, other: &Self) -> bool {
            self.sort_field == other.sort_field
        }
    }

    #[duplicate(
    the_class sort_field;
    [ WordIndex<'a> ] [ id ];
    [ WordSorted<'a> ] [ value ];
    )]

    impl <'a> PartialEq for the_class {
        fn eq(&self, other: &Self) -> bool {
            self.sort_field == other.sort_field
        }
}



    impl PartialEq for FloatSorted {
        fn eq(&self, other: &Self) -> bool {
            self.value.approx_eq(other.value, (0.0, 2))
        }
    }

    impl Eq for FloatSorted {

    }

    #[duplicate(
        the_class;
        [ DocumentWordIndex ];
        [ DocumentIndex ];
        [ WordIndex<'a> ];
        [ WordSorted<'a> ];
        [ FloatSorted ];
        [ IntegerSorted<'_> ];
        [ DateSorted ];
    )]

    impl <'a> PartialOrd for the_class {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    #[duplicate(
        the_class sort_field;
        [ DocumentWordIndex ] [ id ];
        [ DocumentIndex ] [ id ];
        [ WordIndex<'a> ] [ id ];
        [ WordSorted<'a> ] [ value ];
        [ IntegerSorted<'_> ] [ value ];
        [ DateSorted ] [ value ];
    )]

    impl <'a> Ord for the_class {
        fn cmp(&self, other: &Self) -> Ordering {
            self.sort_field.cmp(&other.sort_field)
        }
    }

    impl Ord for FloatSorted {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value.partial_cmp(&other.value).unwrap()
        }
    }


    impl HasChildren<u64> for DocumentWordIndex {
        fn get_vec_mut(&mut self) -> &mut Vec<u64> { &mut self.position }
        fn get_vec(&self) -> &Vec<u64> { &self.position }
    }

    impl HasChildren<DocumentWordIndex> for DocumentIndex  {
        fn get_vec_mut(&mut self) -> &mut Vec<DocumentWordIndex> { &mut self.words }
        fn get_vec(&self) -> &Vec<DocumentWordIndex> { &self.words }
    }


    impl <'a> HasChildren<WordSorted<'a>> for WordIndex<'a> {
        fn get_vec_mut(&mut self) -> &mut Vec<WordSorted<'a>> { &mut self.words }
        fn get_vec(&self) -> &Vec<WordSorted<'a>> { &self.words }
    }

    impl <'a> HasChildren<DocumentIndex> for WordSorted<'a> {
        fn get_vec_mut(&mut self) -> &mut Vec<DocumentIndex> { &mut self.docs }
        fn get_vec(&self) -> &Vec<DocumentIndex> { &self.docs }
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]

    pub struct FieldIndex<G:Debug + Clone + Ord > {
        pub name: String,
        pub index: Vec<G>
    }

    impl<G: Clone + Ord + Debug> HasChildren<G> for FieldIndex<G> {
        fn get_vec_mut(&mut self) -> &mut Vec<G> { &mut self.index }
        fn get_vec(&self) -> &Vec<G> { &self.index }
    }


    pub trait Between<B: Clone + Ord + Debug> {
        fn between(&self, start: B, stop: B) -> (usize, usize);
    }

    pub trait GetValue<V: Clone + Debug > {
        fn get_value(&self) -> V;
    }

    #[duplicate(
    the_class val_type;
    [ IntegerSorted<'_> ] [ u64 ];
    [ DateSorted ] [ u64 ];
    [ FloatSorted ] [ f64 ];
    )]
    impl GetValue<val_type> for the_class {
        fn get_value(&self) -> val_type {
            return self.value;
        }
    }




    impl Between<u64> for FieldIndex<IntegerSorted<'_>> {

        fn between(&self,start: u64, stop: u64) -> (usize, usize) {

            let mut start_index = match self.get_vec().binary_search_by_key(&start, |&e| e.value) {
                Ok(pos) => pos,
                Err(pos) => pos
            };

            let stop_index = match self.get_vec().binary_search_by_key(&start, |&e| e.value) {
                Ok(pos) => pos,
                Err(pos) => pos
            };

            while self.get_vec()[start_index].value == start && start_index > 0{
                start_index = start_index - 1
            }

            while self.get_vec()[stop_index].value == stop && stop_index < self.index.len() - 1 {
                start_index = start_index + 1
            }

            return (start_index, stop_index)

        }
    }
/*
    impl GetValue<&str> for WordSorted<'_> {
        fn get_value(&self) -> &str {
            return self.value;
        }
    }
*/

