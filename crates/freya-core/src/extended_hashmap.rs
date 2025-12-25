use std::{
    collections::{
        HashMap,
        HashSet,
        hash_map::Entry,
    },
    hash::Hash,
};

pub trait ExtendedHashMap<K, V> {
    fn get_disjoint_entries<const N: usize>(
        &mut self,
        entries: [&K; N],
        default: impl FnMut(&K) -> V,
    ) -> Option<[&mut V; N]>;

    fn get_disjoint_two_entries(
        &mut self,
        left: &K,
        right: &K,
        left_default: impl FnMut(&K) -> V,
        right_default: impl FnMut(&V, &K) -> V,
    ) -> [Option<&mut V>; 2]
    where
        V: Clone;
}

impl<K: Eq + Hash + ToOwned<Owned = K>, V, S: std::hash::BuildHasher + std::default::Default>
    ExtendedHashMap<K, V> for HashMap<K, V, S>
{
    fn get_disjoint_entries<const N: usize>(
        &mut self,
        entries: [&K; N],
        mut default: impl FnMut(&K) -> V,
    ) -> Option<[&mut V; N]> {
        let keys = HashSet::<&K, S>::from_iter(entries);

        if keys.len() != N {
            return None;
        }

        Some(entries.map(|key| {
            let ptr: *mut V = match self.entry(key.to_owned()) {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(default(key)),
            };
            unsafe { &mut *ptr }
        }))
    }

    fn get_disjoint_two_entries(
        &mut self,
        left: &K,
        right: &K,
        mut left_default: impl FnMut(&K) -> V,
        mut right_default: impl FnMut(&V, &K) -> V,
    ) -> [Option<&mut V>; 2]
    where
        V: Clone,
    {
        let left_val = self
            .entry(left.to_owned())
            .or_insert_with(|| left_default(left))
            .clone();

        self.entry(right.to_owned())
            .or_insert_with(|| right_default(&left_val, right));

        self.get_disjoint_mut([left, right])
    }
}
