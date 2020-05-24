use crate::structures::*;


#[cfg(test)]
mod tests {
    use super::*;
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

        // index.between(&70, &99);



    }
}