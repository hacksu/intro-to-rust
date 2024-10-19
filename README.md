# intro-to-rust

Let's learn some cool thing about Rust!

## Installing rustup

First, we are going to need to install some tools (specifically `rustup`) so we can get the latest version of Rust, as well as build our project.
Here are the instructions on [how to install rustup](https://www.rust-lang.org/tools/install) for your system.

**NOTE:** For Windows users, you need [MSVC](https://visualstudio.microsoft.com/downloads/) (you should've installed it in CS1) to build a Rust project.

## Updating rustup

Once we have `rustup` installed. We can run `rustup up` (funny) and update Rust on our system. If you have previously installed Rust, this will probably update it for you. If you just installed Rust, this will just tell you that you're up to date!

## Digging into Rust

Alright, the Rust compiler is installed, let's write some code. 

### Hello world
All Rust source files end in the `.rs` extension, so we will create a `hello-world.rs` file and then write some code in it.

In `hello-world.rs`:
```Rust
fn main() {
    println!("Hello, world!");
}
```

First off, let's get this out of the way. Why is there an `!` in the `println!` function? In Rust, we refer to this as a *macro*. When we write `println!("Hello, world!")` and then compile our code, the compiler takes this `println!` and generates code to "replace" it. There are lots of macros in Rust and you can even make your own. `println!` is probably the most common one you will use.

Cool. Now that we understand macros, let's build this program. To build it, we just run `rustc hello-world.rs` from the command line. Then it will generate a program that we can run with `./hello-world.exe` or `./hello-world` depending on your operating system.

When you run it, you should see this:
```
Hello, world!
```

Wonderful! We're now super awesome epic Rust programmers!

### Interesting language features

Now that we've written a "Hello, world!" program, let's dig in a little more to some of the semantics of Rust.

First, let's look at variable definitions. Let's define some variables:
```Rust
let x = 0;
x = 1; // ERROR! 
```

Error? We can't re-assign variables by default in Rust, if you want to make a variable reassignable, you have to mark it was `mut` or mutable:
```Rust
let mut x = 0;
x = 1; // yay 
```

You can redefine a non-mutable variable, that looks something like this:
```Rust
let x = 0;
let x = 1;
```

Next, we will look at loops. Take this C++ for-loop:
```C++
for (int i = 0; i < 10; i++) {
    std::cout << i << std::endl;
}
```

We would write the same loop in Rust like:
```Rust
for i in 0..10 {
    println!("{i}");
}
```

Rust has a `Range` datatype, which is iterable (you can use a for-loop with it) and can be created with the `from..to` syntax. 

You'll also notice that there aren't parenthesis around the statement in the for-loop. This is actually a very common thing in Rust:
```Rust
let x = 0;
if x == 1 {
    println!("X is one!")
} else if x == 2 {
    println!("X is two!")
} else {
    println!("X is something else!")
}

while x != 0 {
    // ...
}
```

We also have the `match` keyword, which can be used like a switch statement:
```Rust
let x = 5;
match x {
    1 => println!("X is one!"),
    2..5 => println!("X is 2 to 5!"),
    _ => println!("X is some other value!"),
}
```
or it can be used to evaluate something in line:
```Rust
let boolean = true;
let output = match boolean {
    false => 0,
    true => 1,
}
// output will be true
```

