---
title: 'Custom State management'
date: 2024-08-08
description: 'Building a custom state management solution for Dioxus.'
author: 'marc2332'
layout: ../../layouts/BlogPostLayout.astro
slug: "custom-state-management"
---

In this post I will teach you how hooks work and how to make make
a reactive state lib from scratch.

### Why?

You may ask yourself why do we even need third-party libraries for state management
and the truth is that you might not need them. Dioxus comes with a Signals
a very basic and simple to use state management solution that while
it works great, it might not scale for more complex apps.

### What?

So a bit of spoiler here, our library will consist of a hook that let use subscribe 
and write to a key-value store. We will call it `use_value`.

```rs
fn CoolComponent() {
    let mut count = use_value("counter");

    rsx!(
        Button {
            onclick: |_| {
                count += 1;
            }
        }
        label {
            "{count}"
        }
    )
}

fn app() {
    let same_count = use_value("counter");

    rsx!(
        label {
            "{same_count}"
        }
    )
}

```

### The basics

#### `use_hook`

All hooks in Dioxus are built on top of a low level core hook called `use_hook`.
This one let us store a value that tied to life of  the **Scope** it was created in,
which means that when the **Component** is dropped, our stored value will be dropped as well.

Every time the component where the store value is created is re-run it will give is access to the value we store, 
but for this there is a catch, the stored value must be `Clone` so we can get a hold of it
on every render of the component.

But if the value is cloned it means that any change that we make to the cloned value will not be 
persisted for the next change? Worry no more, we got smart pointers.

Here there is an example, even though our component will run as many times as it needs it will always hold the same value 
it was created with, the `Default` of `T`. Because when the component runs what is cloned is the `Rc` and not `T`.
```rs
fn use_value<T: Default>() -> Rc<RefCell<T>> {
    use_hook(|| {
        Rc::new(RefCell::new(T::default()))
    })
}
```

### The not so basic
Alright we got a dummy hook that all it does is store a value, but how do we share this value with an assigned key?
We just need to build a registry.



```rs
struct RegistryEntry<T> {
    value: Signal<T>,
    subscribers: HashSet<ScopeId>
}

struct Registry<T> {
    map: HashMap<String, RegistryEntry<T>>
}

impl<T: Default> Registry<T> {
    /// Subscribe the given [Scope] in this registry to `key`.
    /// Will initialize the value to `T::default` if there wasn't one before.
    pub fn subscribe(&mut self, key: String, scope_id: ScopeId) -> {
        self.map.insert_or_get(key, || RegistryEntry {
            value: Signal::new(T::default()),
            subscribers: HashSet::from([scope_id])
        })
        .subscribers.insert(scope_id)
    }

    /// Unsubscriber the given [ScopeId] from this registry `key`.
    pub fn unsubscribe(&mut self, key: &str, scope_id: ScopeId) {
        let registry_entry = self.map.get_mut(key).unwrap();
        registry_entry.subscribers.remove(scope_id);

        // Remove the registry entry if there are no more subscribers left
        if registry_entry.subscribers.is_empty() {
            self.map.remove(key);
        }
    }

    /// Get the [Signal] assigned to the given `key`.
    pub fn get(&self, key: &str) -> Signal<T> {
        self.map.get(key).copied().unwrap()
    }
}

fn use_value<T>(key: &str) -> Signal<T> {
    use_hook(|| {
        // Access the registry and if it doesn't exist create it in the root scope
        let mut registry = consume_context::<Signal<Registry<T>>>().unwrap_or_else(|| {
            provide_root_context(Signal::new_in_scope(Registry {
                map: HashMap::default()
            }, ScopeId::ROOT))
        });
        
        // Subscribe this component
        registry.subscribe(current_scope_id().unwrap());

        // Return the Signal assigned to the given key
        registry.get(key)
    })
}
```

