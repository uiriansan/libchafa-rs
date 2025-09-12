/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaCanvas.html
 */

use crate::canvas::Config;
use crate::misc;
use crate::{ffi, placement::Placement, term::Info};
use std::{ffi::CStr, fmt::write};

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
    /// # Parameters:
    /// --- `config`: Configuration to use.
    pub fn new(config: &Config) -> Result<Self, &'static str> {
        let raw: *mut ffi::ChafaCanvas = unsafe { ffi::chafa_canvas_new(config.raw) };
        if raw.is_null() {
            Err("Chafa -> Failed to create canvas")
        } else {
            Ok(Canvas { raw })
        }
    }

    /// Returns the configuration belonging to canvas .
    /// This can be inspected using the ChafaCanvasConfig getter functions, but not changed.
    pub fn config(&self) -> Result<Config, &'static str> {
        let raw: *const ffi::ChafaCanvasConfig = unsafe { ffi::chafa_canvas_peek_config(self.raw) };

        if raw.is_null() {
            Err("Chafa -> Failed to retrieve config")
        } else {
            Ok(Config { raw: raw as *mut _ })
        }
    }

    /// Places placement on canvas , replacing the latter's content. The placement will cover the entire canvas.
    ///
    /// The canvas will keep a reference to the placement until it is replaced or the canvas itself is freed.
    pub fn set_placement(&self, placement: Placement) {
        unsafe {
            ffi::chafa_canvas_set_placement(self.raw, placement.raw);
        }
    }

    /// Replaces pixel data of canvas with a copy of that found at pixels,
    /// which must be in one of the formats supported by ChafaPixelType.
    /// # Parameters:
    /// --- `pixel_type`: Pixel format of src_pixels;
    /// --- `pixels`: Image bytes;
    /// --- `src_width`: Width in pixels of source pixel data;
    /// --- `src_height`: Height in pixels of source pixel data;
    /// --- `src_rowstride`: Number of bytes between the start of each pixel row.
    pub fn set_pixels(
        &self,
        pixels: &[u8],
        pixel_type: misc::PixelType,
        src_width: i32,
        src_height: i32,
        src_rowstride: i32,
    ) {
        unsafe {
            ffi::chafa_canvas_draw_all_pixels(
                self.raw,
                pixel_type as u32,
                pixels.as_ptr(),
                src_width,
                src_height,
                src_rowstride,
            );
        }
    }

    /// Builds a UTF-8 string of terminal control sequences and symbols representing the canvas' current contents.
    /// This can be printed to a terminal. The exact choice of escape sequences and symbols, dimensions, etc. is determined by the configuration assigned to canvas on its creation.
    ///
    /// All output lines except for the last one will end in a newline.
    /// # Parameters:
    /// --- `term_info`: Terminal to format for, or `None` for fallback
    pub fn create_string(&self, term_info: Option<Info>) -> Result<String, &'static str> {
        let mut term: *mut ffi::ChafaTermInfo = std::ptr::null_mut();

        if let Some(ti) = term_info {
            term = ti.raw;
        }
        let g_str = unsafe { ffi::chafa_canvas_print(self.raw, term) };
        if g_str.is_null() {
            return Err("Chafa -> Failed to retrieve GString from `chafa_canvas_print()`");
        }
        let str = unsafe { std::ffi::CStr::from_ptr((*g_str).str_ as *const std::os::raw::c_char) };
        unsafe {
            ffi::g_string_free(g_str, 1);
        }
        Ok(str.to_string_lossy().into_owned())
    }

    /// Builds an array of UTF-8 strings made up of terminal control sequences and symbols representing the canvas' current contents.
    /// These can be printed to a terminal. The exact choice of escape sequences and symbols, dimensions, etc. is determined by the configuration assigned to canvas on its creation.
    ///
    /// When the canvas' pixel mode is CHAFA_PIXEL_MODE_SYMBOLS, each element will hold the contents of exactly one symbol row.
    /// There will be no row separators, newlines or control sequences to reposition the cursor between rows. Row positioning is left to the caller.
    ///
    /// In other pixel modes, there may be one or more strings, but the splitting criteria should not be relied on.
    /// They must be printed in sequence, exactly as they appear.
    /// # Parameters:
    /// --- `term_info`: Terminal to format for, or `None` for fallback
    pub fn create_string_rows(&self, term_info: Option<Info>) -> Result<Vec<String>, &'static str> {
        let mut term: *mut ffi::ChafaTermInfo = std::ptr::null_mut();

        if let Some(ti) = term_info {
            term = ti.raw;
        }
        let mut vec: Vec<String> = Vec::new();
        unsafe {
            let arr: *mut *mut ffi::gchar = ffi::chafa_canvas_print_rows_strv(self.raw, term);

            if arr.is_null() {
                return Err("Chafa -> Failed to create array of rows");
            }

            let mut i = 0;
            loop {
                let ptr = *arr.add(i);
                if ptr.is_null() {
                    break;
                }
                let str = std::ffi::CStr::from_ptr(ptr as *const std::os::raw::c_char);
                vec.push(str.to_string_lossy().into_owned());

                i += 1;
            }
            ffi::g_strfreev(arr);
        };
        Ok(vec)
    }

    /// Returns the character at cell (x, y). The coordinates are zero-indexed.
    /// For double-width characters, the leftmost cell will contain the character and the rightmost cell will contain 0.
    pub fn get_char_at(&self, x: i32, y: i32) -> char {
        let c: char;

        unsafe {
            c = char::from_u32_unchecked(ffi::chafa_canvas_get_char_at(self.raw, x, y));
        }
        c
    }

    /// Sets the character at cell (x, y). The coordinates are zero-indexed.
    /// For double-width characters, the leftmost cell must contain the character and the cell to the right of it will automatically be set to 0.
    ///
    /// If the character is a nonprintable or zero-width, no change will be made.
    /// # Parameters:
    /// --- `c`: The character value to store;
    /// # Returns:
    /// The number of cells output (0, 1 or 2)
    pub fn set_char_at(&self, c: char, x: i32, y: i32) -> i32 {
        let cells: i32;

        unsafe {
            cells = ffi::chafa_canvas_set_char_at(self.raw, x, y, u32::from(c));
        }
        cells
    }

    /// Gets the colors at cell (x, y). The coordinates are zero-indexed. For double-width characters, both cells will contain the same colors.
    ///
    /// The colors will be -1 for transparency, packed 8bpc RGB otherwise, i.e. 0x00RRGGBB hex.
    ///
    /// If the canvas is in an indexed mode, palette lookups will be made for you.
    /// # Returns:
    /// A tuple containing the foreground and the background color, respectively.
    pub fn get_colors_at(&self, x: i32, y: i32) -> (i32, i32) {
        let mut fg: i32 = -1;
        let mut bg: i32 = -1;

        unsafe {
            ffi::chafa_canvas_get_colors_at(self.raw, x, y, &mut fg, &mut bg);
        }
        (fg, bg)
    }

    /// Sets the colors at cell (x, y). The coordinates are zero-indexed. For double-width characters, both cells will be set to the same color.
    ///
    /// The colors must be -1 for transparency, packed 8bpc RGB otherwise, i.e. 0x00RRGGBB hex.
    ///
    /// If the canvas is in an indexed mode, palette lookups will be made for you.
    /// # Parameters:
    /// --- `fg`: Foreground color;
    /// --- `bg`: Background color;
    pub fn set_colors_at(&self, x: i32, y: i32, fg: i32, bg: i32) {
        unsafe {
            ffi::chafa_canvas_set_colors_at(self.raw, x, y, fg, bg);
        }
    }

    /// Gets the colors at cell (x, y). The coordinates are zero-indexed. For double-width characters, both cells will contain the same colors.
    ///
    /// The colors will be -1 for transparency, packed 8bpc RGB, i.e. 0x00RRGGBB hex in truecolor mode, or the raw pen value (0-255) in indexed modes.
    ///
    /// It's the caller's responsibility to handle the color values correctly according to the canvas mode (truecolor or indexed).
    /// # Returns:
    /// A tuple containing the foreground and the background color, respectively.
    pub fn get_raw_colors_at(&self, x: i32, y: i32) -> (i32, i32) {
        let mut fg: i32 = -1;
        let mut bg: i32 = -1;

        unsafe {
            ffi::chafa_canvas_get_raw_colors_at(self.raw, x, y, &mut fg, &mut bg);
        }
        (fg, bg)
    }

    /// Sets the colors at cell (x, y). The coordinates are zero-indexed. For double-width characters, both cells will be set to the same color.
    ///
    /// The colors must be -1 for transparency, packed 8bpc RGB, i.e. 0x00RRGGBB hex in truecolor mode, or the raw pen value (0-255) in indexed modes.
    ///
    /// It's the caller's responsibility to handle the color values correctly according to the canvas mode (truecolor or indexed).
    /// # Parameters:
    /// --- `fg`: Foreground color;
    /// --- `bg`: Background color;
    pub fn set_raw_colors_at(&self, x: i32, y: i32, fg: i32, bg: i32) {
        unsafe {
            ffi::chafa_canvas_set_raw_colors_at(self.raw, x, y, fg, bg);
        }
    }
}

impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = self.create_string(None);

        if let Ok(s) = str {
            return write!(f, "{}", s);
        }
        write!(f, "")
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
