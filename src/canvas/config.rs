/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaCanvasConfig.html
 */

use crate::ffi;

/// A ChafaCanvasConfig describes a set of parameters for ChafaCanvas, such as its geometry, color space and other output characteristics.
///
/// To create a new ChafaCanvasConfig, use chafa_canvas_config_new(). You can then modify it using its setters, e.g. chafa_canvas_config_set_canvas_mode() before assigning it to a new ChafaCanvas with chafa_canvas_new().
///
/// Note that it is not possible to change a canvas' configuration after the canvas is created.
pub struct Config {
    pub raw: *mut ffi::ChafaCanvasConfig,
}

impl Config {
    /// Creates a new ChafaCanvasConfig with default settings. This object can later be used in the creation of a ChafaCanvas.
    pub fn new() -> Result<Self, &'static str> {
        let raw: *mut ffi::ChafaCanvasConfig = unsafe { ffi::chafa_canvas_config_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create config")
        } else {
            Ok(Config { raw })
        }
    }

    pub fn new_detect() -> Result<Self, &'static str> {
        let config = Self::new();
        if let Err(e) = config {
            return Err(e);
        }
        let conf = config.unwrap();

        // TODO: ...
        // conf.set_pixel_mode(PixelMode::Sixels);

        Ok(conf)
    }

    /// Returns a tuple containing config's width and height in character cells.
    pub fn get_geometry(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        unsafe {
            ffi::chafa_canvas_config_get_geometry(self.raw, &mut width, &mut height);
        }
        (width, height)
    }

    /// Sets config 's width and height in character cells to width x height.
    pub fn set_geometry(&self, width: i32, height: i32) {
        unsafe {
            ffi::chafa_canvas_config_set_geometry(self.raw, width, height);
        }
    }

    /// Returns a tuple containing config's cell width and height in pixels.
    pub fn get_cell_geometry(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        unsafe {
            ffi::chafa_canvas_config_get_cell_geometry(self.raw, &mut width, &mut height);
        }
        (width, height)
    }

    /// Sets config 's cell width and height in pixels to cell_width x cell_height.
    pub fn set_cell_geometry(&self, width: i32, height: i32) {
        unsafe {
            ffi::chafa_canvas_config_set_cell_geometry(self.raw, width, height);
        }
    }

    /// Returns config's ChafaPixelMode.
    pub fn get_pixel_mode(&self) -> PixelMode {
        let pm: u32;

        unsafe {
            pm = ffi::chafa_canvas_config_get_pixel_mode(self.raw);
        }
        let mode = match pm {
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SYMBOLS => PixelMode::Symbols,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SIXELS => PixelMode::Sixels,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_KITTY => PixelMode::Kitty,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_ITERM2 => PixelMode::Iterm2,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_MAX => PixelMode::Max,
            _ => PixelMode::Symbols,
        };
        mode
    }

    /// Sets config's stored ChafaPixelMode to pixel_mode. This determines how pixel graphics are rendered in the output.
    pub fn set_pixel_mode(&self, mode: PixelMode) {
        let pm: u32 = match mode {
            PixelMode::Symbols => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SYMBOLS,
            PixelMode::Sixels => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SIXELS,
            PixelMode::Kitty => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_KITTY,
            PixelMode::Iterm2 => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_ITERM2,
            PixelMode::Max => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_MAX,
        };
        unsafe {
            ffi::chafa_canvas_config_set_pixel_mode(self.raw, pm);
        }
    }

    /// Returns config 's ChafaCanvasMode. This determines how colors (and color control codes) are used in the output.
    pub fn get_canvas_mode(&self) -> CanvasMode {
        let cm: u32;

        unsafe {
            cm = ffi::chafa_canvas_config_get_canvas_mode(self.raw);
        }
        let mode = match cm {
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_TRUECOLOR => CanvasMode::TrueColor,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_256 => CanvasMode::Indexed256,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_240 => CanvasMode::Indexed240,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16 => CanvasMode::Indexed16,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG_BGFG => CanvasMode::FgbgBgfg,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG => CanvasMode::FgBg,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_8 => CanvasMode::Indexed8,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16_8 => CanvasMode::Indexed168,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_MAX => CanvasMode::Max,
            _ => CanvasMode::TrueColor,
        };
        mode
    }

    /// Sets config 's stored ChafaCanvasMode to mode . This determines how colors (and color control codes) are used in the output.
    pub fn set_canvas_mode(&self, mode: CanvasMode) {
        let cm: u32 = match mode {
            CanvasMode::TrueColor => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_TRUECOLOR,
            CanvasMode::Indexed256 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_256,
            CanvasMode::Indexed240 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_240,
            CanvasMode::Indexed16 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16,
            CanvasMode::FgbgBgfg => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG_BGFG,
            CanvasMode::FgBg => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG,
            CanvasMode::Indexed8 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_8,
            CanvasMode::Indexed168 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16_8,
            CanvasMode::Max => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_MAX,
        };
        unsafe {
            ffi::chafa_canvas_config_set_canvas_mode(self.raw, cm);
        }
    }

    /// Returns config 's ChafaColorExtractor. This determines how colors are approximated in character symbol output.
    pub fn get_color_extractor(&self) -> ColorExtractor {
        let ce: u32;

        unsafe {
            ce = ffi::chafa_canvas_config_get_color_extractor(self.raw);
        }
        let ext = match ce {
            ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_AVERAGE => ColorExtractor::Average,
            ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_MEDIAN => ColorExtractor::Median,
            ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_MAX => ColorExtractor::Max,
            _ => ColorExtractor::Average,
        };
        ext
    }

    /// Sets config 's stored ChafaColorExtractor to color_extractor . This determines how colors are approximated in character symbol output.
    pub fn set_color_extractor(&self, ext: ColorExtractor) {
        let ce: u32 = match ext {
            ColorExtractor::Average => ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_AVERAGE,
            ColorExtractor::Median => ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_MEDIAN,
            ColorExtractor::Max => ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_MAX,
        };
        unsafe {
            ffi::chafa_canvas_config_set_color_extractor(self.raw, ce);
        }
    }

    /// Returns config 's ChafaColorSpace.
    pub fn get_color_space(&self) -> ColorSpace {
        let cs: u32;

        unsafe {
            cs = ffi::chafa_canvas_config_get_color_space(self.raw);
        }
        let mode = match cs {
            ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_RGB => ColorSpace::RGB,
            ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_DIN99D => ColorSpace::DIN99d,
            ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_MAX => ColorSpace::Max,
            _ => ColorSpace::RGB,
        };
        mode
    }

    /// Sets config 's stored ChafaColorSpace to color_space .
    pub fn set_color_space(&self, space: ColorSpace) {
        let cs: u32 = match space {
            ColorSpace::RGB => ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_RGB,
            ColorSpace::DIN99d => ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_DIN99D,
            ColorSpace::Max => ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_MAX,
        };
        unsafe {
            ffi::chafa_canvas_config_set_color_space(self.raw, cs);
        }
    }

    /// Queries whether automatic image preprocessing is enabled. This allows Chafa to boost contrast and saturation in an attempt to improve legibility.
    /// The type of preprocessing applied (if any) depends on the canvas mode.
    pub fn get_preprocessing_enabled(&self) -> bool {
        let preprocess: i32;

        unsafe {
            preprocess = ffi::chafa_canvas_config_get_preprocessing_enabled(self.raw);
        }
        if preprocess == 0 { false } else { true }
    }

    /// Indicates whether automatic image preprocessing should be enabled. This allows Chafa to boost contrast and saturation in an attempt to improve legibility.
    /// The type of preprocessing applied (if any) depends on the canvas mode.
    pub fn set_preprocessing_enabled(&self, preprocess: bool) {
        unsafe {
            ffi::chafa_canvas_config_set_preprocessing_enabled(
                self.raw,
                if preprocess { 1 } else { 0 },
            );
        }
    }

    // /// Returns a pointer to the symbol map belonging to config .
    // /// This can be inspected using the ChafaSymbolMap getter functions, but not changed.
    // pub fn get_symbol_map(&self) -> SymbolMap {}
}

