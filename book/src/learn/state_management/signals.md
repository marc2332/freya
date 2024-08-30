# Signals

Signals are a state management solution built-in into Dioxus. They are simple reactive value containers that simplify the mutation and reading of state, even across components.

They are usually created by using the `use_signal` hook.

Example:

```rs
fn app() -> Element {
    let mut count = use_signal(|| 0); 
    // The closure passed to `use_signal` will be called only 
    // the first time this component renders, 
    // it will return the initial value for the Signal. 
    // This closure is to prevent having to create the initial value 
    // every time the component runs again, as it is only needed the first time.

    let onclick = move |_| {
        count += 1; // Shorthand for count.write() += 1;
        // The moment the signal is mutated it will notify 
        // all the components that have a read subscription 
        // to this signal (in this case, only `app`) 
        // that there has been a change. 
        // When that happens they will renders again 
        // and thus producing the new UI.
    };
    
    rsx!(
        label {
            onclick,
            "{count}" 
            // Because the signal is being read here, 
            // everytime that it gets mutated, this component 
            // will rerender as it has a read subscription.
            // "{count}" is the same as using "{count.read()}".
        }
    )
}
```