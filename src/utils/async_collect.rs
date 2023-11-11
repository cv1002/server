use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    future::Future,
    hash::Hash,
};

use async_trait::async_trait;

#[async_trait]
pub trait AsyncCollect<Ret, Fut: Future<Output = Ret>> {
    async fn async_collect<B: AsyncCollectable<Ret>>(self) -> B;
}

#[async_trait]
impl<Ret, Fut, Itr> AsyncCollect<Ret, Fut> for Itr
where
    Ret: Send,
    Fut: Send + Future<Output = Ret>,
    Itr: Send + Iterator<Item = Fut>,
{
    async fn async_collect<B: AsyncCollectable<Ret>>(self) -> B {
        let capacity = {
            let (low, high) = self.size_hint();
            match high {
                Some(high) => high - low,
                None => 0,
            }
        };
        let mut result = B::with_capacity(capacity);
        for fut in self {
            result.insert(fut.await);
        }
        result
    }
}

pub trait AsyncCollectable<T>: Send {
    fn with_capacity(capacity: usize) -> Self;
    fn insert(&mut self, value: T);
}

impl<T: Send> AsyncCollectable<T> for Vec<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn insert(&mut self, value: T) {
        self.push(value)
    }
}

impl<T: Send + Eq + Hash> AsyncCollectable<T> for HashSet<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn insert(&mut self, value: T) {
        self.insert(value);
    }
}

impl<K: Send + Eq + Hash, V: Send> AsyncCollectable<(K, V)> for HashMap<K, V> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn insert(&mut self, kv: (K, V)) {
        let (key, value) = kv;
        self.insert(key, value);
    }
}

impl<T: Send + Eq + Ord> AsyncCollectable<T> for BTreeSet<T> {
    fn with_capacity(_: usize) -> Self {
        Self::new()
    }

    fn insert(&mut self, value: T) {
        self.insert(value);
    }
}

impl<K: Send + Eq + Ord, V: Send> AsyncCollectable<(K, V)> for BTreeMap<K, V> {
    fn with_capacity(_: usize) -> Self {
        Self::new()
    }

    fn insert(&mut self, kv: (K, V)) {
        let (key, value) = kv;
        self.insert(key, value);
    }
}
