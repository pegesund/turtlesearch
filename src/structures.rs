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

    #[duplicate(
        the_class sort_field;
        [ DocumentWordIndex ] [ doc_id ];
    )]
    impl PartialEq for the_class {
        fn eq(&self, other: &Self) -> bool {
            self.sort_field == other.sort_field
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










