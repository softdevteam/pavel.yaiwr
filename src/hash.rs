use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait HashId {
    fn id(&self) -> u64;
}

impl HashId for String {
    fn id(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl HashId for str {
    fn id(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}
