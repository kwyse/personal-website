title: Building a personal website in Rust
author: Krishan Wyse
date: 13 July, 2016
path: building-personal-website-in-rust
summary: The chronicle of the beginnings of this web site

Inaugural blog post time! Here I will be covering how to create a basic templated website using Rust and HTML5/CSS3. I wish to state up front that I'm pretty new to web development and learnt all of this from other resources on the internet. It's my hope that condensing what I learnt into this post will help save you some time if you wish to develop your own website. I also hope that, by demonstrating how easy Rust and modern CSS makes it, this may serve as some inspiration!

If you're familiar with Rust and project setup, you can skip to the following section. Otherwise, let's get started!

# Project setup

On Unix, I'd definitely recommend [rustup](https://www.rustup.rs/) for installing Rust. You need only execute:

```shell
curl https://sh.rustup.rs -sSf | sh
```

in your shell and the installer will take you through the process. I'd recommend installing `nightly` as your default toolchain as some nice extra features like compiler plugins are currently restricted to it. 

If you're on Windows, you'll need to download an appropriate installer from the [downloads](https://www.rust-lang.org/en-US/downloads.html) page.

With Rust installed, you'll have access to some new tools in your shell. The most important of these is `cargo` and the only one we'll use in this post. Cargo is Rust's package manager as well as a wrapper around `rustc`, the compiler. You can do most project management tasks directly from Cargo (near enough all with additional tools like [cargo edit](https://github.com/killercup/cargo-edit), also recommended). If you'd like to know more about Cargo, the [Cargo docs](http://doc.crates.io/guide.html) have you covered.

To create our project, run `cargo new --bin demo-website`. The `--bin` flag tells Cargo we intend for this project to produce an executable binary, as opposed to a library that others will use. It will have created a `src/main.rs` file that acts as the entry point. Running `cargo build` inside the root directory of the project (this is always the folder containing the `Cargo.toml` file) will create an executable in `target/debug`. Running `cargo run` should print out the ever joyous `Hello, world!` Executing `cargo run` will automatically do a `cargo build` before running if you've changed the source code since the last build.

Instead of just printing to stdout, how about starting up a server and printing to a web page? It's surprisingly little effort. Read on!

# Servers of Iron

We're now going to deep dive into some Rust code. I'll try to explain the thought process at a high level, but if you're new to the language, you may want to keep the [Rust book](https://doc.rust-lang.org/book/) nearby. We're going to use a library called [Iron](https://crates.io/crates/iron). While web development in Rust is [not extensively mature](http://www.arewewebyet.org/), it's certainly not in its infancy. Iron is one of the older and more established frameworks. Before we write the server, let's make one change to that `Cargo.toml` file. Add `iron` as a dependency, such that the dependency section looks like:

```toml
[dependencies]
iron = "*"
router = "*"
```

The `Cargo.toml` file acts as the specification for your project. It contains metadata about the project like authors and current version, pulls any dependencies needed, and can even run custom build scripts. [crates.io](https://crates.io/) acts as a central repository for all Rust projects (called crates) and the information on their is sourced from each project's `Cargo.toml` in turn.

Now let's demonstrate using Iron. Edit `src/main.rs` to the following:

```rust
extern crate iron; // We want to use the iron and router crate
extern crate router;

use iron::prelude::*; // Bring certain Iron and Router objects into scope
use iron::status;
use router::Router;

fn main() {
    // Define a closure that takes no arguments and returns an OK response
    let handler = || Ok(Response::with(
        (status::Ok, "Hello, world!")
    ));

    // ::new() methods are conventually used for object construction
    let mut router = Router::new();
    // Execute handler() when we recieve a GET request to the root URL
    router.get("/", handler);

    let url = "localhost:3000";
    // println! is a macro that prints its arguments to stdout
    println!("Starting server at {}", url);
    // unwrap() discards any errors that could have occurred on its callee
    Iron::new(router).http(url).unwrap();
}
```

If you execute `cargo run`, you'll see Cargo download the necessary dependencies, compile them, and run our application. If you navigate to the URL specified in a web browser, you should see the output. This is all that's required to a server up and running and demonstrates some of the fundementals of Rust, so let's go through it line by line.

The first two lines state we want to link to two external crates: `iron` and `router`. The following three bring specific structs and functions declared in those crates into scope so that we can call them. The `use` call functions similarly to Java's `import` statement, where you can grab all objects at that level of the hierarchy with an asterisk. The [Iron docs](http://ironframework.io/doc/iron/prelude/index.html) state what's included in the `prelude` module.

In the `main` function, we first define a *closure* (called a lambda or anonymous function in other languages). Closures themselves are [too big of a topic](https://doc.rust-lang.org/book/closures.html) to cover in depth here, but you can think of them as inline functions. In Rust, functions and closures even have the same signature. Parameters of the function are declared between the two vertical pipes and, just as with all scopes in Rust, the final expression is used as the return value if it's not terminated with a semicolon. In this case, it's the *closure* itself that is terminated with the semicolon, not the *closure contents*, so this closure still returns an `OK` response. The contents of the `Ok` are an implementation detail of what [Iron expects as a response](http://ironframework.io/doc/iron/response/struct.Response.html). Here, we're returning a [tuple](https://doc.rust-lang.org/std/primitive.tuple.html), which is a sequence in Rust that can contain objects of differing types. Iron can accept a response structured as a 2-length tuple containing an HTTP status and a body as the two elements.

The next two lines declare a `Router` object, implemented by the `router` crate and independent of Iron. We then register the function/closure to be called when we recieve a `GET` request to the root URL of the application.

The final block of code sets the host and port number we want to run on, prints this out for logging purposes, and initalizes and starts the Iron server. [Macros](https://doc.rust-lang.org/book/macros.html) are another topic too big to discuss here, though one of Rust's greatest assets. Macros play a key role in keeping Rust code concise and expressive. The final thing to note is the `unwrap()` call at the very end. The [error handling](https://doc.rust-lang.org/book/error-handling.html) chapter in the Rust book is a marvelous read and definitely recommended. It goes into the specifics of `unwrap()` and why there are better alternatives to it. `unwrap()` will cause an application to panic (gracefully unwind and/or abort) if you unwrap an error value, so it should be avoided in production code, though is useful for demonstrating other concepts when error handling isn't as much of a concern.

# Templating with Handlebars

Writing a hello world app never gets old, but we can do better. The [`handlebars-iron`](https://crates.io/crates/handlebars-iron) crate adds support for the [Handlebars](http://handlebarsjs.com/) templating language directly to Iron. Templating languages allow for dynamic content to be loaded into an otherwise static HTML page. The post you're reading now differs from others based on the URL entered, but the header and footer remain the same across blog posts. This templating promotes clean project structure and keeps things [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself). Add the crate as a dependency in `Cargo.toml`, declare it for linkage at the top of `src/main.rs` and update the `use` statements for the structs we will use. Update the `main` function to this:
