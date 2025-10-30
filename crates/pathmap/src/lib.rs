use core::fmt;

pub struct PathMapEntry<V> {
    value: Option<V>,
    items: Vec<PathMapEntry<V>>,
}

impl<V: fmt::Debug> fmt::Debug for PathMapEntry<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathMapEntry")
            .field("value", &self.value)
            .field("items", &self.items)
            .finish()
    }
}

impl<V> PathMapEntry<V> {
    pub fn insert(&mut self, path: &[u32], value: V) {
        if path.is_empty() {
            self.value = Some(value);
        } else {
            if self.items.get(path[0] as usize).is_none() {
                self.items.resize_with(path[0] as usize + 1, || Self {
                    value: None,
                    items: Vec::new(),
                });
            } else if path.len() == 1 {
                self.items.insert(
                    path[0] as usize,
                    Self {
                        value: None,
                        items: Vec::new(),
                    },
                );
            }
            self.items[path[0] as usize].insert(&path[1..], value)
        }
    }

    pub fn insert_entry(&mut self, path: &[u32], entry: PathMapEntry<V>) {
        if path.is_empty() {
            *self = entry;
        } else {
            if self.items.get(path[0] as usize).is_none() {
                self.items.resize_with(path[0] as usize + 1, || Self {
                    value: None,
                    items: Vec::new(),
                });
            } else if path.len() == 1 {
                self.items.insert(
                    path[0] as usize,
                    Self {
                        value: None,
                        items: Vec::new(),
                    },
                );
            }
            self.items[path[0] as usize].insert_entry(&path[1..], entry)
        }
    }

    pub fn remove(&mut self, path: &[u32]) -> Option<PathMapEntry<V>> {
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
}

pub struct PathMap<V> {
    entry: Option<PathMapEntry<V>>,
}

impl<V> Default for PathMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: fmt::Debug> fmt::Debug for PathMap<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathMap")
            .field("entry", &self.entry)
            .finish()
    }
}

impl<V> PathMap<V> {
    pub fn new() -> Self {
        Self { entry: None }
    }

    pub fn insert(&mut self, path: &[u32], value: V) {
        if let Some(entry) = &mut self.entry {
            entry.insert(path, value);
        } else {
            let mut entry = PathMapEntry {
                value: None,
                items: Vec::new(),
            };
            entry.insert(path, value);
            self.entry = Some(entry);
        }
    }

    pub fn insert_entry(&mut self, path: &[u32], entry: PathMapEntry<V>) {
        if let Some(root_entry) = &mut self.entry {
            root_entry.insert_entry(path, entry);
        } else {
            let mut root_entry = PathMapEntry {
                value: None,
                items: Vec::new(),
            };
            root_entry.insert_entry(path, entry);
            self.entry = Some(root_entry);
        }
    }

    pub fn remove(&mut self, path: &[u32]) -> Option<PathMapEntry<V>> {
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

    pub fn traverse_1_level(&self, target: &[u32], mut traverser: impl FnMut(&[u32], &V)) {
        if let Some(entry) = &self.entry {
            entry.traverse_1_level(target, vec![], &mut traverser);
        }
    }
}
