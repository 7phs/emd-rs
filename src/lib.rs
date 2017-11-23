#![feature(test)]
#![feature(rand)]
#![feature(plugin)]
//#![plugin(clippy)]

extern crate libc;
extern crate rand;
extern crate test;

mod emd_c;
pub mod emd;
mod utils;