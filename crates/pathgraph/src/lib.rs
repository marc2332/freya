use core::fmt;

pub struct PathGraphEntry<V> {
    value: Option<V>,
    items: Vec<PathGraphEntry<V>>,
}

impl<V: fmt::Debug> fmt::Debug for PathGraphEntry<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathGraphEntry")
            .field("value", &self.value)
            .field("items", &self.items)
            .finish()
    }
}

impl<V> PathGraphEntry<V> {
    pub fn insert(&mut self, path: &[u32], value: V) {
        if path.is_empty() {
            self.value = Some(value);
        } else {
            match self.items.get(path[0] as usize) {
                None => {
                    self.items.resize_with(path[0] as usize + 1, || Self {
                        value: None,
                        items: Vec::new(),
                    });
                }
                Some(existing) if path.len() == 1 && existing.value.is_some() => {
                    // Real sibling already here: shift it right and insert.
                    self.items.insert(
                        path[0] as usize,
                        Self {
                            value: None,
                            items: Vec::new(),
                        },
                    );
                }
                // Empty placeholder (`value: None`) or intermediate entry:
                // fall through and let the recursive call fill it in place.
                _ => {}
            }
            self.items[path[0] as usize].insert(&path[1..], value)
        }
    }

    pub fn insert_entry(&mut self, path: &[u32], entry: PathGraphEntry<V>) {
        if path.is_empty() {
            *self = entry;
        } else {
            match self.items.get(path[0] as usize) {
                None => {
                    self.items.resize_with(path[0] as usize + 1, || Self {
                        value: None,
                        items: Vec::new(),
                    });
                }
                Some(existing) if path.len() == 1 && existing.value.is_some() => {
                    self.items.insert(
                        path[0] as usize,
                        Self {
                            value: None,
                            items: Vec::new(),
                        },
                    );
                }
                _ => {}
            }
            self.items[path[0] as usize].insert_entry(&path[1..], entry)
        }
    }

    pub fn remove(&mut self, path: &[u32]) -> Option<PathGraphEntry<V>> {
        if path.is_empty() {
            unreachable!()
        } else if path.len() == 1 {
            if path[0] as usize >= self.items.len() {
                self.items.pop()
            } else {
                Some(self.items.remove(path[0] as usize))
            }
        } else if let Some(item) = self.items.get_mut(path[0] as usize) {
            item.remove(&path[1..])
        } else {
            None
        }
    }

    pub fn find_path(
        &self,
        path: Vec<u32>,
        finder: &impl Fn(Option<&V>) -> bool,
    ) -> Option<Vec<u32>> {
        if finder(self.value.as_ref()) {
            return Some(path);
        }
        for (i, item) in self.items.iter().enumerate() {
            let mut path = path.clone();
            path.push(i as u32);
            if let Some(path) = item.find_path(path, finder) {
                return Some(path);
            }
        }
        None
    }

    pub fn find(&self, path: Vec<u32>, finder: &impl Fn(Option<&V>) -> bool) -> Option<&V> {
        if finder(self.value.as_ref()) {
            return self.value.as_ref();
        }
        for (i, item) in self.items.iter().enumerate() {
            let mut path = path.clone();
            path.push(i as u32);
            if let Some(res) = item.find(path, finder) {
                return Some(res);
            }
        }
        None
    }

    pub fn reduce<A>(
        &self,
        acc: &mut A,
        path: Vec<u32>,
        reducer: &impl Fn(Option<&V>, &[u32], &mut A) -> bool,
    ) -> bool {
        if reducer(self.value.as_ref(), &path, acc) {
            return true;
        }
        for (i, item) in self.items.iter().enumerate() {
            let mut path = path.clone();
            path.push(i as u32);
            if item.reduce(acc, path, reducer) {
                return true;
            }
        }
        false
    }

    pub fn find_child_path(
        &self,
        path: Vec<u32>,
        target: &[u32],
        finder: &impl Fn(Option<&V>) -> bool,
    ) -> Option<Vec<u32>> {
        if !path.is_empty() && &path[0..path.len() - 1] == target && finder(self.value.as_ref()) {
            return Some(path);
        }
        for (i, item) in self.items.iter().enumerate() {
            let mut path = path.clone();
            path.push(i as u32);
            if let Some(path) = item.find_child_path(path, target, finder) {
                return Some(path);
            }
        }
        None
    }

