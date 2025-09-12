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
            ffi::chafa_term_info_set_name(self.raw, name.as_ptr() as *const i8);
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
    pub fn get_seq(&self, seq: Seq) {
        ()
    }
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
