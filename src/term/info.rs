use crate::{SymbolTags, ffi};

/// A ChafaTermInfo describes the characteristics of one particular kind of display terminal. It stores control sequences that can be used to move the cursor, change text attributes, mark the beginning and end of sixel graphics data, etc.
///
/// ChafaTermInfo also implements an efficient low-level API for formatting these sequences with marshaled arguments so they can be sent to the terminal.
pub struct Info {
    pub raw: *mut ffi::ChafaTermInfo,
}

impl Info {
    /// Creates a new, blank ChafaTermInfo.
    pub fn new() -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_term_info_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create term info")
        } else {
            Ok(Self { raw })
        }
    }

    /// Terminal emulators and applications are often nested, with the inner application's capabilities limiting, extending or modifying the outer's.
    ///
    /// Examples are terminal multiplexers like Screen and tmux, or terminal emulators running inside editors like Emacs and vi.
    ///
    /// This merges the outer and inner sequences into a single ChafaTermInfo according to the following rules.
    ///
    /// For sequences marked as inherited in inner :
    ///
    /// If either inner or outer sequence is NULL, pick the outer sequence.
    ///
    /// Otherwise, pick inner sequence.
    ///
    /// For sequences not marked as inherited, always use the inner sequence.
    ///
    /// This allows for using the inner term's sequences while clearing them if the outer term does not support the sequence at all. This is useful for muxers (e.g. fbterm supports 256 colors, but with private seqs; we want to use the inner mux' corresponding seqs).
    ///
    /// The merged ChafaTermInfo is a new instance, with the initial reference owned by the caller.
    ///
    /// This function can be used repeatedly to create chains that're arbitrarily long, but is unlikely to be useful beyond three levels (terminal emulator, multiplexer, application).
    pub fn chain(outer: Self, inner: Self) -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_term_info_chain(outer.raw, inner.raw) };
        if raw.is_null() {
            Err("Chafa -> Failed to merge term info")
        } else {
            Ok(Self { raw })
        }
    }

    /// Supplements missing sequences in term_info with ones copied from source .
    pub fn supplement(&self, source: Self) {
        unsafe {
            ffi::chafa_term_info_supplement(self.raw, source.raw);
        }
    }

    /// Gets the name associated with the term_info . This may be NULL. The returned string belongs to term_info , and is only valid until the next operation on this data structure.
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            let str = ffi::chafa_term_info_get_name(self.raw);
            if str.is_null() {
                return None;
            } else {
                let c_str = std::ffi::CStr::from_ptr(str as *const std::os::raw::c_char);
                return Some(c_str.to_string_lossy().into_owned());
            }
        };
    }

    /// Assigns a new name to term_info . The name should be a short lowercase ASCII string that uniquely identifies the terminal or program described by term_info.
    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::chafa_term_info_set_name(
                self.raw,
                std::ffi::CString::new(name)
                    .expect("Chafa -> Failed to create CString")
                    .as_ptr() as *const i8,
            );
        }
    }

    /// Gets the quirks associated with term_info.
    pub fn get_quirks(&self) -> Quirks {
        unsafe { Quirks::from_bits_truncate(ffi::chafa_term_info_get_quirks(self.raw)) }
    }

    /// Assigns a set of quirks to term_info .
    pub fn set_quirks(&self, quirks: Quirks) {
        unsafe {
            ffi::chafa_term_info_set_quirks(self.raw, quirks.bits());
        }
    }

    /// Gets the ChafaSymbolTags that are likely safe to use with term_info . The ChafaSymbolTags are a bitwise OR of flags from the enum.
    pub fn get_safe_symbol_tags(&self) -> SymbolTags {
        unsafe {
            SymbolTags::from_bits_truncate(ffi::chafa_term_info_get_safe_symbol_tags(self.raw))
        }
    }

    /// Sets the ChafaSymbolTags that are likely safe to use with term_info . The tags are a bitwise OR of flags from the enum.
    pub fn set_safe_symbol_tags(&self, tags: SymbolTags) {
        unsafe {
            ffi::chafa_term_info_set_safe_symbol_tags(self.raw, tags.bits());
        }
    }

    /// Gets the string equivalent of seq stored in term_info .
    pub fn get_seq(&self, seq: Seq) -> Option<String> {
        unsafe {
            let seq_str = ffi::chafa_term_info_get_seq(self.raw, seq as u32);

            if seq_str.is_null() {
                None
            } else {
                let s = std::ffi::CStr::from_ptr(seq_str as *const std::os::raw::c_char)
                    .to_string_lossy()
                    .into_owned();
                ffi::g_free(seq_str as *mut _);
                Some(s)
            }
        }
    }

    /// Sets the control sequence string equivalent of seq stored in term_info to str .
    ///
    /// The string may contain argument indexes to be substituted with integers on formatting. The indexes are preceded by a percentage character and start at 1, i.e. %1, %2, %3, etc.
    ///
    /// The string's length after formatting must not exceed CHAFA_TERM_SEQ_LENGTH_MAX bytes. Each argument can add up to four digits, or three for those specified as 8-bit integers. If the string could potentially exceed this length when formatted, chafa_term_info_set_seq() will return FALSE.
    ///
    /// If parsing fails or str is too long, any previously existing sequence will be left untouched.
    ///
    /// Passing NULL for str clears the corresponding control sequence.
    /// # Parameters:
    /// --- `seq_str`: A control sequence string, or None to clear;
    pub fn set_seq(&self, seq: Seq, seq_str: Option<&str>) -> Result<(), String> {
        let mut seq_str_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
        let c_str: std::ffi::CString;
        if let Some(s) = seq_str {
            c_str = std::ffi::CString::new(s).expect("Chafa -> Failed to create CString");
            seq_str_ptr = c_str.as_ptr() as *mut _;
        }
        let mut error: *mut ffi::GError = std::ptr::null_mut();

        unsafe {
            ffi::chafa_term_info_set_seq(self.raw, seq as u32, seq_str_ptr, &mut error);

            if !error.is_null() {
                let msg = std::ffi::CStr::from_ptr((*error).message as *const std::os::raw::c_char)
                    .to_string_lossy()
                    .into_owned();
                let err_msg = format!("Chafa -> Failed to apply selectors: {}", &msg);

                ffi::g_error_free(error);
                return Err(err_msg);
            }
        }

        Ok(())
    }

    /// Checks if term_info can emit seq.
    pub fn have_seq(&self, seq: Seq) -> bool {
        if unsafe { ffi::chafa_term_info_have_seq(self.raw, seq as u32) } == 0 {
            false
        } else {
            true
        }
    }

    /// Gets whether seq can be inherited from the outer ChafaTermInfo when chaining with chafa_term_info_chain().
    pub fn get_inherit_seq(&self, seq: Seq) -> bool {
        if unsafe { ffi::chafa_term_info_get_inherit_seq(self.raw, seq as u32) } == 0 {
            false
        } else {
            true
        }
    }

    /// Sets whether seq can be inherited from the outer ChafaTermInfo when chaining with chafa_term_info_chain().
    pub fn set_inherit_seq(&self, seq: Seq, inherit: bool) {
        unsafe {
            ffi::chafa_term_info_set_inherit_seq(self.raw, seq as u32, if inherit { 1 } else { 0 });
        }
    }

    // TODO: I honestly have no idea how to implement these easily.
    // pub fn emit_seq(&self) {}
    // pub fn emit_seq_valist(&self) {}

    /// Attempts to parse a terminal sequence from an input data array. If successful, CHAFA_PARSE_SUCCESS will be returned, the input pointer will be advanced and the parsed length will be subtracted from input_len .
    ///
    /// Any numeric parsed arguments are returned as an array starting at args_out , which must have room for up to CHAFA_TERM_SEQ_ARGS_MAX elements.
    ///
    /// The number of parsed arguments is returned in n_args_out . This is useful for seqs with a variable number of arguments, like CHAFA_TERM_SEQ_PRIMARY_DEVICE_ATTRIBUTES.
    ///
    /// Either or both of args_out and n_args_out can be NULL, in which case nothing is returned for that parameter.
    pub fn parse_seq_varargs(&self, seq: Seq) {}
}

