use crate::zlong;

use crate::hist::hist_stack;
use crate::lex::lex_stack;
use crate::parse::parse_stack;

use crate::{queue_signals_function, unqueue_signals_function};

use core::ffi::{c_char, c_int, c_short, c_uchar};

use std::vec::Vec;

unsafe extern "C" {
    fn hist_context_save(hs: *mut hist_stack, toplevel: c_int);
    fn hist_context_restore(hs: *const hist_stack, toplevel: c_int);
    fn lex_context_save(ls: *mut lex_stack, toplevel: c_int);
    fn lex_context_restore(ls: *const lex_stack, toplevel: c_int);
    fn parse_context_save(ps: *mut parse_stack, toplevel: c_int);
    fn parse_context_restore(ps: *const parse_stack, toplevel: c_int);
}

/* Context to save and restore (bit fields) */
#[repr(C)]
pub enum ContextParts {
    /* History mechanism */
    ZCONTEXT_HIST       = (1<<0),
    /* Lexical analyser */
    ZCONTEXT_LEX        = (1<<1),
    /* Parser */
    ZCONTEXT_PARSE      = (1<<2)
}

struct Context {
        context_stack: Vec<(hist_stack, lex_stack, parse_stack)>,
}

impl Context {
        fn save_partial(&mut self, parts: u8) {
                unsafe { queue_signals_function(); }
                let mut hs: hist_stack = hist_stack::default();
                let mut ls: lex_stack = lex_stack::default();
                let mut ps: parse_stack = parse_stack::default();

                if parts & ContextParts::ZCONTEXT_HIST as u8 != 0{
                        unsafe { hist_context_save(&mut hs, self.context_stack.is_empty() as c_int) }
                }

                if parts & ContextParts::ZCONTEXT_LEX as u8 != 0{
                        unsafe { lex_context_save(&mut ls, self.context_stack.is_empty() as c_int) }
                }

                if parts & ContextParts::ZCONTEXT_PARSE as u8 != 0{
                        unsafe { parse_context_save(&mut ps, self.context_stack.is_empty() as c_int) }

                }

                self.context_stack.push((hs, ls, ps));

                unsafe { unqueue_signals_function(); }
        }

        fn restore_partial(&mut self, parts: u8) {

        }
}
