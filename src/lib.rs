#![feature(test)]
#![feature(rand)]
#![feature(plugin)]
//#![plugin(clippy)]

extern crate libc;
extern crate rand;
extern crate test;
extern crate wordvector as wordvector_base;

pub mod emd;
pub mod wordvector;
mod emd_c;
mod utils;