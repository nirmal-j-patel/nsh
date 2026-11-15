// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod context;
pub mod hist;
pub mod lex;
pub mod parse;

pub type zlong = i64;
pub type zulong = u64;

unsafe extern "C" {
    fn queue_signals_function();
    fn unqueue_signals_function();
}

