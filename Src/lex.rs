// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::zlong;

use core::ffi::{c_char, c_int, c_short, c_uchar};

// Lexical tokens: unlike the character tokens above, these never
// appear in strings and don't necessarily represent a single character.
// See Src/lex.c:tokstrings[] for hints on what these mean.  Note that
// SEPER or SEMI are both stringified as ";".
#[repr(C)]
#[derive(Default)]
pub enum Token {
	#[default]
	NULLTOK, /* 0  */
	SEPER,
	NEWLIN,
	SEMI,
	DSEMI,
	AMPER, /* 5  */
	INPAR,
	OUTPAR,
	DBAR,
	DAMPER,
	OUTANG, /* 10 */
	OUTANGBANG,
	DOUTANG,
	DOUTANGBANG,
	INANG,
	INOUTANG, /* 15 */
	DINANG,
	DINANGDASH,
	INANGAMP,
	OUTANGAMP,
	AMPOUTANG, /* 20 */
	OUTANGAMPBANG,
	DOUTANGAMP,
	DOUTANGAMPBANG,
	TRINANG,
	BAR, /* 25 */
	BARAMP,
	INOUTPAR,
	DINPAR,
	DOUTPAR,
	AMPERBANG, /* 30 */
	SEMIAMP,
	SEMIBAR,
	DOUTBRACK,
	STRING,
	ENVSTRING, /* 35 */
	ENVARRAY,
	ENDINPUT,
	LEXERR,

	/* Tokens for reserved words */
	BANG,     /* !         */
	DINBRACK, /* [[        */
	/* 40 */
	INBRACE,  /* {         */
	OUTBRACE, /* }         */
	CASE,     /* case      */
	COPROC,   /* coproc    */
	DOLOOP,   /* do        */
	/* 45 */
	DONE, /* done      */
	ELIF, /* elif      */
	ELSE, /* else      */
	ZEND, /* end       */
	ESAC, /* esac      */
	/* 50 */
	FI,      /* fi        */
	FOR,     /* for       */
	FOREACH, /* foreach   */
	FUNC,    /* function  */
	IF,      /* if        */
	/* 55 */
	NOCORRECT, /* nocorrect */
	REPEAT,    /* repeat    */
	SELECT,    /* select    */
	THEN,      /* then      */
	TIME,      /* time      */
	/* 60 */
	UNTIL,   /* until     */
	WHILE,   /* while     */
	TYPESET, /* typeset or similar */
}

#[repr(C)]
#[derive(Default)]
pub struct LexBufState {
	pub ptr: *mut c_char,
	pub siz: c_int,
	pub len: c_int,
}

/********************************************/
/* Structure to save and restore lex module */
/********************************************/
#[repr(C)]
#[derive(Default)]
pub struct LexStack {
	pub dbparens: c_int,
	pub isfirstln: c_int,
	pub isfirstch: c_int,
	pub lexflags: c_int,
	pub tok: Token,
	pub tokstr: *mut c_char,
	pub zshlextext: *mut c_char,
	pub lexbuf: LexBufState,
	pub lex_add_raw: c_int,
	pub tokstr_raw: *mut c_char,
	pub lexbuf_raw: LexBufState,
	pub lexstop: c_int,
	pub toklineno: zlong,
}
