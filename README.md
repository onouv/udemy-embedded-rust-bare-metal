# Exercises on STM32F303

This is a flollow-along in the Udemy course "Mastering Embedded Rust: Bare Metal ...", but in my own interpretation. This implementation stresses type state programming to create GPIO with zero-cost-abstractions. It is a personal playground to better understand the concepts laid down in the [Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html). 


There is a typesafe, threadsafe `static mut` constant [MCU`](src/board/mcu.rs), which is implemented as a singleton according to the concepts explained in the [Embedded Rust Book](https://docs.rust-embedded.org/book/peripherals/singletons.html).

This `MCU` type carefully manages an array of 96 `Option(())` marker flags, one for each GPIOx pins of the STM32F303 MCU. Clients of this code can obtain a singleton of a `Port` instance, which cannot be used for anything else but creating one of several readily configured input or output ports. This is achieved by mapping the identifiers of (GpioId, Pin) to an index in the array and calling the `take()` method of the `Option`, which sets the array entry for that port from `Some(())` to `None`. Trying to regain this particular port again results in returning an `Err()`.

Yes, this is incredibly rudimentary and error-prone, and an associative data structure would be much less troublesome. But we don't want to use a heap. One of the types from the `heapless` crate comes to mind. However, the memory footprint of this would be roughly **ten times** than that of the naked array.

Some of the port variants are:

| I/O Mode | Pin Mode  | Output Mode |
|----------|-----------|-------------|
| Input    | Floating  | -- |
|          | Pull Up   | -- |
|          | Pull Down | -- |
| Output   | Pull Up   | PushPull |
|          |           | OpenDrain |
|          | Pull Down | PushPull |
|          |           | OpenDrain |


For these distinctions, a set of generic implementations, marker structs and marker traits are used, which constitute zero-cost abstractions in the runtime. The idea is that it is impossible to create other combinations than shown above or to use non-sensical methods on the variants (such as setting a pin) on an input or reading an output pin.

This concept is explained in the [Embedded Rust Book](https://docs.rust-embedded.org/book/static-guarantees/design-contracts.html)

When creating these variants, the appropriate bits are set in the GPIOx port control registers. For this, the appropriate set of `Register`s are provided to ports. 

The next layer is representing resources on the board such as LED. These acquire only the associated ports as the application demands. The resulting memory footprint is minimal, since the entire type system disappears from the binary, it is only there at compile time to prevent you from shooting yourself in the foot.

This is work in progress and an excercise, so there are gaps and incomplete parts. 