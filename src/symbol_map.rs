/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaSymbolMap.html
 */

use crate::ffi;

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

    pub fn add_by_tags(&self, tags: SymbolTags) {}
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

/// The width of an internal symbol pixel matrix. If you are prescaling input graphics, you will get the best results when scaling to a multiple of this value.
pub const SYMBOL_WIDTH_PIXELS: u32 = ffi::CHAFA_SYMBOL_WIDTH_PIXELS;
/// The height of an internal symbol pixel matrix. If you are prescaling input graphics, you will get the best results when scaling to a multiple of this value.
pub const SYMBOL_HEIGHT_PIXELS: u32 = ffi::CHAFA_SYMBOL_HEIGHT_PIXELS;

#[repr(i32)]
pub enum SymbolTags {
    /// Special value meaning no symbols.
    None = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_NONE,

    /// Space.
    Space = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SPACE,
    /// Solid (inverse of space).
    Solid = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SOLID,
    /// Stipple symbols.
    Stipple = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_STIPPLE,
    /// Block symbols.
    Block = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BLOCK,
    /// Border symbols.
    Border = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BORDER,
    /// Diagonal border symbols.
    Diagonal = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DIAGONAL,
    /// Symbols that look like isolated dots (excluding Braille).
    Dot = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DOT,
    /// Quadrant block symbols.
    Quad = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_QUAD,
    /// Horizontal half block symbols.
    Hhalf = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_HHALF,
    /// Vertical half block symbols.
    Vhalf = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_VHALF,
    /// Joint set of horizontal and vertical halves.
    Half = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_HALF,
    /// Symbols that are the inverse of simpler symbols. When two symbols complement each other, only one will have this tag.
    Inverted = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_INVERTED,
    /// Braille symbols.
    Braille = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BRAILLE,
    /// Miscellaneous technical symbols.
    Technical = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_TECHNICAL,
    /// Geometric shapes
    Geometric = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_GEOMETRIC,
    /// Printable ASCII characters.
    ASCII = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ASCII,
    /// Letters.
    Alpha = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALPHA,
    /// Digits.
    Digit = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DIGIT,
    /// Joint set of letters and digits.
    AlNum = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALNUM,
    /// Characters that are one cell wide.
    Narrow = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_NARROW,
    /// Characters that are two cells wide.
    Wide = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_WIDE,
    /// Characters of uncertain width. Always excluded unless specifically asked for.
    Ambiguous = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_AMBIGUOUS,
    /// Characters that are generally undesired or unlikely to render well. Always excluded unless specifically asked for.
    Ugly = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_UGLY,
    /// Legacy computer symbols, including sextants, wedges and more.   
    Legacy = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_LEGACY,
    /// Sextant 2x3 mosaics.
    Sextant = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SEXTANT,
    /// Wedge shapes that align with sextants.
    Wedge = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_WEDGE,
    /// Latin and Latin-like symbols (superset of ASCII).
    Latin = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_LATIN,
    /// Symbols for which glyphs were imported with chafa_symbol_map_add_glyph().
    Imported = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_IMPORTED,
    /// Octant 2x4 mosaics.
    Octant = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_OCTANT,
    /// Symbols not in any other category.
    Extra = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_EXTRA,
    /// Joint set of ugly and ambiguous characters. Always excluded unless specifically asked for.
    Bad = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BAD,
    /// Special value meaning all supported symbols.
    All = ffi::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALL,
}
