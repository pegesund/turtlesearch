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

use crate::structures::DocumentWordIndex;



#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
 #[derive(Copy)]
pub struct FloatWrapper {
    pub value: f64
}

#[duplicate(
    the_class sort_field;
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
#[derive(Eq)]
#[derive(Clone)]
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

pub trait SortedVector<E: Debug + Clone + Ord> {
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

impl SortedVector<u32> for DocumentWordIndex {
    fn get_vec(&self) -> &Rc<RefCell<Vec<u32>>> {
        return &self.position;
    }
}

impl <'a> SortedVector<DocumentWordIndex> for WordSorted {
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

impl <'a> SortedVector<val_type> for the_class {
    fn get_vec(&self) -> &Rc<RefCell<Vec<val_type>>> {
        return &self.doc_ids;
    }
}




