#![feature(custom_derive, custom_attribute, plugin)]

#![plugin(diesel_codegen, dotenv_macros, serde_macros)]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate diesel;
#[macro_use] extern crate log;

extern crate fern;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron;
extern crate hoedown;
extern crate rustc_serialize;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

pub mod db;
pub mod route;
