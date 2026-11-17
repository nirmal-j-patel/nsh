// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::zlong;

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct heredocs {
	pub next: *mut heredocs,
	pub r#type: c_int,
	pub pc: c_int,
	pub str: *mut c_char,
}

pub type Wordcode = *mut wordcode;
pub type wordcode = c_uint;

// A binary tree of strings.
// Refer to the "Word code." comment at the top of Src/parse.c for details.
pub type Eccstr = *mut eccstr;

#[repr(C)]
pub struct eccstr {
	// Left child pointers.
	pub left: *mut eccstr, // cbindgen puts typedef for Eccstr type after struct

	// Right child pointers.
	pub right: *mut eccstr, // cbindgen puts typedef for Eccstr type after struct

	// String; pointer into to estate::strs.
	pub str: *mut c_char,

	// Wordcode of a long string, as described in the Src/parse.c comment.
	pub offs: wordcode,

	// Raw memory offset of str in estate::strs.
	pub aoffs: wordcode,

	// ### The number of starts and ends of function definitions up to this point.
	// String reuse may only happen between strings that have the same "nfunc" value.
	pub nfunc: c_int,

	// Hash of str.
	pub hashval: c_uint,
}

/**********************************************/
/* Structure to save and restore parse module */
/**********************************************/
#[repr(C)]
#[derive(Default)]
pub struct ParseStack {
	pub hdocs: *mut heredocs,
	pub incmdpos: c_int,
	pub aliasspaceflag: c_int,
	pub incond: c_int,
	pub inredir: c_int,
	pub incasepat: c_int,
	pub isnewlin: c_int,
	pub infor: c_int,
	pub inrepeat_: c_int,
	pub intypeset: c_int,
	pub eclen: c_int,
	pub ecused: c_int,
	pub ecnpats: c_int,
	pub ecbuf: Wordcode,
	pub ecstrs: Eccstr,
	pub ecsoffs: c_int,
	pub ecssub: c_int,
	pub ecnfunc: c_int,
}
