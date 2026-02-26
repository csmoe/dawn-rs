#![allow(unexpected_cfgs, dead_code, unreachable_patterns)]

mod backend;
mod compat;
mod dispatch;
mod error;
mod future;
mod mapping;
mod types;

pub use compat::{Compat, CompatTexture, WgpuCompatError};
