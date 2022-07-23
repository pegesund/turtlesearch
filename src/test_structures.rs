// use crate::structures::*;
use crate::{sorted_vector::*, structures::Field};


#[cfg(test)]
mod tests {
    use crate::structures::{DocumentWordAndPositions, FieldIndex}; 

    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::borrow::BorrowMut;


    #[test]
    fn between() {
        println!("---------- Testing between!");
        let mut is1 = IntegerSorted {
            value: 0,
            doc_ids: vec![]
        };
        is1.insert(1);
        is1.insert(2);

        let mut is2 = IntegerSorted {
            value: 0,
            doc_ids: vec![]
        };
        is2.insert(1);
        is2.insert(2);

        let is3 = IntegerSorted {
            value: 0,
            doc_ids: vec![]
        };
        is2.insert(1);
        is2.insert(2);

        let is4 = IntegerSorted {
            value: 0,
            doc_ids: vec![]
        };
        is2.insert(1);
        is2.insert(2);

        let is5 = IntegerSorted {
            value: 0,
            doc_ids: vec![]
        };
        is2.insert(1);
        is2.insert(2);


        let mut index = FieldIndex {
            // added comment
            name: "".to_string(),
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
        let res0 = DocumentWordAndPositions {
            doc_id: 0,
            position: vec![]
        };
        let res1 = DocumentWordAndPositions {
            doc_id: 10,
            position: vec![]
        };
        let res2 = DocumentWordAndPositions {
            doc_id: 5,
            position: vec![]
        };



        let res3 = DocumentWordAndPositions {
            doc_id: 12,
            position: vec![]
        };



        let mut doc_index = WordSorted {
            value:"hupp".to_string(),
            freq: 100,
            docs: vec![],
            optimized: false
        };
        doc_index.insert(res0);
        doc_index.insert(res1);
        doc_index.insert(res2);
        doc_index.insert(res3);


        println!("DocumentWordIndex: {:?}", doc_index);
        {
            let zero = &mut doc_index.get_vec()[0];
            println!("Zero: {:?}", zero);
            zero.insert(888);
            zero.insert(88);
            zero.insert(8888);
        }
        println!("DocumentWordIndex: {:?}", doc_index);

        {
            let zero = &mut doc_index.get_vec()[0];
            let zero_children = &mut zero.get_vec().to_vec();
            assert_eq!(zero_children.to_vec(), vec![88,888,8888])
        }

        {
            let children = &doc_index.get_vec().iter().map(|e: &DocumentWordAndPositions| e.doc_id).collect::<Vec<u64>>();
            assert_eq!(children, &vec![0, 5, 10, 12])
        }


    }
}

