// Nova Programming Language Runtime Library

pub mod vm;
pub mod memory;
pub mod gc;
pub mod native;

// Re-exports for public API
pub use vm::*;
pub use memory::*;
pub use gc::*;
pub use native::*;