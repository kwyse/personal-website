title: Building a personal website in Rust
author: Krishan Wyse
date: 13 July, 2016
path: building-personal-website-in-rust
summary: The chronicle of the beginnings of this web site

Inaugural blog post time! Here I will be covering how to create a basic templated website using Rust and HTML5/CSS3. I wish to state up front that I'm pretty new to web development and learnt all of this from other resources on the internet. It's my hope that condensing what I learnt into this post will help save you some time if you wish to develop your own website. I also hope that, by demonstrating how easy Rust and modern CSS makes it, this may serve as some inspiration!

We'll cover the basics of Rust as a lot of people I know haven't tried it out yet. If you're familiar with Rust and project setup, you can skip the following section and perhaps skim read the one after. For those new to the language, I've tried to provide links to the (top-quality) docs wherever possible. With that, let's get started!

# Project setup

On Unix, I'd definitely recommend [rustup](https://www.rustup.rs/) for installing Rust. You need only execute:

```shell
curl https://sh.rustup.rs -sSf | sh
```

in your shell and the installer will take you through the process. I'd recommend installing `nightly` as your default toolchain as some nice extra features like compiler plugins are currently restricted to it. 

If you're on Windows, you'll need to download an appropriate installer from the [downloads](https://www.rust-lang.org/en-US/downloads.html) page.

With Rust installed, you'll have access to some new tools in your shell. The most important of these is `cargo` and the only one we'll use in this post. Cargo is Rust's package manager as well as a wrapper around `rustc`, the compiler. You can do most project management tasks directly from Cargo (near enough all with additional tools like [cargo edit](https://github.com/killercup/cargo-edit), also recommended). If you'd like to know more about Cargo, the [Cargo docs](http://doc.crates.io/guide.html) have you covered.

To create our project, run `cargo new --bin demo-website`. The `--bin` flag tells Cargo we intend for this project to produce an executable binary, as opposed to a library that others will use. It will create a `src/main.rs` file that acts as the entry point. Running `cargo build` inside the root directory of the project (this is always the folder containing the `Cargo.toml` file) will create an executable in `target/debug`. Running `cargo run` should print out the ever joyous `Hello, world!` Executing `cargo run` will automatically do a `cargo build` before running if you've changed the source code since the last build.

Instead of just printing to stdout, how about starting up a server and printing to a web page? It's surprisingly little effort. Read on!

# Servers of Iron

We're now going to deep dive into some Rust code. I'll try to explain the thought process at a high level, but if you're new to the language, you may want to keep the [Rust book](https://doc.rust-lang.org/book/) within reach. We're going to use a library called [Iron](https://crates.io/crates/iron). While web development in Rust is [not extensively mature](http://www.arewewebyet.org/), it's certainly not in its infancy. Iron is one of the older and more established frameworks. Before we write the server, let's make one change to that `Cargo.toml` file. Add `iron` as a dependency, along with one of its utility libraries `router`, such that the dependency section looks like:

```toml
[dependencies]
iron = "0.3"
router = "0.1.1"
```

The `Cargo.toml` file acts as the specification for your project. It contains metadata about the project like authors and current version, pulls any dependencies needed, and can even run custom build scripts. [crates.io](https://crates.io/) acts as a central repository for all Rust projects (called crates) and the information on their is sourced from each project's `Cargo.toml` in turn. We're specifying versions to negate future regressions and ensure the example below runs as intended. In fact, at the time of writing, the latest version of Iron is `0.4`, but `router` [has not been updated](https://github.com/iron/router/issues/117) to use `0.4` and will fail to compile with it.

Before we demonstrate how to use Iron, let's go over the basic structure of a standalone Rust file, as it varies from more conventional languages. We first declare any external crates that are needed for our application. This informs the compiler to compile and link against these. This must be done in addition to declaring them in the `Cargo.toml` file, as that file is for dependency management only, not compiling and linking. We then declare what symbols from these crates we want to bring into scope and use freely. This section can be ommitted and you can instead fully qualify and usages of symbols you use from external crates. Finally, we have our `main` function, which servers as the entry as in most other languages.

Edit `src/main.rs` to the following:

```rust
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    // Define a function that takes one Request parameter and returns an
    // IronResult. The IronResult may be either an Response or an IronError. We
    // do not process the Request argument so signal to the compiler its
    // non-usage with an underscore. We return a 200 OK response with content
    // "Hello, world!".
    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }

    // Execute handler() when we recieve a GET request to the root URL
    let mut routes = Router::new();
    routes.get("/", handler);

    // Initialize the router and discard any errors via the unwrap() call
    let url = "localhost:3000";
    println!("Starting server at {}", url);
    Iron::new(routes).http(url).unwrap();
}
```

If you execute `cargo run`, you'll see Cargo download the necessary dependencies, compile them along with out application, and run it. If you navigate to the URL specified in a web browser, you should see the output. This is all that's required to get a server up and running and demonstrates some of the fundementals of Rust, so let's go through it line by line.

The first two lines state we want to link to two external crates: `iron` and `router`. The next three lines bring specific structs and functions declared in those crates into scope so that we can call them. The `use` call functions similarly to Java's `import` statement, where you can glob import symbols at that level of the hierarchy with an asterisk. The [Iron docs](http://ironframework.io/doc/iron/prelude/index.html) state what's included in the `prelude` module, but you can be sure it contains symbols you will need in most Iron apps. By convention, some Rust libraries include a `prelude` module just for this convinience purpose.

In the `main` function, we first define a function that will handle a request to the server. Rust syntax can be a big jarring at first but very expressive once you are used to it. A key thing to note is that everything is an expression, and if you leave the semicolon off of the last expression of a function, that expression is the *return value*. Our function must return an `IronResult<Response>`, which is a [type alias](https://doc.rust-lang.org/book/type-aliases.html) for `Result<Response, IronError>`. `Result` is one of the two fundamental error handling constructs in Rust and composed of two values: `Ok` or `Err`, signalling expected and error values respectively. In the body of the function, we wrap our actual response in `Ok` to singal everything is as expected. The [error handling](https://doc.rust-lang.org/book/error-handling.html) chapter in the Rust book is a marvelous read and definitely recommended. It explains the `Result` type and its friend, `Option`, in great detail.

The contents of the `Ok` are an implementation detail of what [Iron expects as a response](http://ironframework.io/doc/iron/response/struct.Response.html). Here, we're returning a [tuple](https://doc.rust-lang.org/std/primitive.tuple.html), which is a sequence in Rust that can contain objects of differing types. Iron can accept a response structured as a 2-length tuple containing an HTTP status and a body as the two elements.

The next two lines declare a `Router` object, implemented by the `router` crate and independent of Iron. We then register the function/closure to be called when we recieve a `GET` request to the root URL of the application. `let` is used for variable bindings in Rust, whilst `mut` is used to designate mutability in the variable as by default all variables are immutable. The `get` function mutates the `router` variable, so you would get a compiler error if you ommmitted `mut`. Rust is strongly typed, but supports type inferrence, which is why we didn't need to explicity state the type of the variable. The inferrence algorithm is not perfect though, and may get confused with more advanced constructs. You can explicitly state the type of a variable via this syntax, which is equivelent to what we have already:

```rust
let mut routes: Router = Router::new();
```

The final block of code sets the host and port number we want to run on, prints this out for logging purposes, and initalizes and starts the Iron server. [Macros](https://doc.rust-lang.org/book/macros.html), the methods by which `println!` is implemented, are too large a topic to discuss here, though one of Rust's greatest assets. You can tell it's a macro by the trailing exclamation mark, though functions just like a regular funciton. Macros play a key role in keeping Rust code concise and expressive. A well-designed macro can be treated as a black box, which is the case here for `println!`. It accepts a format string followed by a variable number of arguments corresponding to the number of format specifiers (designated by `{}` in Rust). Rust does not support a variable number of arguments in functions, which is why printing to standard output is implemented as a macro.

The final thing to note is the `unwrap()` call at the very end. Again, the [error handling](https://doc.rust-lang.org/book/error-handling.html) chapter in the Rust book covers this topic nicely. It goes into the specifics of `unwrap()` and why there are better alternatives to it. `unwrap()` will cause an application to panic (gracefully unwind and/or abort) if you unwrap an error value, so it should be avoided in production code, though it is useful for demonstrating other concepts when error handling isn't as much of a concern.

# Templating with Handlebars

Writing a hello world app never gets old, but we can do better. The [`handlebars-iron`](https://crates.io/crates/handlebars-iron) crate adds support for the [Handlebars](http://handlebarsjs.com/) templating language directly to Iron. Templating languages allow for dynamic content to be loaded into an otherwise static HTML page. The post you're reading now differs from others based on the URL entered, but the header and footer remain the same across blog posts. This templating promotes clean project structure and keeps things [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself). Add the crate as a dependency in `Cargo.toml`, declare it for linkage at the top of `src/main.rs` and update the `use` statements for the structs we will use. Update the `main` function to this:

In the `main` function, we first define a *closure* (called a lambda or anonymous function in other languages). Closures themselves are [too big of a topic](https://doc.rust-lang.org/book/closures.html) to cover in depth here, but you can think of them as inline functions. In Rust, functions and closures even have the same signature. Parameters of the function are declared between the two vertical pipes and, just as with all scopes in Rust, the final expression is used as the return value if it's not terminated with a semicolon. The contents of the `Ok` are an implementation detail of what [Iron expects as a response](http://ironframework.io/doc/iron/response/struct.Response.html). Here, we're returning a [tuple](https://doc.rust-lang.org/std/primitive.tuple.html), which is a sequence in Rust that can contain objects of differing types. Iron can accept a response structured as a 2-length tuple containing an HTTP status and a body as the two elements.
