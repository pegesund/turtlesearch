use std::fmt::{Debug, Result};
use std::cmp::Ordering;

use crate::comparator::StoreValue;
pub trait DocumentStore<G: Clone + Debug + Eq> {
    fn put(&self, key: &[u8], value: &[u8]) -> Result;
    fn get(&self, key: &[u8]) -> G;
}
