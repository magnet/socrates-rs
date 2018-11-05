#![feature(core_intrinsics)]
#[macro_use]
extern crate query_interface;

pub mod component;
pub mod module;
pub mod service;

pub type Result<T> = std::result::Result<T, String>;
