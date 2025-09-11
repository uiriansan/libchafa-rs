/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaSymbolMap.html
 */

use crate::ffi;
use crate::misc;

/// A ChafaSymbolMap describes a selection of the supported textual symbols that can be used in building a printable output string from a ChafaCanvas.
///
/// To create a new ChafaSymbolMap, use chafa_symbol_map_new(). You can then add symbols to it using chafa_symbol_map_add_by_tags() before copying it into a ChafaCanvasConfig using chafa_canvas_config_set_symbol_map().
///
/// Note that some symbols match multiple tags, so it makes sense to e.g. add symbols matching CHAFA_SYMBOL_TAG_BORDER and then removing symbols matching CHAFA_SYMBOL_TAG_DIAGONAL.
///
/// The number of available symbols is a significant factor in the speed of ChafaCanvas. For the fastest possible operation you could use a single symbol -- CHAFA_SYMBOL_TAG_VHALF works well by itself.
pub struct SymbolMap {
    pub raw: *mut ffi::ChafaSymbolMap,
}

impl SymbolMap {
    /// Creates a new ChafaSymbolMap representing a set of Unicode symbols. The symbol map starts out empty.
    pub fn new() -> Result<Self, &'static str> {
        let raw: *mut ffi::ChafaSymbolMap = unsafe { ffi::chafa_symbol_map_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create symbol map")
        } else {
            Ok(SymbolMap { raw })
        }
    }

    /// Adds symbols matching the set of tags to symbol_map.
    pub fn add_by_tags(&self, tags: SymbolTags) {
        unsafe {
            ffi::chafa_symbol_map_add_by_tags(self.raw, tags.bits() as i32);
        }
    }

    /// Adds symbols in the code point range starting with first and ending with last to symbol_map.
    /// # Parameters:
    /// --- `first`: First code point to add, inclusive;
    /// --- `last`: Last code point to add, inclusive;
    pub fn add_by_range(&self, first: char, last: char) {
        unsafe {
            ffi::chafa_symbol_map_add_by_range(self.raw, u32::from(first), u32::from(last));
        }
    }

    /// Removes symbols matching the set of tags from symbol_map .
    pub fn remove_by_tags(&self, tags: SymbolTags) {
        unsafe {
            ffi::chafa_symbol_map_remove_by_tags(self.raw, tags.bits() as i32);
        }
    }

    /// Removes symbols in the code point range starting with first and ending with last from symbol_map .
    pub fn remove_by_range(&self, first: char, last: char) {
        unsafe {
            ffi::chafa_symbol_map_remove_by_range(self.raw, u32::from(first), u32::from(last));
        }
    }

    /// Parses a string consisting of symbol tags separated by [+-,] and applies the pattern to symbol_map . If the string begins with + or -, it's understood to be relative to the current set in symbol_map , otherwise the map is cleared first.
    ///
    /// The symbol tags are string versions of ChafaSymbolTags, i.e. [all, none, space, solid, stipple, block, border, diagonal, dot, quad, half, hhalf, vhalf, braille, technical, geometric, ascii, extra].
    ///
    /// Examples: "block,border" sets map to contain symbols matching either of those tags. "+block,border-dot,stipple" adds block and border symbols then removes dot and stipple symbols.
    ///
    /// If there is a parse error, none of the changes are applied.
    pub fn apply_selectors(&self, selectors: &str) -> Result<(), String> {
        unsafe {
            let mut error: *mut ffi::GError = std::ptr::null_mut();
            ffi::chafa_symbol_map_apply_selectors(
                self.raw,
                selectors.as_ptr() as *const i8,
                &mut error,
            );
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

    /// Queries whether a symbol map is allowed to use built-in glyphs for symbol selection. This can be turned off if you want to use your own glyphs exclusively (see chafa_symbol_map_add_glyph()).
    ///
    /// Defaults to TRUE.
    pub fn get_allow_builtin_glyphs(&self) -> bool {
        let allow: bool;

        unsafe {
            allow = if ffi::chafa_symbol_map_get_allow_builtin_glyphs(self.raw) == 0 {
                false
            } else {
                true
            };
        }
        allow
    }

    /// Controls whether a symbol map is allowed to use built-in glyphs for symbol selection. This can be turned off if you want to use your own glyphs exclusively (see chafa_symbol_map_add_glyph()).
    ///
    /// Defaults to TRUE.
    pub fn set_allow_builtin_glyphs(&self, use_builtin_glyphs: bool) {
        unsafe {
            ffi::chafa_symbol_map_set_allow_builtin_glyphs(
                self.raw,
                if use_builtin_glyphs { 1 } else { 0 },
            );
        }
    }

    /// Returns data for the glyph corresponding to code_point stored in symbol_map . Any of pixels_out , width_out , height_out and rowstride_out can be NULL, in which case the corresponding data is not retrieved.
    ///
    /// If pixels_out is not NULL, a pointer to freshly allocated memory containing height * rowstride bytes in the pixel format specified by pixel_format will be stored at this address. It must be freed using g_free() when you're done with it.
    ///
    /// Monochrome glyphs (the only kind currently supported) will be rendered as opaque white on a transparent black background (0xffffffff for inked pixels and 0x00000000 for uninked).
    /// # Parameters:
    /// --- `code_point`: A Unicode code point;
    /// --- `pixel_format`: Desired pixel format of pixels_out;
    pub fn get_glyph(&self, code_point: u32, pixel_format: misc::PixelType) -> Option<Glyph> {
        let mut pixels_ptr = std::ptr::null_mut();
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut rowstride: i32 = 0;

        unsafe {
            let ok = ffi::chafa_symbol_map_get_glyph(
                self.raw,
                code_point,
                pixel_format as u32,
                &mut pixels_ptr,
                &mut width,
                &mut height,
                &mut rowstride,
            );
            if ok == 0 {
                return None;
            }
        }

        Some(Glyph {
            pixels: pixels_ptr as *mut u8,
            len: (height * rowstride) as usize,
            width,
            height,
            rowstride,
        })
    }

    /// Assigns a rendered glyph to a Unicode code point. This tells Chafa what the glyph looks like so the corresponding symbol can be used appropriately in output.
    ///
    /// Assigned glyphs override built-in glyphs and any earlier glyph that may have been assigned to the same code point.
    ///
    /// If the input is in a format with an alpha channel, the alpha channel will be used for the shape. If not, an average of the color channels will be used.
    /// # Parameters:
    /// --- `code_point`: The Unicode code point for this glyph;
    /// --- `pixel_format`: Glyph pixel format of pixels;
    pub fn add_glyph(&self, code_point: u32, pixel_format: misc::PixelType, glyph: &Glyph) {
        unsafe {
            ffi::chafa_symbol_map_add_glyph(
                self.raw,
                code_point,
                pixel_format as u32,
                glyph.pixels as *mut std::os::raw::c_void,
                glyph.width,
                glyph.height,
                glyph.rowstride,
            );
        }
    }
}

impl Drop for SymbolMap {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_symbol_map_unref(self.raw);
            }
        }
    }
}

