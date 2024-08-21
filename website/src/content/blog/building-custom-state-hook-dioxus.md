---
title: 'Building a custom state Hook for Dioxus'
date: 2024-08-18
description: 'Building a custom state management solution for Dioxus.'
author: 'marc2332'
layout: ../../layouts/BlogPostLayout.astro
slug: "building-custom-state-hook-dioxus"
---

As nobody has done this before, I think it would be interesting to people interested in Dioxus, to learn
how are hooks made.

### 👋 Preface 

**Dioxus** has built-in support for reactive state management in the form of hooks and **Signals**.
I am not gonna dive into what hooks or signals are, so if you don't know what they are you may check the official docs first.

You may be asking yourself why do we even need to build custom hooks if Dioxus already give us different tools for different use cases.
And it actually just depends on your needs, for the majority of the cases the official tools will work just fine. 

Now, there are certain cases where it's just not a good fit and you might see yourself wanting a certain reactive behavior or just a different API. 

For those who happen to be in this situation, as I was myself when building [Valin](https://github.com/marc2332/valin/), worry no more, I got you.

### 🤝 Introduction 

So, a bit of introduction. We will build a custom hook that let us have cross-component reactive `key-value` store. 
The idea is that components subscribe to specific keys, so every time anyone modifies the value of that specific key, the component will rerun. We will call this hook  `use_value`

This is how it will look like:

```rs
fn CoolComponent() {
    // Subscribe to any changes to the value identified by the key `counter`.
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
    // Just like above, subscribe to any changes to the value identified by the key `counter`.
    let same_count = use_value("counter");

    rsx!(
        label {
            "{same_count}"
        }
    )
}

```

### 🧵 The basics 

#### `use_hook`

`use_hook` is the foundational core hook in which all hooks are built on top of.
It let us store value that is tied to life of the **Scope** it was created in,
which means that when the **Component** is dropped, the  stored value will be dropped as well.

It takes a closure to initialize our value, this be called when the component runs for the first time, this way the value is only created when it truly needs to be created. It also returns the value it created or a cloned value of it if this isn't the first run anymore, is why it's value requires to be `Clone`.

But if the value is cloned it means that any change that we make to the cloned value will not be 
persisted for the next change? Correct, but this is why we are going to be using smart pointers.

Here there is an example, even though our component will run as many times as it needs it will always hold the same value 
it was created with, the `Default` of `T`. Because when the component runs, what gets is not the `T` but the `Rc`.
```rs
fn use_value<T: Default>() -> Rc<RefCell<T>> {
    use_hook(|| {
        Rc::new(RefCell::new(T::default()))
    })
}
```

### The not so basic
Alright we got a dummy hook that all it does is store a value, but how do we share this value with an assigned key?
We just need to build a registry where all subscribers self-register and self-unregister.


```rs
/// For every key we have in the registry 
/// there will be a `RegistryEntry` where the value and 
/// subscribers are stored.
struct RegistryEntry<T> {
    value: Signal<T>,
    subscribers: HashSet<ScopeId>
}

struct Registry<T> {
    map: HashMap<String, RegistryEntry<T>>
}

impl<T: Default> Registry<T> {
    /// Subscribe the given [ScopeId] in this registry to `key`.
    /// Will initialize the value to `T::default` if there wasn't one before.
    pub fn subscribe(&mut self, key: String, scope_id: ScopeId) -> {
        self.map.insert_or_get(key, || RegistryEntry {
            value: Signal::new_in_scope(T::default(), ScopeId::ROOT),
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

    /// Get the [Signal] of the value assigned to the given `key`.
    pub fn get(&self, key: &str) -> Signal<T> {
        self.map.get(key).copied().unwrap()
    }
}

/// Access the value assigned to the given key.
fn use_value<T>(key: &str) -> Signal<T> {
    use_hook(|| {
        // Access the registry and if it doesn't exist create it in the root scope
        let mut registry = consume_context::<Signal<Registry<T>>>().unwrap_or_else(|| {
            provide_root_context(Signal::new_in_scope(Registry {
                map: HashMap::default()
            }, ScopeId::ROOT))
        });
        
        // Subscribe this component
        registry.subscribe(key, current_scope_id().unwrap());

        // Return value of the given key
        registry.get(key)
    })
}
```

