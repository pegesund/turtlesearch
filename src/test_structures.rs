use crate::structures::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn between() {
        println!("---------- Testing between!");
        let is1 = IntegerSorted {
            value: 88,
            doc_ids: &vec![1,2,3,4,5]
        };
        let is2 = IntegerSorted {
            value: 100,
            doc_ids: &vec![1,2,3,4,5]
        };
        let is3 = IntegerSorted {
            value: 10,
            doc_ids: &vec![1,2,3,4,5]
        };
        let is4 = IntegerSorted {
            value: 1,
            doc_ids: &vec![1,2,3,4,5]
        };
        let is5 = IntegerSorted {
            value: 99,
            doc_ids: &vec![1,2,3,4,5]
        };

        let mut index = FieldIndex {
            name: "testfoeød".to_string(),
            index: vec![]
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
            freq: 0
        };
        let res1 = DocumentWordIndex {
            id: 10,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0
        };
        let res2 = DocumentWordIndex {
            id: 5,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0        };

        let res3 = DocumentWordIndex {
            id: 12,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0        };

        let doc_index = DocumentIndex {
            id: 0,
            words: Rc::new(RefCell::new(vec![]))
        };
        doc_index.insert(res0);
        doc_index.insert(res1);
        doc_index.insert(res2);
        doc_index.insert(res3);

        println!("DocumentWordIndex: {:?}", doc_index);
    }
}