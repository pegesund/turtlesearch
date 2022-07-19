#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use duplicate::duplicate;


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


    impl PartialEq for DocumentWordIndex {
        fn eq(&self, other: &Self) -> bool {
            self.doc_id == other.doc_id
        }
    }


    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct DocumentWordIndex {
        pub doc_id: u64,
        pub position: Rc<RefCell<Vec<u32>>>,
    } 

    #[duplicate(
    the_class;
    [ DocumentWordIndex ];
)]

impl <'a> PartialOrd for the_class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



impl <'a> Ord for DocumentWordIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.doc_id.cmp(&other.doc_id)
    }
}










