#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;


#[cfg(windows)]
extern crate winapi;

#[cfg(test)]
#[macro_use]
extern crate doc_comment;

#[cfg(test)]
doctest!("../README.md");

#[cfg(windows)]
mod fs;

#[cfg(windows)]
pub use self::fs::remove_dir_all;

// #[cfg(not(windows))]
#[cfg(all(feature = "mesalock_sgx", not(windows)))]
pub use std::untrusted::fs::remove_dir_all;

#[cfg(all(not(feature = "mesalock_sgx"),not(windows)))]
pub use std::fs::remove_dir_all;

// pub use std::fs::remove_dir_all;
