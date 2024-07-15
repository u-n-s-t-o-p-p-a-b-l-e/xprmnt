`Mutex` creates a counter that can be safely incremented by multiple threads using a Mutex.

Arc: Stands for "Atomic Reference Counted". It allows multiple ownership of the same data.

It's used to share ownership of the Mutex between multiple threads.

Mutex: Provides mutual exclusion, ensuring that only one thread can access the data inside the Mutex at a time.

`thread::spawn`: Spawns a new thread to execute the given closure.


```
rustc mutex.rs -o mutex && ./mutex
```

`Multiple Counters` can be incremented by multiple threads using Mutex.

```
rustc multiple_counters.rs -o multiple_counters && ./multiple_counters
```

`Shared Vector` use Mutex to safely share a vector between multiple threads.

```
rustc shared_vector.rs -o shared_vector && ./shared_vector

```
