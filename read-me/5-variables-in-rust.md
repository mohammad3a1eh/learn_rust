[Home](https://github.com/kinite-gp/learn_rust)

### Variables in **Rust**

<hr>

#### Mutable Variables:

- To create a mutable variable, you use the `let mut` syntax. Mutable variables allow you to modify their values after
  the initial assignment. Here's an example:

```rust
let mut y = 5;
println!("The value of y is: {}", y);

// Modifying the value of y

y = 10;
println!("Now the value of y is: {}", y);
```

- Rust allows you to shadow variables, where you can redeclare a variable with the same name, effectively creating a new
  variable. Shadowing is often used for transformations or conversions without the need for a new variable name.

```rust
let x = 5;
let x = x + 1; // Shadows the previous x
let x = x * 2; // Shadows the previous x

println!("The final value of x is: {}", x);
```

#### Immutable Variables:

- To declare an immutable variable, you use the let keyword without the mut modifier. Once assigned, the value of an
  immutable variable cannot be changed. Here's an example:

```rust
let x = 5;
println!("The value of x is: {}", x);

// This line will result in an error because x is immutable
// x = 10;
```
