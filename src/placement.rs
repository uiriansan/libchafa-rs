/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaPlacement.html
 */

use crate::ffi;

pub struct Placement {
    pub raw: *mut ffi::ChafaPlacement,
}

impl Placement {
    /// Creates a new ChafaPlacement.
    /// # Parameters:
    /// --- `id`: An ID to assign to the placement, or <= 0 to assign one automatically.
    pub fn new(image: crate::Image, id: i32) -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_placement_new(image.raw, id) };
        if raw.is_null() {
            Err("Chafa -> Failed to create Placement")
        } else {
            Ok(Self { raw })
        }
    }

    /// Gets the tucking policy of placement. This describes how the image is resized to fit placement 's extents, and defaults to CHAFA_TUCK_STRETCH.
    pub fn get_tuck(&self) -> Tuck {
        unsafe { Tuck::from(ffi::chafa_placement_get_tuck(self.raw)) }
    }

    /// Sets the tucking policy for placement to tuck . This describes how the image is resized to fit placement 's extents, and defaults to CHAFA_TUCK_STRETCH.
    pub fn set_tuck(&self, tuck: Tuck) {
        unsafe {
            ffi::chafa_placement_set_tuck(self.raw, tuck as u32);
        }
    }

    /// Gets the horizontal alignment of placement . This determines how any padding added by the tucking policy is distributed, and defaults to CHAFA_ALIGN_START.
    pub fn get_halign(&self) -> Align {
        unsafe { Align::from(ffi::chafa_placement_get_halign(self.raw)) }
    }

    /// Sets the horizontal alignment of placement . This determines how any padding added by the tucking policy is distributed, and defaults to CHAFA_ALIGN_START
    pub fn set_halign(&self, align: Align) {
        unsafe {
            ffi::chafa_placement_set_halign(self.raw, align as u32);
        }
    }

    /// Gets the vertical alignment of placement . This determines how any padding added by the tucking policy is distributed, and defaults to CHAFA_ALIGN_START.
    pub fn get_valign(&self) -> Align {
        unsafe { Align::from(ffi::chafa_placement_get_valign(self.raw)) }
    }

    /// Sets the vertical alignment of placement . This determines how any padding added by the tucking policy is distributed.
    pub fn set_valign(&self, align: Align) {
        unsafe {
            ffi::chafa_placement_set_valign(self.raw, align as u32);
        }
    }
}

impl Drop for Placement {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_placement_unref(self.raw);
            }
        }
    }
}

#[repr(u32)]
pub enum Tuck {
    /// Resize element to fit the area exactly, changing its aspect ratio.
    Stretch = ffi::ChafaTuck_CHAFA_TUCK_STRETCH,
    /// Resize element to fit the area, preserving its aspect ratio by adding padding.
    Fit = ffi::ChafaTuck_CHAFA_TUCK_FIT,
    /// Like CHAFA_TUCK_FIT , but prohibit enlargement.
    ShrinkToFit = ffi::ChafaTuck_CHAFA_TUCK_SHRINK_TO_FIT,

    /// Last supported tucking policy, plus one.
    Max = ffi::ChafaTuck_CHAFA_TUCK_MAX,
}
impl Into<u32> for Tuck {
    fn into(self) -> u32 {
        match self {
            Tuck::Stretch => ffi::ChafaTuck_CHAFA_TUCK_STRETCH,
            Tuck::Fit => ffi::ChafaTuck_CHAFA_TUCK_FIT,
            Tuck::ShrinkToFit => ffi::ChafaTuck_CHAFA_TUCK_SHRINK_TO_FIT,
            Tuck::Max => ffi::ChafaTuck_CHAFA_TUCK_MAX,
        }
    }
}
impl From<u32> for Tuck {
    fn from(value: u32) -> Self {
        match value {
            ffi::ChafaTuck_CHAFA_TUCK_STRETCH => Tuck::Stretch,
            ffi::ChafaTuck_CHAFA_TUCK_FIT => Tuck::Fit,
            ffi::ChafaTuck_CHAFA_TUCK_SHRINK_TO_FIT => Tuck::ShrinkToFit,
            ffi::ChafaTuck_CHAFA_TUCK_MAX => Tuck::Max,
            _ => Tuck::Stretch,
        }
    }
}

#[repr(u32)]
pub enum Align {
    /// Align flush with beginning of the area (top or left in LTR locales).
    Start = ffi::ChafaAlign_CHAFA_ALIGN_START,
    /// Align flush with end of the area (bottom or right in LTR locales).
    End = ffi::ChafaAlign_CHAFA_ALIGN_END,
    /// Align in the middle of the area.
    Center = ffi::ChafaAlign_CHAFA_ALIGN_CENTER,

    /// Last supported alignment, plus one.
    Max = ffi::ChafaAlign_CHAFA_ALIGN_MAX,
}
impl Into<u32> for Align {
    fn into(self) -> u32 {
        match self {
            Align::Start => ffi::ChafaAlign_CHAFA_ALIGN_START,
            Align::End => ffi::ChafaAlign_CHAFA_ALIGN_END,
            Align::Center => ffi::ChafaAlign_CHAFA_ALIGN_CENTER,
            Align::Max => ffi::ChafaAlign_CHAFA_ALIGN_MAX,
        }
    }
}
impl From<u32> for Align {
    fn from(value: u32) -> Self {
        match value {
            ffi::ChafaAlign_CHAFA_ALIGN_START => Align::Start,
            ffi::ChafaAlign_CHAFA_ALIGN_END => Align::End,
            ffi::ChafaAlign_CHAFA_ALIGN_CENTER => Align::Center,
            ffi::ChafaAlign_CHAFA_ALIGN_MAX => Align::Max,
            _ => Align::Start,
        }
    }
}
