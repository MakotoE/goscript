#[macro_use]
mod util;
mod display;

mod assignment;
mod builtin;
mod call;
pub mod check;
mod decl;
mod expr;
mod interface;
mod resolver;
mod stmt;
mod typexpr;

pub use check::Checker;
pub use interface::{IfaceInfo, MethodInfo};
pub use resolver::DeclInfo;
