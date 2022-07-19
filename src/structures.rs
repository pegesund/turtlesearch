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
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Document {
        pub id: u64,
        pub len: u32
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct DocumentContainer {
        pub id: u64,
        pub docs: Rc<RefCell<Vec<Document>>>
    }



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
     #[derive(Copy)]
    pub struct FloatWrapper {
        pub value: f64
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct DocumentWordIndex {
        pub doc_id: u64,
        pub position: Rc<RefCell<Vec<u32>>>,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct WordSorted {
        pub value: String,
        pub freq: u64,
        pub docs: Rc<RefCell<Vec<DocumentWordIndex>>>,
        pub optimized: bool
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct IntegerSorted {
        pub value: i64,
        pub doc_ids: Rc<RefCell<Vec<u64>>>
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct FloatSorted {
        pub value: FloatWrapper,
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
    pub struct BoolSorted {
        pub value: bool,
        pub doc_ids: Rc<RefCell<Vec<u64>>>
    }




    #[duplicate(
        the_class sort_field;
        [ DocumentWordIndex ] [ doc_id ];
        [ IntegerSorted ] [ value ];
        [ DateSorted ] [ value ];
        [ FloatSorted ] [ value ];
        [ BoolSorted ] [ value ];


    )]
    impl PartialEq for the_class {
        fn eq(&self, other: &Self) -> bool {
            self.sort_field == other.sort_field
        }
    }

    #[duplicate(
    the_class sort_field;
    [ WordSorted ] [ value ];
    )]

    impl <'a> PartialEq for the_class {
        fn eq(&self, other: &Self) -> bool {
            self.sort_field == other.sort_field
        }
}

    impl PartialEq for FloatWrapper {
        fn eq(&self, other: &Self) -> bool {
            self.value.approx_eq(other.value, (0.0, 2))
        }
    }

    impl Eq for FloatWrapper {

    }

    #[duplicate(
        the_class;
        [ DocumentWordIndex ];
        [ WordSorted ];
        [ FloatSorted ];
        [ IntegerSorted ];
        [ DateSorted ];
        [ BoolSorted ];
    )]

    impl <'a> PartialOrd for the_class {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialOrd for FloatWrapper {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let c = self.value.partial_cmp(&other.value);
            return c;
        }
    }


    #[duplicate(
        the_class sort_field;
        [ DocumentWordIndex ] [ doc_id ];
        [ WordSorted ] [ value ];
        [ IntegerSorted ] [ value ];
        [ DateSorted  ] [ value ];
        [ FloatSorted  ] [ value ];
        [ BoolSorted  ] [ value ];
    )]

    impl <'a> Ord for the_class {
        fn cmp(&self, other: &Self) -> Ordering {
            self.sort_field.cmp(&other.sort_field)
        }
    }

    impl Ord for FloatWrapper {
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
    [ IntegerSorted  ] [ i64 ];
    [ DateSorted  ] [ u64 ];
    [ BoolSorted  ] [ bool ];
    [ FloatSorted ] [ FloatWrapper ];
    )]
    impl <'a> GetValue<val_type> for the_class {
        fn get_value(&self) -> val_type {
            return self.value;
        }
    }

    #[duplicate(
    the_class val_type;
    [ IntegerSorted ] [ i64 ];
    [ DateSorted ][ u64 ];
    [ FloatSorted ][ FloatWrapper ];
    )]
    impl <'a> Between<val_type> for FieldIndex<the_class> {

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

    fn delete(&self, element: &E) {
        let delete_pos = match self.get_vec().as_ref().borrow().binary_search(&element) {
            Ok(pos) => Some(pos),
            Err(pos) => None
        };

        match delete_pos {
            Some(pos) =>  { (*(*self.get_vec())).borrow_mut().remove(pos); () },
            _ => ()
        };
    }
}


pub trait HasSortKey<I: Ord + Debug + Clone> {
    fn sort_key(&self) -> I;
}


impl HasChildrenNew<u32> for DocumentWordIndex {
    fn get_vec(&self) -> &Rc<RefCell<Vec<u32>>> {
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
    [ BoolSorted ] [ u64 ];
    [ FloatSorted ] [ u64 ];
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



