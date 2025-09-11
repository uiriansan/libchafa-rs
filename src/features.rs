/*
 * https://hpjansson.org/chafa/ref/chafa-Features.html#
 */

use crate::ffi;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct Features: u32 {
        /// Flag indicating MMX support.
        const MMX = ffi::ChafaFeatures_CHAFA_FEATURE_MMX;
        /// Flag indicating SSE 4.1 support.
        const SSE41 = ffi::ChafaFeatures_CHAFA_FEATURE_SSE41;
        /// Flag indicating popcnt support.
        const POPCNT = ffi::ChafaFeatures_CHAFA_FEATURE_POPCNT;
        /// Flag indicating AVX2 support.
        const AVX2 = ffi::ChafaFeatures_CHAFA_FEATURE_AVX2;
    }
}

/// Gets a list of the platform-specific features this library was built with.
pub fn get_builtin_features() -> Features {
    let feats = unsafe { ffi::chafa_get_builtin_features() };
    Features::from_bits_truncate(feats as u32)
}

pub fn get_supported_features() -> Features {
    let feats = unsafe { ffi::chafa_get_supported_features() };
    Features::from_bits_truncate(feats as u32)
}

/// Takes a set of flags potentially returned from chafa_get_builtin_features() or chafa_get_supported_features() and generates a human-readable ASCII string descriptor.
pub fn describe_features(features: Features) -> Result<String, &'static str> {
    unsafe {
        let str_p = ffi::chafa_describe_features(features.bits() as u32);

        if str_p.is_null() {
            return Err("Chafa -> Failed to create string from Features");
        } else {
            let str = std::ffi::CStr::from_ptr((*str_p) as *const std::os::raw::c_char)
                .to_string_lossy()
                .into_owned();

            ffi::g_free(str_p as *mut std::os::raw::c_void);

            return Ok(str);
        }
    };
}

/// Queries the maximum number of worker threads to use for parallel processing.
/// # Returns:
/// The number of threads, or -1 if determined automatically
pub fn get_n_threads() -> i32 {
    unsafe { ffi::chafa_get_n_threads() }
}

/// Sets the maximum number of worker threads to use for parallel processing, or -1 to determine this automatically. The default is -1.
///
/// Setting this to 0 or 1 will avoid using thread pools and instead perform all processing in the main thread.
pub fn set_n_threads(n: i32) {
    unsafe {
        ffi::chafa_set_n_threads(n);
    }
}

/// Queries the number of worker threads that will actually be used for parallel processing.
/// # Returns:
/// Number of threads, always >= 1
pub fn get_n_actual_threads() -> i32 {
    unsafe { ffi::chafa_get_n_actual_threads() }
}