impl Drop for Config {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_canvas_config_unref(self.raw);
            }
        }
    }
}

#[repr(u32)]
pub enum PixelMode {
    /// Pixel data is approximated using character symbols ("ANSI art").
    Symbols = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SYMBOLS,
    /// Pixel data is encoded as sixels.
    Sixels = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SIXELS,
    /// Pixel data is encoded using the Kitty terminal protocol.
    Kitty = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_KITTY,
    /// Pixel data is encoded using the iTerm2 terminal protocol.
    Iterm2 = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_ITERM2,

    /// Last supported pixel mode plus one.
    Max = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_MAX,
}

#[repr(u32)]
pub enum CanvasMode {
    /// Truecolor.
    TrueColor = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_TRUECOLOR,
    /// 256 colors.
    Indexed256 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_256,
    /// 256 colors, but avoid using the lower 16 whose values vary between terminal environments.
    Indexed240 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_240,
    /// 16 colors using the aixterm ANSI extension.
    Indexed16 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16,
    /// Default foreground and background colors, plus inversion.
    FgbgBgfg = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG_BGFG,
    /// Default foreground and background colors. No ANSI codes will be used.
    FgBg = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG,
    /// 8 colors, compatible with original ANSI X3.64.
    Indexed8 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_8,
    /// 16 FG colors (8 of which enabled with bold/bright) and 8 BG colors.
    Indexed168 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16_8,