    pub fn get(&self, path: &[u32]) -> Option<&V> {
        if path.is_empty() {
            self.value.as_ref()
        } else if let Some(item) = self.items.get(path[0] as usize) {
            item.get(&path[1..])
        } else {
            None
        }
    }

    pub fn len(&self, path: &[u32]) -> Option<usize> {
        if path.is_empty() {
            Some(self.items.len())
        } else if let Some(item) = self.items.get(path[0] as usize) {
            item.len(&path[1..])
        } else {
            None
        }
    }

    pub fn size(&self, size: &mut usize) {
        if self.value.is_some() {
            *size += 1;
        }

        for item in &self.items {
            item.size(size);
        }
    }

    pub fn traverse(
        &self,
        target: &[u32],
        mut path: Vec<u32>,
        traverser: &mut impl FnMut(&[u32], &V),
    ) {
        if path.starts_with(target)
            && let Some(value) = self.value.as_ref()
        {
            traverser(&path, value);
        }

        for (i, item) in self.items.iter().enumerate() {
            path.push(i as u32);
            if target.starts_with(&path) || path.starts_with(target) {
                item.traverse(target, path.clone(), traverser);
            }
            path.pop();
        }
    }

    pub fn retain(
        &mut self,
        target: &[u32],
        parent_is_retained: bool,
        mut path: Vec<u32>,
        retainer: &mut impl FnMut(&[u32], &V) -> bool,
        traverser: &mut impl FnMut(&[u32], &V),
    ) -> bool {
        let mut retain = parent_is_retained;
        if path.starts_with(target)
            && let Some(value) = self.value.as_ref()
        {
            if parent_is_retained {
                retain = retainer(&path, value);
            }

            if !retain {
                traverser(&path, value);
            }
        }

        let mut i = 0;
        self.items.retain_mut(|item| {
            let mut retain = retain;
            path.push(i as u32);
            if target.starts_with(&path) || path.starts_with(target) {
                retain = item.retain(target, retain, path.clone(), retainer, traverser);
            }
            path.pop();
            i += 1;
            retain
        });
        retain
    }

    pub fn traverse_1_level(
        &self,
        target: &[u32],
        mut path: Vec<u32>,
        traverser: &mut impl FnMut(&[u32], &V),
    ) {
        if path == target {
            for (i, item) in self.items.iter().enumerate() {
                if let Some(value) = item.value.as_ref() {
                    path.push(i as u32);
                    traverser(&path, value);
                    path.pop();
                }
            }
        } else {
            for (i, item) in self.items.iter().enumerate() {
                path.push(i as u32);
                item.traverse_1_level(target, path.clone(), traverser);
                path.pop();
            }
        }
    }

    pub fn value(self) -> Option<V> {
        self.value
    }
}

pub struct PathGraph<V> {
    entry: Option<PathGraphEntry<V>>,
}

impl<V> Default for PathGraph<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: fmt::Debug> fmt::Debug for PathGraph<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathGraph")
            .field("entry", &self.entry)
            .finish()
    }
}

impl<V> PathGraph<V> {
    pub fn new() -> Self {
        Self { entry: None }
    }

    pub fn insert(&mut self, path: &[u32], value: V) {
        if let Some(entry) = &mut self.entry {
            entry.insert(path, value);
        } else {
            let mut entry = PathGraphEntry {
                value: None,
                items: Vec::new(),
            };
            entry.insert(path, value);
            self.entry = Some(entry);
        }
    }

    pub fn insert_entry(&mut self, path: &[u32], entry: PathGraphEntry<V>) {
        if let Some(root_entry) = &mut self.entry {
            root_entry.insert_entry(path, entry);
        } else {
            let mut root_entry = PathGraphEntry {
                value: None,
                items: Vec::new(),
            };
            root_entry.insert_entry(path, entry);
            self.entry = Some(root_entry);
        }
    }

    pub fn remove(&mut self, path: &[u32]) -> Option<PathGraphEntry<V>> {
        if let Some(entry) = &mut self.entry {
            entry.remove(path)
        } else {
            None
        }
    }

    pub fn find_path(&self, finder: impl Fn(Option<&V>) -> bool) -> Option<Vec<u32>> {
        if let Some(entry) = &self.entry {
            entry.find_path(vec![], &finder)
        } else {
            None
        }
    }

