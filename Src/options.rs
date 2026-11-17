// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::zlong;

use core::ffi::{c_char, c_int, c_short, c_uchar};

// current emulation (used to decide which set of option letters is used)
#[unsafe(no_mangle)]
static mut emulation: c_int = 0;

// the options; e.g. if opts[SHGLOB] != 0, SH_GLOB is turned on
#[unsafe(no_mangle)]
static mut opts: [c_char; OptionsIndex::OPT_SIZE as usize] =
	[0 as c_char; OptionsIndex::OPT_SIZE as usize];

// current sticky emulation:  sticky = NULL means none
#[unsafe(no_mangle)]
pub static mut sticky: Emulation_options =
	core::ptr::null::<emulation_options>() as *mut emulation_options;

/* Possible values of emulation */
#[repr(C)]
pub enum EmulateOptions {
	EMULATE_CSH = 1 << 1, // C shell
	EMULATE_KSH = 1 << 2, // Korn shell
	EMULATE_SH = 1 << 3,  // Bourne shell
	EMULATE_ZSH = 1 << 4, // `native` mode
	
	// Additional flags
	EMULATE_FULLY = 1<<5, // "emulate -R" in effect
	EMULATE_UNUSED = 1<<6, // Higher bits are used in options.rs, record lowest unused bit...
}

/* option indices */
#[repr(C)]
pub enum OptionsIndex {
	OPT_INVALID,
	ALIASESOPT,
	ALIASFUNCDEF,
	ALLEXPORT,
	ALWAYSLASTPROMPT,
	ALWAYSTOEND,
	APPENDHISTORY,
	AUTOCD,
	AUTOCONTINUE,
	AUTOLIST,
	AUTOMENU,
	AUTONAMEDIRS,
	AUTOPARAMKEYS,
	AUTOPARAMSLASH,
	AUTOPUSHD,
	AUTOREMOVESLASH,
	AUTORESUME,
	BADPATTERN,
	BANGHIST,
	BAREGLOBQUAL,
	BASHAUTOLIST,
	BASHREMATCH,
	BEEP,
	BGNICE,
	BRACECCL,
	BSDECHO,
	CASEGLOB,
	CASEMATCH,
	CASEPATHS,
	CBASES,
	CDABLEVARS,
	CDSILENT,
	CHASEDOTS,
	CHASELINKS,
	CHECKJOBS,
	CHECKRUNNINGJOBS,
	CLOBBER,
	CLOBBEREMPTY,
	APPENDCREATE,
	COMBININGCHARS,
	COMPLETEALIASES,
	COMPLETEINWORD,
	CORRECT,
	CORRECTALL,
	CONTINUEONERROR,
	CPRECEDENCES,
	CSHJUNKIEHISTORY,
	CSHJUNKIELOOPS,
	CSHJUNKIEQUOTES,
	CSHNULLCMD,
	CSHNULLGLOB,
	DEBUGBEFORECMD,
	EMACSMODE,
	EQUALS,
	ERREXIT,
	ERRRETURN,
	EXECOPT,
	EXTENDEDGLOB,
	EXTENDEDHISTORY,
	EVALLINENO,
	FLOWCONTROL,
	FORCEFLOAT,
	FUNCTIONARGZERO,
	GLOBOPT,
	GLOBALEXPORT,
	GLOBALRCS,
	GLOBASSIGN,
	GLOBCOMPLETE,
	GLOBDOTS,
	GLOBSTARSHORT,
	GLOBSUBST,
	HASHCMDS,
	HASHDIRS,
	HASHEXECUTABLESONLY,
	HASHLISTALL,
	HISTALLOWCLOBBER,
	HISTBEEP,
	HISTEXPIREDUPSFIRST,
	HISTFCNTLLOCK,
	HISTFINDNODUPS,
	HISTIGNOREALLDUPS,
	HISTIGNOREDUPS,
	HISTIGNORESPACE,
	HISTLEXWORDS,
	HISTNOFUNCTIONS,
	HISTNOSTORE,
	HISTREDUCEBLANKS,
	HISTSAVEBYCOPY,
	HISTSAVENODUPS,
	HISTSUBSTPATTERN,
	HISTVERIFY,
	HUP,
	IGNOREBRACES,
	IGNORECLOSEBRACES,
	IGNOREEOF,
	INCAPPENDHISTORY,
	INCAPPENDHISTORYTIME,
	INTERACTIVE,
	INTERACTIVECOMMENTS,
	KSHARRAYS,
	KSHAUTOLOAD,
	KSHGLOB,
	KSHOPTIONPRINT,
	KSHTYPESET,
	KSHZEROSUBSCRIPT,
	LISTAMBIGUOUS,
	LISTBEEP,
	LISTPACKED,
	LISTROWSFIRST,
	LISTTYPES,
	LOCALLOOPS,
	LOCALOPTIONS,
	LOCALPATTERNS,
	LOCALTRAPS,
	LOGINSHELL,
	LONGLISTJOBS,
	MAGICEQUALSUBST,
	MAILWARNING,
	MARKDIRS,
	MENUCOMPLETE,
	MONITOR,
	MULTIBYTE,
	MULTIFUNCDEF,
	MULTIOS,
	NOMATCH,
	NOTIFY,
	NULLGLOB,
	NUMERICGLOBSORT,
	OCTALZEROES,
	OVERSTRIKE,
	PATHDIRS,
	PATHSCRIPT,
	PIPEFAIL,
	POSIXALIASES,
	POSIXARGZERO,
	POSIXBUILTINS,
	POSIXCD,
	POSIXIDENTIFIERS,
	POSIXJOBS,
	POSIXSTRINGS,
	POSIXTRAPS,
	PRINTEIGHTBIT,
	PRINTEXITVALUE,
	PRIVILEGED,
	PROMPTBANG,
	PROMPTCR,
	PROMPTPERCENT,
	PROMPTSP,
	PROMPTSUBST,
	PUSHDIGNOREDUPS,
	PUSHDMINUS,
	PUSHDSILENT,
	PUSHDTOHOME,
	RCEXPANDPARAM,
	RCQUOTES,
	RCS,
	RECEXACT,
	REMATCHPCRE,
	RMSTARSILENT,
	RMSTARWAIT,
	SHAREHISTORY,
	SHFILEEXPANSION,
	SHGLOB,
	SHINSTDIN,
	SHNULLCMD,
	SHOPTIONLETTERS,
	SHORTLOOPS,
	SHORTREPEAT,
	SHWORDSPLIT,
	SINGLECOMMAND,
	SINGLELINEZLE,
	SOURCETRACE,
	SUNKEYBOARDHACK,
	TRANSIENTRPROMPT,
	TRAPSASYNC,
	TYPESETSILENT,
	TYPESETTOUNSET,
	UNSET,
	VERBOSE,
	VIMODE,
	WARNCREATEGLOBAL,
	WARNNESTEDVAR,
	XTRACE,
	USEZLE,
	DVORAK,
	OPT_SIZE,
}