    /// Last supported canvas mode plus one.
    Max = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_MAX,
}

#[repr(u32)]
pub enum ColorExtractor {
    /// Use the average colors of each symbol's coverage area.
    Average = ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_AVERAGE,
    /// Use the median colors of each symbol's coverage area.
    Median = ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_MEDIAN,

    /// Last supported color extractor plus one.
    Max = ffi::ChafaColorExtractor_CHAFA_COLOR_EXTRACTOR_MAX,
}

#[repr(u32)]
pub enum ColorSpace {
    /// RGB color space. Fast but imprecise.
    RGB = ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_RGB,
    /// DIN99d color space. Slower, but good perceptual color precision.
    DIN99d = ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_DIN99D,

    /// Last supported color space plus one.
    Max = ffi::ChafaColorSpace_CHAFA_COLOR_SPACE_MAX,
}

#[repr(u32)]
pub enum DitherMode {
    /// No dithering.
    None = ffi::ChafaDitherMode_CHAFA_DITHER_MODE_NONE,
    /// Ordered dithering (Bayer or similar).
    Ordered = ffi::ChafaDitherMode_CHAFA_DITHER_MODE_ORDERED,
    /// Error diffusion dithering (Floyd-Steinberg or similar).
    Diffusion = ffi::ChafaDitherMode_CHAFA_DITHER_MODE_DIFFUSION,
    /// Noise pattern dithering (blue noise or similar).
    Noise = ffi::ChafaDitherMode_CHAFA_DITHER_MODE_NOISE,

    /// Last supported dither mode plus one.
    Max = ffi::ChafaDitherMode_CHAFA_DITHER_MODE_MAX,
}

#[repr(u32)]
pub enum Optimizations {
    /// Suppress redundant SGR control sequences.
    ReuseAttributes = ffi::ChafaOptimizations_CHAFA_OPTIMIZATION_REUSE_ATTRIBUTES,
    /// Reserved for future use.
    SkipCells = ffi::ChafaOptimizations_CHAFA_OPTIMIZATION_SKIP_CELLS,
    /// Use REP sequence to compress repeated runs of similar cells.
    RepeatCells = ffi::ChafaOptimizations_CHAFA_OPTIMIZATION_REPEAT_CELLS,

    /// All optimizations disabled.
    None = ffi::ChafaOptimizations_CHAFA_OPTIMIZATION_NONE,
    /// All optimizations enabled.
    All = ffi::ChafaOptimizations_CHAFA_OPTIMIZATION_ALL,
}

#[repr(u32)]
pub enum Passthrough {
    /// No passthrough guards will be used.
    None = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_NONE,
    /// Passthrough guards for GNU Screen will be used.
    Screen = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_SCREEN,
    /// Passthrough guards for tmux will be used.
    Tmux = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_TMUX,

    /// Last supported passthrough mode plus one.
    Max = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_MAX,
}
