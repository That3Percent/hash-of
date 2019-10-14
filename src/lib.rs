use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct HashOf<T, H: Hasher = DefaultHasher> {
    hash: u64,
    _marker: PhantomData<*const (T, H)>, // Indicate we do not own anything
}

// Manually implementing some traits because they are not implemented when T or H do not
// implement them. In particular, DefaultHasher doesn't implement PartialEq
impl<T, H: Hasher> PartialEq for HashOf<T, H> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<T, H: Hasher> Eq for HashOf<T, H> {}

impl<T, H: Hasher> Hash for HashOf<T, H> {
    #[inline]
    fn hash<H2: Hasher>(&self, state: &mut H2) {
        state.write_u64(self.hash)
    }
}

impl<T> Clone for HashOf<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for HashOf<T> {}

impl<T> HashOf<T> {
    #[inline]
    pub fn to_inner(self) -> u64 {
        self.hash
    }
}

// Example types to explain the confusing signature...
// T: str
// Q: String
impl<T: Hash + ?Sized, Q: Borrow<T>, H: Hasher + Default> From<&T> for HashOf<Q, H> {
    fn from(value: &T) -> Self {
        let mut hasher = H::default();
        value.hash(&mut hasher);
        Self {
            hash: hasher.finish(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_hash_eq() {
        let x = HashOf::<String>::from("test");
        let s2 = String::new() + "test";
        let y = HashOf::from(&s2);
        assert_eq!(x, y);
    }

    #[test]
    fn can_hash_non_borrow() {
        // TODO: Have this work for non-borrow
        let x = HashOf::<u32>::from(&0);
        assert_ne!(x.to_inner(), 0);
    }
}