//  Record of emulation and options that need to be set
//  for a full "emulate".
#[repr(C)]
pub struct emulation_options {
	// The emulation itself
	pub emulation: c_int,

	// The number of options in on_opts.
	pub n_on_opts: c_int,

	// The number of options in off_opts.
	pub n_off_opts: c_int,

	// Array of options to be turned on.
	// Only options specified explicitly in the emulate command
	// are recorded.  Null if n_on_opts is zero.
	pub on_opts: *mut OptIndex,

	// Array of options to be turned off, similar.
	pub off_opts: *mut OptIndex,
}

// Size required to fit an option number.
// If OPT_SIZE goes above 256 this will need to expand.
pub type OptIndex = c_uchar;

pub type Emulation_options = *mut emulation_options;

const OPT_CSH: u8 = EmulateOptions::EMULATE_CSH as u8;
const OPT_KSH: u8 = EmulateOptions::EMULATE_KSH as u8;
const OPT_SH: u8 = EmulateOptions::EMULATE_SH as u8;
const OPT_ZSH: u8 = EmulateOptions::EMULATE_ZSH as u8;

const OPT_ALL: u8 = 	(OPT_CSH|OPT_KSH|OPT_SH|OPT_ZSH);
const OPT_BOURNE: u8 = (OPT_KSH|OPT_SH);
const OPT_BSHELL: u8 = (OPT_KSH|OPT_SH|OPT_ZSH);
const OPT_NONBOURNE: u8 = (OPT_ALL & !OPT_BOURNE);
const OPT_NONZSH: u8 = (OPT_ALL & !OPT_ZSH);


#[unsafe(no_mangle)]
pub unsafe extern "C" fn isset(x: c_int) -> bool {
	opts[x as usize] != 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unset(x: c_int) -> bool {
	opts[x as usize] == 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EMULATION(x: c_int) -> c_int {
	emulation & x
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SHELL_EMULATION() -> c_int {
	emulation & ((1<<5)-1)
}
