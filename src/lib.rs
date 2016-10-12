#![feature(custom_derive, custom_attribute, plugin, proc_macro)]

#![plugin(diesel_codegen, dotenv_macros)]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate clap;
extern crate chrono;
extern crate dotenv;
extern crate fern;
extern crate handlebars_iron;
extern crate hoedown;
extern crate iron;
extern crate mount;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate urlencoded;

pub mod app;
pub mod cli;
pub mod db;
pub mod route;
