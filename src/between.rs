pub trait Between<B: Clone + Debug + Ord> {
    fn between(&self, start: B, stop: B) -> (usize, usize);
}

pub trait GetValue<V: Clone + Debug + Ord> {
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

        let index = self.get_vec();


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



