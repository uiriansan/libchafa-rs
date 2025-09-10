use crate::ffi;
mod config;
pub use config::*;

/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaCanvas.html
 */

/// A ChafaCanvas is a canvas that can render its contents as text strings.
///
/// To create a new ChafaCanvas, use chafa_canvas_new(). If you want to specify any parameters, like the geometry, color space and so on, you must create a ChafaCanvasConfig first.
///
/// You can draw an image to the canvas using chafa_canvas_draw_all_pixels() and create an ANSI text (or sixel) representation of the canvas' current contents using chafa_canvas_build_ansi().
pub struct Canvas {
    pub raw: *mut ffi::ChafaCanvas,
}

impl Canvas {
    /// Creates a new canvas with the specified configuration.
    /// The canvas makes a private copy of the configuration, so it will not be affected by subsequent changes.
    /// Parameters:
    /// --- `config`: Configuration to use or NULL for hardcoded defaults
    pub fn new(config: &Config) -> Result<Self, &'static str> {
        let raw: *mut ffi::ChafaCanvas = unsafe { ffi::chafa_canvas_new(config.raw) };
        if raw.is_null() {
            Err("Chafa -> Failed to create canvas")
        } else {
            Ok(Canvas { raw })
        }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_canvas_unref(self.raw);
            }
        }
    }
}
