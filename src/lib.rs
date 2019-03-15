#![recursion_limit = "1024"]

extern crate base64;
extern crate bincode;
extern crate bitcoin;
extern crate viacoin_bech32;
extern crate chan_signal;
extern crate crypto;
extern crate dirs;
extern crate glob;
extern crate hex;
extern crate hyper;
extern crate libc;
extern crate lru;
extern crate lru_cache;
extern crate num_cpus;
extern crate page_size;
extern crate prometheus;
extern crate rocksdb;
extern crate secp256k1;
extern crate serde;
extern crate stderrlog;
extern crate sysconf;
extern crate time;
extern crate tiny_http;
extern crate url;

#[macro_use]
extern crate chan;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod app;
pub mod bulk;
pub mod config;
pub mod daemon;
pub mod errors;
pub mod fake;
pub mod index;
pub mod mempool;
pub mod metrics;
pub mod query;
pub mod rest;
pub mod signal;
pub mod store;
pub mod util;
