// src/lib.rs
pub mod common;
pub mod scanner;
pub mod protocols;
pub mod reporters;

pub use common::types::ProxyType;
pub use scanner::{Scanner, ScanResult, ScannerBuilder};
