title: Building a personal website in Rust
author: Krishan Wyse
date: 18 July, 2016
path: building-personal-website-in-rust
summary: How this website came into existence and how to build your own using Rust, Handlebars and Flexbox

Inaugural blog post time! Here I will be covering how to create a basic templated website using Rust and HTML5/CSS3. I wish to state up front that I'm pretty new to web development and learnt all of this from other resources online. It's my hope that condensing what I learnt will help save you some time if you wish to develop your own website. I also hope that, by demonstrating how easy Rust and modern CSS makes it, this may serve as some inspiration!

We'll cover the basics of Rust as a lot of people I know haven't tried it out yet. If you're familiar with Rust and project setup, you can skip the following section and perhaps skim read the one after. For those new to the language, I've tried to provide links to the (top-quality) docs wherever possible. Let's get started!

# Project setup

On Unix, I'd definitely recommend [rustup](https://www.rustup.rs/) for installing Rust. You need only execute:

```bash
curl https://sh.rustup.rs -sSf | sh
```

in your shell and the installer will take you through the process. I'd recommend installing `nightly` as your default toolchain as some nice extra features like compiler plugins are currently restricted to it. 

If you're on Windows, you'll need to download an appropriate installer from the [downloads](https://www.rust-lang.org/en-US/downloads.html) page.

With Rust installed, you'll have access to some new tools in your shell. The most important of these is `cargo` and the only one we'll use in this post. Cargo is Rust's package manager as well as a wrapper around `rustc`, the compiler. You can do most project management tasks directly from Cargo (near enough all with additional tools like [cargo edit](https://github.com/killercup/cargo-edit), also recommended). If you'd like to know more about Cargo, the [Cargo docs](http://doc.crates.io/guide.html) have you covered.

To create our project, run `cargo new --bin demo-website`. The `--bin` flag tells Cargo we intend to produce an executable binary, as opposed to a library that others will use. It will create a `src/main.rs` file that acts as the entry point. Running `cargo build` inside the root directory of the project (this is always the folder containing the `Cargo.toml` file) will create an executable in `target/debug`. Running `cargo run` should print out the ever joyous `Hello, world!` Executing `cargo run` will automatically do a `cargo build` before running if you've changed the source code since the last build.

Instead of just printing to stdout, how about starting up a server and printing to a web page? It's surprisingly little effort. Read on!

# Servers of Iron

We're now going to deep dive into some Rust code. I'll try to explain the thought process at a high level, but if you're new to the language, you may want to keep the [Rust book](https://doc.rust-lang.org/book/) within reach. We're going to use a library called [Iron](https://crates.io/crates/iron). While web development in Rust is [not extensively mature](http://www.arewewebyet.org/), it's certainly not in its infancy. Iron is one of the older and more established frameworks. Before we write the server, let's make one change to that `Cargo.toml` file. Add `iron` as a dependency, along with one of its utility libraries `router`, such that the dependency section looks like:

```toml
[dependencies]
iron = "0.3"
router = "0.1.1"
```

The `Cargo.toml` file acts as the specification for your project. It contains metadata about the project like authors and current version, pulls any dependencies needed, and can even run custom build scripts. [crates.io](https://crates.io/) acts as a central repository for all Rust projects (called crates) and the information on their is sourced from each project's `Cargo.toml` in turn. We're specifying versions to negate future regressions and ensure the example below runs as intended. In fact, at the time of writing, the latest version of Iron is `0.4`, but `router` [has not been updated](https://github.com/iron/router/issues/117) to use `0.4` and will fail to compile with it.

Before we demonstrate how to use Iron, let's go over the basic structure of a standalone Rust file, as it varies from more conventional languages. We first declare any external crates that are needed for our application. This informs the compiler to compile and link against these. This must be done in addition to declaring them in the `Cargo.toml` file, as that file is for dependency management only, not compiling and linking. We then declare what symbols from these crates we want to bring into scope and use freely. This section can be omitted and you can instead fully qualify any usages. Finally, we have our `main` function, which servers as the entry as in most other languages.

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

    // Execute handler() when we receive a GET request to the root URL
    let mut routes = Router::new();
    routes.get("/", handler);

    // Initialise the router and discard any errors via the unwrap() call
    let url = "localhost:3000";
    println!("Starting server at {}", url);
    Iron::new(routes).http(url).unwrap();
}
```

If you execute `cargo run`, you'll see Cargo download the necessary dependencies, compile them along with out application, and run it. If you navigate to the URL specified in a web browser, you should see the output. This is all that's required to get a server up and running and demonstrates some of the fundamentals of Rust, so let's go through it line by line.

The first two lines state that we want to link to two external crates: `iron` and `router`. The next three lines bring specific structs and functions declared in those crates into scope so that we can call them without further qualification. The `use` call functions similarly to Java's `import` statement, where you can glob import symbols at that level of the hierarchy with an asterisk. The [Iron docs](http://ironframework.io/doc/iron/prelude/index.html) state what's included in the `prelude` module, but you can be sure it contains symbols you will need in most Iron apps. By convention, some Rust libraries include a `prelude` module just for this convenience purpose.

In the `main` function, we first define a function that will handle a request to the server. Rust syntax can be a big jarring at first but very expressive once you are used to it. A key thing to note is that everything is an expression, and if you leave the semicolon off of the last expression of a function, that expression is the *return value*. Our function must return an `IronResult<Response>`, which is a [type alias](https://doc.rust-lang.org/book/type-aliases.html) for `Result<Response, IronError>`. `Result` is one of the two fundamental error handling constructs in Rust and composed of two values: `Ok` or `Err`, signalling expected and error values respectively. In the body of the function, we wrap our actual response in `Ok` to signal everything is as expected. The [error handling](https://doc.rust-lang.org/book/error-handling.html) chapter in the Rust book is a marvellous read and definitely recommended. It explains the `Result` type and its friend, `Option`, in great detail.

The contents of the `Ok` are an implementation detail of what [Iron expects as a response](http://ironframework.io/doc/iron/response/struct.Response.html). Here, we're returning a [tuple](https://doc.rust-lang.org/std/primitive.tuple.html), which is a sequence in Rust that can contain objects of differing types. Iron can accept a response structured as a 2-length tuple containing an HTTP status and a body as the two elements.

The next two lines declare a `Router` object, implemented by the `router` crate and independent of Iron. We then register the function to be called when we receive a `GET` request to the root URL of the application. `let` is used for variable bindings in Rust, whilst `mut` is used to designate mutability in the variable as all variables are immutable by default. The `get` function mutates the `router` variable, so you would get a compiler error if you omitted `mut`. Rust is strongly typed, but supports type inference, which is why we didn't need to explicitly state the type of the variable. The inference algorithm is not perfect though, and may get confused with more advanced constructs. You can explicitly state the type of a variable via this syntax, which is equivalent to what we have already:

```rust
let mut routes: Router = Router::new();
```

The final block of code sets the host and port number that we want to run on, prints this out for logging purposes, and initialises and starts the Iron server. [Macros](https://doc.rust-lang.org/book/macros.html), the methods by which `println!` is implemented, are too large a topic to discuss here, though one of Rust's greatest assets. You can tell it's a macro by the trailing exclamation mark, though it functions just like a regular function. Macros play a key role in keeping Rust code concise and expressive. A well-designed macro can be treated as a black box, as is the case here for `println!`. It accepts a format string followed by a variable number of arguments corresponding to the number of format specifiers (designated by `{}` in Rust). Rust does not support a variable number of arguments in functions, which is why printing to standard output is implemented as a macro.

The final thing to note is the `unwrap()` call at the very end. Again, the [error handling](https://doc.rust-lang.org/book/error-handling.html) chapter in the Rust book covers this topic nicely. It goes into the specifics of `unwrap()` and why there are better alternatives to it. `unwrap()` will cause an application to panic (gracefully unwind and/or abort) if you unwrap an error value, so it should be avoided in production code, though it is useful for demonstrating other concepts when error handling isn't as much of a concern.

# Templating with Handlebars

Writing a hello world app never gets old, but we can do better. The [`handlebars-iron`](https://crates.io/crates/handlebars-iron) crate adds support for the [Handlebars](http://handlebarsjs.com/) templating language directly to Iron. Templating languages allow for dynamic content to be loaded into an otherwise static HTML page. The blog post you're reading now differs from others based on the URL entered, but the header and footer remain the same across blog posts. This templating promotes clean project structure and keeps things [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself). Add the crate as a dependency in `Cargo.toml`, declare it for linkage at the top of `src/main.rs` with another `extern` statement, and update the `use` statements to include the structs we use below. Let's update the `main` function to this:

```rust
fn main() {
    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Template::new("landing", ()))))
    }

    let mut routes = Router::new();
    routes.get("/", handler);

    // Define a new templating engine that will look in the templates folder of
    // the root project directory for .hbs files, and load them up.
    let mut templates = HandlebarsEngine::new();
    templates.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    templates.reload().expect("Failed to load templates");

    // Iron allows you to arbitrarily link different middlewares together via
    // the Chain struct. Here we link our routing and templating middlewares
    // together.
    let mut chain = Chain::new(routes);
    chain.link_after(templates);

    // Remember to pass the chain to Iron::new() instead of the router
    let url = "localhost:3000";
    println!("Starting server at {}", url);
    Iron::new(chain).http(url).expect("Failed to start the server");