pub struct Glyph {
    /// Pointer to the glyph data.
    pub pixels: *mut u8,
    /// Length of the glyph data.
    pub len: usize,
    /// Width of the glyph, in pixels.
    pub width: i32,
    /// Height of the glyph, in pixels.
    pub height: i32,
    /// Offset from start of one row to the next, in bytes.
    pub rowstride: i32,
}

impl Glyph {
    pub fn as_slice(&self) -> &[u8] {
        if self.pixels.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.pixels, self.len) }
        }
    }
}

impl Drop for Glyph {
    fn drop(&mut self) {
        if !self.pixels.is_null() {
            unsafe {
                ffi::g_free(self.pixels as ffi::gpointer);
            }
        }
    }
}

/// The width of an internal symbol pixel matrix. If you are prescaling input graphics, you will get the best results when scaling to a multiple of this value.
pub const SYMBOL_WIDTH_PIXELS: u32 = ffi::CHAFA_SYMBOL_WIDTH_PIXELS;
/// The height of an internal symbol pixel matrix. If you are prescaling input graphics, you will get the best results when scaling to a multiple of this value.
pub const SYMBOL_HEIGHT_PIXELS: u32 = ffi::CHAFA_SYMBOL_HEIGHT_PIXELS;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct SymbolTags: i32 {
        /// Special value meaning no symbols.
        const None = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_NONE;
        /// Space.
        const Space = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SPACE;
        /// Solid (inverse of space).
        const Solid = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SOLID;
        /// Stipple symbols.
        const Stipple = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_STIPPLE;
        /// Block symbols.
        const Block = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BLOCK;
        /// Border symbols.
        const Border = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BORDER;
        /// Diagonal border symbols.
        const Diagonal = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DIAGONAL;
        /// Symbols that look like isolated dots (excluding Braille).
        const Dot = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DOT;
        /// Quadrant block symbols.
        const Quad = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_QUAD;
        /// Horizontal half block symbols.
        const Hhalf = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_HHALF;
        /// Vertical half block symbols.
        const Vhalf = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_VHALF;
        /// Joint set of horizontal and vertical halves.
        const Half = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_HALF;
        /// Symbols that are the inverse of simpler symbols. When two symbols complement each other, only one will have this tag.
        const Inverted = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_INVERTED;
        /// Braille symbols.
        const Braille = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BRAILLE;
        /// Miscellaneous technical symbols.
        const Technical = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_TECHNICAL;
        /// Geometric shapes
        const Geometric = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_GEOMETRIC;
        /// Printable ASCII characters.
        const ASCII = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ASCII;
        /// Letters.
        const Alpha = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALPHA;
        /// Digits.
        const Digit = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DIGIT;
        /// Joint set of letters and digits.
        const AlNum = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALNUM;
        /// Characters that are one cell wide.
        const Narrow = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_NARROW;
        /// Characters that are two cells wide.
        const Wide = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_WIDE;
        /// Characters of uncertain width. Always excluded unless specifically asked for.
        const Ambiguous = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_AMBIGUOUS;
        /// Characters that are generally undesired or unlikely to render well. Always excluded unless specifically asked for.
        const Ugly = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_UGLY;
        /// Legacy computer symbols, including sextants, wedges and more.
        const Legacy = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_LEGACY;
        /// Sextant 2x3 mosaics.
        const Sextant = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SEXTANT;
        /// Wedge shapes that align with sextants.
        const Wedge = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_WEDGE;
        /// Latin and Latin-like symbols (superset of ASCII).
        const Latin = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_LATIN;
        /// Symbols for which glyphs were imported with chafa_symbol_map_add_glyph().
        const Imported = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_IMPORTED;
        /// Octant 2x4 mosaics.
        const Octant = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_OCTANT;
        /// Symbols not in any other category.
        const Extra = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_EXTRA;
        /// Joint set of ugly and ambiguous characters. Always excluded unless specifically asked for.
        const Bad = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BAD;
        /// Special value meaning all supported symbols.
        const All = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALL;
    }
}
