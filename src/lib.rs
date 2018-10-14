#![warn(clippy::pedantic)]
//! Gyrus is a new tool for creating machine learning that runs blazingly fast when learning has finnished.

pub mod generation;
mod tests;

pub(crate) const DEBUG: DebugList = DebugList { check: false };

pub(crate) struct DebugList {
    check: bool,
}
