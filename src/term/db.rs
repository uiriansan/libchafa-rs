/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaTermDb.html
 */

use crate::ffi;
use crate::term::Info;

/// A ChafaTermDb contains information on terminals, and can be used to obtain a suitable ChafaTermInfo for a terminal environment.
pub struct Db {
    pub raw: *mut ffi::ChafaTermDb,
}

impl Db {
    /// Creates a new, blank ChafaTermDb.
    pub fn new() -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_term_db_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create Db")
        } else {
            Ok(Self { raw })
        }
    }

    /// Gets the global ChafaTermDb. This can normally be used safely in a read-only capacity. The caller should not unref the returned object.
    pub fn default() -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_term_db_get_default() };
        if raw.is_null() {
            Err("Chafa -> Failed to retrieve default Db")
        } else {
            Ok(Self { raw })
        }
    }

    /// Builds a new ChafaTermInfo with capabilities implied by the provided environment variables (principally the TERM variable, but also others).
    ///
    /// envp can be gotten from g_get_environ().
    pub fn detect(&self) -> Result<Info, &'static str> {
        unsafe {
            let envp = ffi::g_get_environ();
            if envp.is_null() {
                return Err("Glib -> Failed to retrieve envp");
            }
            let info = ffi::chafa_term_db_detect(self.raw, envp);
            ffi::g_strfreev(envp);

            if info.is_null() {
                Err("Chafa -> Failed to detect term info")
            } else {
                Ok(Info { raw: info })
            }
        }
    }

    /// Builds a new ChafaTermInfo with fallback control sequences. This can be used with unknown but presumably modern terminals, or to supplement missing capabilities in a detected terminal.
    ///
    /// Fallback control sequences may cause unpredictable behavior and should only be used as a last resort.
    pub fn get_fallback_info(&self) -> Result<Info, &'static str> {
        let info = unsafe { ffi::chafa_term_db_get_fallback_info(self.raw) };
        if info.is_null() {
            Err("Chafa -> Failed to retrieve fallback term info")
        } else {
            Ok(Info { raw: info })
        }
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_term_db_unref(self.raw);
            }
        }
    }
}
