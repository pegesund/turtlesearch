use crate::structures::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::borrow::BorrowMut;
    use std::ptr;

    #[test]
    fn between() {
        println!("---------- Testing between!");
        let is1 = IntegerSorted {
            value: 0,
            doc_ids: Rc::new(RefCell::new(vec![]))
        };
        is1.insert(1);
        is1.insert(2);

        let is2 = IntegerSorted {
            value: 0,
            doc_ids: Rc::new(RefCell::new(vec![]))
        };
        is2.insert(1);
        is2.insert(2);

        let is3 = IntegerSorted {
            value: 0,
            doc_ids: Rc::new(RefCell::new(vec![]))
        };
        is2.insert(1);
        is2.insert(2);

        let is4 = IntegerSorted {
            value: 0,
            doc_ids: Rc::new(RefCell::new(vec![]))
        };
        is2.insert(1);
        is2.insert(2);

        let is5 = IntegerSorted {
            value: 0,
            doc_ids: Rc::new(RefCell::new(vec![]))
        };
        is2.insert(1);
        is2.insert(2);


        let index = FieldIndex {
            // added comment
            name: "".to_string(),
            index: Rc::new(RefCell::new(vec![]))
        };

        index.insert(is1);
        index.insert(is2);
        index.insert(is3);
        index.insert(is4);
        index.insert(is5);

        println!("Vector: {:?}", index);
    }

    #[test]
    fn test_new_children() {
        let res0 = DocumentWordIndex {
            id: 0,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0,
            doc: ptr::null_mut()
        };
        let res1 = DocumentWordIndex {
            id: 10,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0,
            doc: ptr::null_mut()
        };
        let res2 = DocumentWordIndex {
            id: 5,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0,
            doc: ptr::null_mut()
        };


        let mut doc1 = Document {
            id: 0,
            len: 1000
        };

        let res3 = DocumentWordIndex {
            id: 12,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0,
            doc: &mut doc1
        };


        unsafe {
            let doc1 = &mut *res3.doc;
            assert_eq!(doc1.len, 1000);
        }

        let doc_index = WordSorted {
            value:"hupp".to_string(),
            freq: 100,
            docs: Rc::new(RefCell::new(vec![])),
            optimized: false
        };
        doc_index.insert(res0);
        doc_index.insert(res1);
        doc_index.insert(res2);
        doc_index.insert(res3);


        println!("DocumentWordIndex: {:?}", doc_index);
        {
            let zero = &mut doc_index.get_vec().as_ref().borrow_mut()[0];
            println!("Zero: {:?}", zero);
            zero.freq = 88;
            zero.insert(888);
            zero.insert(88);
            zero.insert(8888);
        }
        println!("DocumentWordIndex: {:?}", doc_index);

        {
            let zero = &doc_index.get_vec().as_ref().borrow()[0];
            assert_eq!(zero.freq, 88);
            let zero_children = zero.get_vec().as_ref().borrow().to_vec();
            assert_eq!(zero_children, vec![88,888,8888])
        }

        {
            let children = &doc_index.get_vec().as_ref().borrow().iter().map(|e: &DocumentWordIndex| e.id).collect::<Vec<u64>>();
            assert_eq!(children, &vec![0, 5, 10, 12])
        }


    }
}