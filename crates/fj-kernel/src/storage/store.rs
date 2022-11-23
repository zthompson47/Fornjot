//! Append-only object storage
//!
//! So, why a custom data structure? Well, for two reasons:
//!
//! 1. No limitations on performance.
//! 2. Best possible convenience.
//!
//! Please note that I'm deliberately saying "no limitations" on performance. So
//! far, performance has not been a priority, so this might not be that fast.
//! But by having a custom data structure, we should be able to make performance
//! as good as we need it, within the limits of the practical.
//!
//! The second point, best possible convenience, is already realized.
//! [`Handle`]s can be owned, cloned, and dereference to the object they are
//! referencing. This is made possible by the append-only nature of our object
//! storage, and our immutable objects.
//!
//! There are other append-only data structures on `crates.io`. Some of them
//! look interesting, but none of them quite fit our needs and possibilities, so
//! a custom development seemed justified.
//!
//! But in any case, this was fun to write, and not that much work.

use std::{marker::PhantomData, sync::Arc};

use parking_lot::RwLock;

use super::{
    blocks::{Blocks, Index},
    Handle,
};

/// Append-only object storage
#[derive(Debug)]
pub struct Store<T> {
    inner: StoreInner<T>,
}

impl<T> Store<T> {
    /// Construct a new instance of `Store`
    ///
    /// Equivalent to calling [`Store::with_block_size`] with a default block
    /// size.
    pub fn new() -> Self {
        Self::with_block_size(16384)
    }

    /// Construct a new instance of `Store` using the provided block size
    pub fn with_block_size(block_size: usize) -> Self {
        let inner = Arc::new(RwLock::new(StoreInnerInner {
            blocks: Blocks::new(block_size),
        }));

        Self { inner }
    }

    /// Insert an object into the store
    pub fn insert(&self, object: T) -> Handle<T> {
        let mut inner = self.inner.write();

        let (index, ptr) = inner.blocks.reserve();
        inner.blocks.insert(index, object);

        Handle {
            store: self.inner.clone(),
            ptr,
        }
    }

    /// Iterate over all objects in this store
    pub fn iter(&self) -> Iter<T> {
        Iter {
            store: self.inner.clone(),
            next_index: Index::zero(),
            _a: PhantomData,
        }
    }
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> IntoIterator for &'a Store<T> {
    type Item = Handle<T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over objects in a [`Store`]
pub struct Iter<'a, T> {
    store: StoreInner<T>,
    next_index: Index,
    _a: PhantomData<&'a ()>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.store.read();

        loop {
            let ptr = inner.blocks.get_and_inc(&mut self.next_index)?;

            if ptr.is_none() {
                // This is a reserved slot.
                continue;
            }

            return Some(Handle {
                store: self.store.clone(),
                ptr,
            });
        }
    }
}

pub type StoreInner<T> = Arc<RwLock<StoreInnerInner<T>>>;

#[derive(Debug)]
pub struct StoreInnerInner<T> {
    blocks: Blocks<T>,
}

#[cfg(test)]
mod tests {
    use super::Store;

    #[test]
    fn insert_and_handle() {
        let store = Store::with_block_size(1);

        let object = 0;
        let handle = store.insert(object);

        assert_eq!(*handle, object);
    }

    #[test]
    fn insert_and_iter() {
        let store = Store::with_block_size(1);

        let a = store.insert(0);
        let b = store.insert(1);

        let objects = store.iter().collect::<Vec<_>>();
        assert_eq!(objects, [a, b])
    }
}
