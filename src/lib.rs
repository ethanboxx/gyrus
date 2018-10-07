#![feature(tool_lints)]
#![warn(clippy::pedantic)]
//! Gyrus is a new tool for creating machine learning that runs blazingly fast when learning has finnished.

pub mod generation;
mod tests;

pub const DEBUG: DebugList = DebugList { check: false };

pub struct DebugList {
    check: bool,
}
