#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use duplicate::duplicate;

use std::sync::{RwLock};
use float_cmp::ApproxEq;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::{BorrowMut, Borrow, Cow};

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
        pub position: Rc<RefCell<Vec<u64>>>,
        pub freq: u64
    }    

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct WordSorted {
        pub value: String,
        pub freq: u64,
        pub docs: Rc<RefCell<Vec<DocumentWordIndex>>>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct ChildrenDocs {
        pub value: u64,
        pub doc_ids: Rc<RefCell<Vec<u64>>>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct ParentDocs {
        pub value: u64,
        pub doc_ids: Rc<RefCell<Vec<u64>>>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct IntegerSorted {
        pub value: u64,
        pub doc_ids: Rc<RefCell<Vec<u64>>>
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct FloatSorted {
        pub value: f64,
        pub doc_ids: Rc<RefCell<Vec<u64>>>
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct DateSorted {
        pub value: u64,
        pub doc_ids: Rc<RefCell<Vec<u64>>>
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct WordIndex {
        pub id: u64,
        pub freq: u64,
        pub words: Rc<RefCell<Vec<WordSorted>>>
    }


    #[duplicate(
        the_class sort_field;
        [ DocumentWordIndex ] [ id ];
        [ IntegerSorted ] [ value ];
        [ DateSorted ] [ value ];
        [ ParentDocs ] [ doc_ids ];
        [ ChildrenDocs ] [ doc_ids ];


    )]
    impl PartialEq for the_class {
        fn eq(&self, other: &Self) -> bool {
            self.sort_field == other.sort_field
        }
    }

    #[duplicate(
    the_class sort_field;
    [ WordIndex ] [ id ];
    [ WordSorted ] [ value ];
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
        [ WordIndex ];
        [ WordSorted ];
        [ FloatSorted ];
        [ IntegerSorted ];
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
        [ WordIndex ] [ id ];
        [ WordSorted ] [ value ];
        [ IntegerSorted ] [ value ];
        [ DateSorted  ] [ value ];
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

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]

    pub struct FieldIndex<G:Debug + Clone + Ord > {
        pub name: String,
        pub index:  Rc<RefCell<Vec<G>>>
    }


    pub trait Between<B: Clone + Debug> {
        fn between(&self, start: B, stop: B) -> (usize, usize);
    }

    pub trait GetValue<V: Clone + Debug > {
        fn get_value(&self) -> V;
    }

    #[duplicate(
    the_class val_type;
    [ IntegerSorted  ] [ u64 ];
    [ DateSorted  ] [ u64 ];
    [ FloatSorted ] [ f64 ];
    )]
    impl <'a> GetValue<val_type> for the_class {
        fn get_value(&self) -> val_type {
            return self.value;
        }
    }

    #[duplicate(
    the_class val_type;
    [ IntegerSorted ] [ u64 ];
    [ DateSorted ][ u64 ];
    )]
    impl <'a> Between<u64> for FieldIndex<the_class> {

        fn between(&self,start: val_type, stop: val_type) -> (usize, usize) {

            let index = self.get_vec().as_ref().borrow();


            let mut start_index = match index.binary_search_by_key(&start, |e| e.value) {
                Ok(pos) => pos,
                Err(pos) => pos
            };

            let stop_index = match index.binary_search_by_key(&start, |e| e.value) {
                Ok(pos) => pos,
                Err(pos) => pos
            };

            while index[start_index].value == start && start_index > 0{
                start_index = start_index - 1
            }


            while index[stop_index].value == stop && stop_index < index.len() - 1 {
                start_index = start_index + 1
            }

            return (start_index, stop_index)

        }
    }


impl Between<f64> for FieldIndex<FloatSorted> {

    fn between(&self,start: f64, stop: f64) -> (usize, usize) {

        let index = self.get_vec().as_ref().borrow();

        let mut start_index = match index.binary_search_by(|e| e.value.partial_cmp(&start).unwrap() ) {
            Ok(pos) => pos,
            Err(pos) => pos
        };

        let stop_index = match index.binary_search_by(|e| e.value.partial_cmp(&stop).unwrap() ) {
            Ok(pos) => pos,
            Err(pos) => pos
        };

        while index[start_index].value.approx_eq(start, (0.0, 2)) && start_index > 0{
            start_index = start_index - 1
        }

        while index[stop_index].value.approx_eq(stop, (0.0, 2)) && stop_index < index.len() - 1 {
            start_index = start_index + 1
        }

        return (start_index, stop_index)

    }
}

pub trait HasChildrenNew<E: Debug + Clone + Ord> {
    fn get_vec(&self) -> &Rc<RefCell<Vec<E>>>;

    fn insert(&self, element: E) -> () {
        let insert_pos = match self.get_vec().as_ref().borrow().binary_search(&element) {
            Ok(pos) => pos,
            Err(pos) => pos
        };
        {
            (*(*self.get_vec())).borrow_mut().insert(insert_pos, element);
        }
    }
}


impl HasChildrenNew<u64> for DocumentWordIndex {
    fn get_vec(&self) -> &Rc<RefCell<Vec<u64>>> {
        return &self.position;
    }
}

impl <'a> HasChildrenNew<DocumentWordIndex> for WordSorted {
    fn get_vec(&self) -> &Rc<RefCell<Vec<DocumentWordIndex>>> {
        return &self.docs;
    }
}


#[duplicate(
the_class val_type;
    [ IntegerSorted ] [ u64 ];
    [ DateSorted ] [ u64 ];
    [ FloatSorted ] [ u64 ];
    [ ChildrenDocs ] [ u64 ];
    [ ParentDocs ] [ u64 ];
)]

impl <'a> HasChildrenNew<val_type> for the_class {
    fn get_vec(&self) -> &Rc<RefCell<Vec<val_type>>> {
        return &self.doc_ids;
    }
}

impl<G: Debug + Clone + Ord > HasChildrenNew<G> for FieldIndex<G> {
    fn get_vec(&self) -> &Rc<RefCell<Vec<G>>> {
        return &self.index;
    }
}