impl Drop for Info {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_term_info_unref(self.raw);
            }
        }
    }
}

pub const CHAFA_TERM_SEQ_ARGS_MAX: u32 = 24;
pub const CHAFA_TERM_SEQ_LENGTH_MAX: u32 = 96;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct Quirks: u32 {
        const SixelOvershoot = ffi::ChafaTermQuirks_CHAFA_TERM_QUIRK_SIXEL_OVERSHOOT;
    }
}

#[repr(u32)]
/// An enumeration of the possible return values from the parsing function.
pub enum ParseResult {
    Success = ffi::ChafaParseResult_CHAFA_PARSE_SUCCESS,
    /// Data mismatch
    Failure = ffi::ChafaParseResult_CHAFA_PARSE_FAILURE,
    /// Partial success, but not enough input
    Again = ffi::ChafaParseResult_CHAFA_PARSE_AGAIN,
}
impl From<u32> for ParseResult {
    fn from(value: u32) -> Self {
        match value {
            ffi::ChafaParseResult_CHAFA_PARSE_SUCCESS => ParseResult::Success,
            ffi::ChafaParseResult_CHAFA_PARSE_FAILURE => ParseResult::Failure,
            ffi::ChafaParseResult_CHAFA_PARSE_AGAIN => ParseResult::Again,
            _ => ParseResult::Success,
        }
    }
}
impl From<ParseResult> for u32 {
    fn from(value: ParseResult) -> Self {
        match value {
            ParseResult::Success => ffi::ChafaParseResult_CHAFA_PARSE_SUCCESS,
            ParseResult::Failure => ffi::ChafaParseResult_CHAFA_PARSE_FAILURE,
            ParseResult::Again => ffi::ChafaParseResult_CHAFA_PARSE_AGAIN,
        }
    }
}

/*
 *
 * The enum for `ChafaTermSeq` is generated at build time.
 *
 */

macro_rules! gen_termseq {
    ($($name:ident = $ffi_const:path,)+) => {
        #[repr(u32)]
        pub enum Seq {
            $($name = $ffi_const,)+
        }

        impl From<Seq> for u32 {
            fn from(value: Seq) -> u32 {
                match value {
                    $(Seq::$name => $ffi_const,)+
                }
            }
        }

        impl From<u32> for Seq {
            fn from(value: u32) -> Seq {
                match value {
                    $($ffi_const => Seq::$name,)*
                    _ => panic!("Invalid value for ChafaTermSeq: {value}")
                }
            }
        }
    };
}

include!("term_seq.rs");
