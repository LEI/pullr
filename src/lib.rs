//! This crate provides the [`pullr`](cli) command to merge multiple pull requests into a local branch for testing.
#![deny(missing_docs)]
#[doc = include_str!("../README.md")]
pub mod cli;
mod exec;
mod repo;