    pub fn find(&self, finder: impl Fn(Option<&V>) -> bool) -> Option<&V> {
        if let Some(entry) = &self.entry {
            entry.find(vec![], &finder)
        } else {
            None
        }
    }

    pub fn reduce<A>(
        &self,
        acc: &mut A,
        reducer: impl Fn(Option<&V>, &[u32], &mut A) -> bool,
    ) -> bool {
        if let Some(entry) = &self.entry {
            entry.reduce(acc, vec![], &reducer)
        } else {
            false
        }
    }

    pub fn find_child_path(
        &self,
        target: &[u32],
        finder: impl Fn(Option<&V>) -> bool,
    ) -> Option<Vec<u32>> {
        if let Some(entry) = &self.entry {
            entry.find_child_path(vec![], target, &finder)
        } else {
            None
        }
    }

    pub fn len(&self, path: &[u32]) -> Option<usize> {
        if let Some(entry) = &self.entry {
            entry.len(path)
        } else {
            None
        }
    }

    pub fn get(&self, path: &[u32]) -> Option<&V> {
        if let Some(entry) = &self.entry {
            entry.get(path)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        if let Some(entry) = &self.entry {
            entry.size(&mut size);
        }
        size
    }

    pub fn traverse(&self, target: &[u32], mut traverser: impl FnMut(&[u32], &V)) {
        if let Some(entry) = &self.entry {
            entry.traverse(target, vec![], &mut traverser);
        }
    }

    pub fn retain(
        &mut self,
        target: &[u32],
        mut retainer: impl FnMut(&[u32], &V) -> bool,
        mut traverser: impl FnMut(&[u32], &V),
    ) {
        if let Some(entry) = &mut self.entry
            && !entry.retain(target, true, vec![], &mut retainer, &mut traverser)
        {
            let _ = self.entry.take();
        }
    }

    pub fn traverse_1_level(&self, target: &[u32], mut traverser: impl FnMut(&[u32], &V)) {
        if let Some(entry) = &self.entry {
            entry.traverse_1_level(target, vec![], &mut traverser);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// When a `remove` + `insert_entry` pair leaves a `None`-valued placeholder
    /// inside an `items` vec, a subsequent `insert` targeting that same slot
    /// must fill the placeholder in place instead of shifting it right.
    /// Otherwise later siblings drift to a higher index than their logical
    /// position and `get` at the intended path returns `None`.
    #[test]
    fn insert_fills_placeholder_left_by_move_sequence() {
        let mut graph = PathGraph::<u32>::new();
        graph.insert(&[], 0);
        graph.insert(&[0], 10);
        graph.insert(&[0, 0], 100);
        graph.insert(&[0, 1], 200);

        // Simulate the move sequence (1,0), (0,2) that `apply_diff` performs:
        let a = graph.remove(&[0, 1]).unwrap();
        graph.insert_entry(&[0, 0], a);
        let b = graph.remove(&[0, 1]).unwrap();
        graph.insert_entry(&[0, 2], b);

        // Now [0,1] should be the empty slot the deferred addition targets.
        assert_eq!(graph.get(&[0, 0]), Some(&200));
        assert_eq!(graph.get(&[0, 1]), None);
        assert_eq!(graph.get(&[0, 2]), Some(&100));

        // Fill the placeholder with a deferred addition.
        graph.insert(&[0, 1], 999);

        // The deferred value lands at its intended position and the existing
        // siblings stay put instead of drifting one index to the right.
        assert_eq!(graph.get(&[0, 0]), Some(&200));
        assert_eq!(graph.get(&[0, 1]), Some(&999));
        assert_eq!(graph.get(&[0, 2]), Some(&100));
        assert_eq!(graph.get(&[0, 3]), None);
        assert_eq!(graph.len(&[0]), Some(3));
    }

    /// `insert` must still shift real siblings when inserting between
    /// occupied positions.
    #[test]
    fn insert_still_shifts_occupied_siblings() {
        let mut graph = PathGraph::<u32>::new();
        graph.insert(&[], 0);
        graph.insert(&[0], 10);
        graph.insert(&[0, 0], 100);
        graph.insert(&[0, 1], 200);

        graph.insert(&[0, 1], 150);

        assert_eq!(graph.get(&[0, 0]), Some(&100));
        assert_eq!(graph.get(&[0, 1]), Some(&150));
        assert_eq!(graph.get(&[0, 2]), Some(&200));
        assert_eq!(graph.len(&[0]), Some(3));
    }
}
