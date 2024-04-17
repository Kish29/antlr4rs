use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{LockResult, RwLock};
use crate::misc::murmur3::MurmurHash;

#[derive(Debug)]
pub struct JStore<T: Debug> {
    store: RwLock<HashMap<u32, Vec<T>>>,
    // len is the number of total elements
    len: usize,
}

impl<T: MurmurHash + PartialEq + Debug> Default for JStore<T> {
    fn default() -> Self {
        JStore { store: RwLock::new(HashMap::with_capacity(32)), len: 0 }
    }
}

impl<T: MurmurHash + PartialEq + Debug> JStore<T> {
/*    pub fn put(&mut self, val: T) -> (&T, bool) {
        let kh = val.murmur();
        match self.store.write() {
            Ok(mut st) => {
                match st.get_mut(&kh) {
                    None => {

                    }
                    Some(_) => {}
                }
                if st.contains_key(&kh) {
                    let ori = st.get(&kh).unwrap();
                    for t in ori {
                        if t.eq(&val) {
                            return (t, true);
                        }
                    }
                }
            }
            Err(e) => {
                panic!("jstore write failed: {:?}", e)
            }
        }
        match self.store.get(&kh) {
            None => {
                let v = &val;
                self.store.insert(kh, vec![val]);
                self.len += 1;
                return (v, false);
            }
            Some(ori) => {}
        }
        let v = &val;
        let mut ori = self.store.get_mut(&kh).unwrap();
        ori.push(val);
        (v, false)
    }*/

    pub fn get(&self, key: &T) -> Option<&T> {
        todo!()
    }
}