```

We redefine our handler to load the template named `landing.hbs` instead of a static string. The empty tuple passed as the second argument will contain our template data, which will add in a moment. If you look at the [signature](http://ironframework.io/doc/iron/response/struct.Response.html#method.with) of the `Response::with` function, it takes a `Modifier`. The Iron crate itself implements this trait for `String`s, and the Handlebars crate implements it for its `Template` struct, which enables you to easily change the arguments passed like in this example.

Next, we add the directory we intend to store the templates in (as `.hbs` files in this case). We then tell the Handlebars engine to actually read the files stored there with the reload call. The `expect` method is another method related to Rust's error handling. Calling `expect` is just like `unwrap`ing, but in the case of an error, the string you pass to `expect` will be outputted as well. In general, you should use `expect` in place of `unwrap` for the additional information it provides.

The next block introduces `Chain` and, as the comments state, it provides a way to link different middlewares together. More are included with the Iron framework, including [`Mount`](http://ironframework.io/doc/mount/struct.Mount.html), which functions similarly to `Router` but allows you process paths as if they were mounted to a specific predefined mount point. `Mount` is particularly useful for organising statically served content.

We're going to want to populate our template with data, otherwise it's no different to static HTML. Let's create a simple blog post listings page. Looking at the [signature](https://sunng87.github.io/handlebars-iron/handlebars_iron/struct.Template.html#method.new) for the `Template::new` method, we can see it takes anything that implements the [`ToJson`](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/trait.ToJson.html) trait. [Traits](https://doc.rust-lang.org/book/traits.html) function similarly to Java's interfaces, in that they provide a contract of what functionality something must implement. In this case, that something must be able to be serialised to JSON. The [`rustc_serialize`](https://crates.io/crates/rustc-serialize) crate implements it for Rust primitives and some common collections, like `HashMap` and `Vec`. It represents a standard JSON object as a [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), so let's structure our data similarly:

```rust
fn handler(_: &mut Request) -> IronResult<Response> {
    let mut template_data = BTreeMap::new();
    let post1 = String::from("This is the contents of post 1");
    let post2 = String::from("This is the contents of post 2");
    template_data.insert(String::from("posts"), vec![post1, post2] );

    Ok(Response::with((status::Ok, Template::new("landing", template_data)))
}
```

The accompanying template may look like:

```handlebars
<h1>Blog Posts</h1>
{{#each posts}}
  {{this}}
  <br />
{{/each}}
```

We've introduced quite a few new things here. First of all, you can treat `BTreeMap` just like any other collection. It has a map interface and all the methods you would expect, but provides some nice optimisations for JSON representation due to its backing implementation as a binary tree. We use it as our enclosing JSON object. We then declare two posts as strings and create a `Vec` out of them with the `vec!` macro, which will take an arbitrary number of arguments and produce the correct `Vec`. Once again, it is implemented as a macro because Rust does not support variable length argument lists. We then pass the JSON object to our template.

You may be wondering on the need for [`String::from`](https://doc.rust-lang.org/std/convert/trait.From.html#tymethod.from). Rust has two string types: the statically allocated [`str`](https://doc.rust-lang.org/std/primitive.str.html) and the heap allocated [`String`](https://doc.rust-lang.org/std/string/struct.String.html). The `ToJson` trait is not implemented for `str`s in their [borrowed](https://doc.rust-lang.org/book/references-and-borrowing.html) form, as is the case for string literals like above. It is implemented for `String`s however, so we can simply do the conversation and use those.

As for the template, you can use Handlebars-specific features inside double braces, but otherwise the file is parsed as HTML. `each` will iterate over an array, which was serialised from our `posts` `Vec`. `this` represents the current object of the iteration, which is a string in this case (we had a `Vec` of `String`s). If you run the application and view it in a web browser, it should show the heading and two strings separated by a line break. The real power here is when you programmatically add data to this template. The blog post you're reading now is composed of a body, a title, a publication date and other attributes. All of these can be manipulated by Rust prior to being passed to the template, and this allows for dynamic content.

# Getting to Grips with Sass and Flexbox

We have our template rendering now, but it's pretty plain to look at. Let's fix that with some styling. CSS is fairly basic markup and, as a result, a number of preprocessors have come about that add convenience features. [Less](http://lesscss.org/) (of Bootstrap fame) and [Sass](http://sass-lang.com/) are two of the more renowned. I chose to use Sass here, though I imagine Less would work equally well. The advantages compared to CSS are outlined in the [Sass guide](http://sass-lang.com/guide), but these are some of things that particularly stand out for me:

```scss
// _mixin.scss
@mixin border-radius($radius) {
  -webkit-border-radius: $radius;
  -moz-border-radius: $radius;
  -ms-border-radius: $radius;
  -o-border-radius: $radius;
  border-radius: $radius;
}

// main.scss
@import 'mixin'
.box {
  @include border-radius(50%);
}
```

This demonstrates two features of Sass: partials and mixins. Notice the leading underscore in the filename of the first file. This marks it as a partial for Sass and allows you to include its contents in other SCSS files with the `@import` statement, providing a nice way to split up selectors. Mixins offer a method to set multiple selectors with one selector, as shown above where we set the various vendor-prefixed selectors. You can do some really neat things within mixins. [This post](http://joshbroton.com/quick-fix-sass-mixins-for-css-keyframe-animations/) by Josh Broton illustrates a way to use CSS3 animations using a more advanced form, complete with looping and conditional execution.

More fundamental than the preprocessor you use is how you structure your CSS, as this will dictate your site layout. Back when I first started using CSS, getting a layout just right was quite tedious. There's numerous hacks to get around existing limitations, but now there's a nicer solution: Flexbox. Flexbox went through some backwards-incompatible revisions in the early 2010s and subsequently lacked an acceptable level of browser support. Fortunately, it's considered stable today and has support from all major browsers. [Solved by Flexbox](https://philipwalton.github.io/solved-by-flexbox/) is a phenomenal resource showing its power.

This example here is from the sticky footer article on Solved by Flexbox. It demonstrates how to write a footer that remains at the bottom of the page regardless of the amount (or lack thereof) of content in the middle of the page.

```scss
.body {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.content {
  flex-grow: 1;
}

.footer {
  flex-shrink: 0;
}
```

By specifying `display: flex`, we are saying we want the `body` class to act as a Flexbox container. Setting `flex-grow` to `1` for the central content on the page makes it use *all* remaining space available and, as we set our container use the `column` direction, it will use all remaining *vertical* space. `flex-shrink` is set to `0` for the footer to make it stay a consistent size, which is usually what you'd want.

I'd encourage you to visit the aforementioned site to get a taster for Flexbox and see if it could help streamline your CSS.

# Additional Resources

I hope this post has given you an idea of how to create a custom server in Rust for use from a templated and styled front-end, and how it's not too difficult using these technologies. All of them served as the foundations for this website in the beginning.

I came into this with very little web development knowledge beyond basic HTML and CSS, and have learnt much through creating the website thanks to the great resources online. Most I've mentioned at appropriate points in the post, but here are two additional ones I couldn't have done without:

* [A Complete Guide to Flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) - very useful reference for Flexbox selectors
* [W3Schools](http://www.w3schools.com/) - useful general reference for HTML and CSS

Thanks for reading!
