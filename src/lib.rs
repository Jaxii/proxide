// src/lib.rs
pub mod common;
pub mod protocols;
pub mod reporters;
pub mod scanner;

pub use common::types::ProxyType;
pub use scanner::{ScanResult, Scanner, ScannerBuilder};
