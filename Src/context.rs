// SPDX-License-Identifier: MIT OR Apache-2.0

// This short file provides a home for the stack of saved contexts.
// The actions for saving and restoring are encapsulated within
// individual modules.


use crate::zlong;

use crate::DPUTS;
use crate::hist::HistStack;
use crate::lex::LexStack;
use crate::parse::ParseStack;
use crate::utils::dputs;

use crate::signal::SignalQueueGuard;

use core::ffi::{CStr, c_char, c_int, c_short, c_uchar};

use std::cell::RefCell;
use std::sync::{LazyLock, Mutex};
use std::vec::Vec;

unsafe extern "C" {
	fn hist_context_save(hs: *mut HistStack, toplevel: c_int);
	fn hist_context_restore(hs: *const HistStack, toplevel: c_int);
	fn lex_context_save(ls: *mut LexStack, toplevel: c_int);
	fn lex_context_restore(ls: *const LexStack, toplevel: c_int);
	fn parse_context_save(ps: *mut ParseStack, toplevel: c_int);
	fn parse_context_restore(ps: *const ParseStack, toplevel: c_int);
}

/* Context to save and restore (bit fields) */
#[repr(C)]
pub enum ContextPart {
	/* History mechanism */
	ZCONTEXT_HIST = (1 << 0),
	/* Lexical analyser */
	ZCONTEXT_LEX = (1 << 1),
	/* Parser */
	ZCONTEXT_PARSE = (1 << 2),
}

struct Context {
	context_stack: Vec<(HistStack, LexStack, ParseStack)>,
}

impl Context {
	fn new() -> Context {
		Context {
			context_stack: vec![],
		}
	}

	// save some or all of current context
	fn save_partial(&mut self, parts: u8) {
		let sig_guard = SignalQueueGuard::new();

		let mut hs = HistStack::default();
		let mut ls = LexStack::default();
		let mut ps = ParseStack::default();

		if parts & ContextPart::ZCONTEXT_HIST as u8 != 0 {
			unsafe { hist_context_save(&mut hs, self.context_stack.is_empty() as c_int) }
		}

		if parts & ContextPart::ZCONTEXT_LEX as u8 != 0 {
			unsafe { lex_context_save(&mut ls, self.context_stack.is_empty() as c_int) }
		}

		if parts & ContextPart::ZCONTEXT_PARSE as u8 != 0 {
			unsafe { parse_context_save(&mut ps, self.context_stack.is_empty() as c_int) }
		}

		self.context_stack.push((hs, ls, ps));
	}

	// restore context or part thereof
	fn restore_partial(&mut self, parts: u8) {
		let sig_guard = SignalQueueGuard::new();

		DPUTS!(
			self.context_stack.is_empty(),
			c"BUG: zcontext_restore() without zcontext_save()"
		);

		let mut cs = self.context_stack.pop().unwrap();
		let mut hs = cs.0;
		let mut ls = cs.1;
		let mut ps = cs.2;

		if parts & ContextPart::ZCONTEXT_HIST as u8 != 0 {
			unsafe { hist_context_restore(&mut hs, self.context_stack.is_empty() as c_int) }
		}

		if parts & ContextPart::ZCONTEXT_LEX as u8 != 0 {
			unsafe { lex_context_restore(&mut ls, self.context_stack.is_empty() as c_int) }
		}

		if parts & ContextPart::ZCONTEXT_PARSE as u8 != 0 {
			unsafe { parse_context_restore(&mut ps, self.context_stack.is_empty() as c_int) }
		}
	}
}

static mut CONTEXT: Context = Context {
	context_stack: vec![],
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zcontext_save_partial(mut parts: c_int) {
	CONTEXT.save_partial(parts as u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zcontext_restore_partial(mut parts: c_int) {
	CONTEXT.restore_partial(parts as u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zcontext_save() {
	let parts = ContextPart::ZCONTEXT_HIST as u8
		| ContextPart::ZCONTEXT_LEX as u8
		| ContextPart::ZCONTEXT_PARSE as u8;
	CONTEXT.save_partial(parts);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zcontext_restore() {
	let parts = ContextPart::ZCONTEXT_HIST as u8
		| ContextPart::ZCONTEXT_LEX as u8
		| ContextPart::ZCONTEXT_PARSE as u8;
	CONTEXT.restore_partial(parts);
}
