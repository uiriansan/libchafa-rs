/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaImage.html
 */

use crate::ffi;

/// A ChafaImage represents a raster image for placement on a ChafaCanvas. It can currently hold a single ChafaFrame.
///
/// To place an image on a canvas, it must first be assigned to a ChafaPlacement.
pub struct Image {
    pub raw: *mut ffi::ChafaImage,
}

impl Image {
    /// Creates a new ChafaImage. The image is initially transparent and dimensionless.
    pub fn new() -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_image_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create Image")
        } else {
            Ok(Image { raw })
        }
    }

    /// Assigns frame as the content for image . The image will keep its own reference to the frame.
    pub fn set_frame(&self, frame: crate::Frame) {
        unsafe {
            ffi::chafa_image_set_frame(self.raw, frame.raw);
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_image_unref(self.raw);
            }
        }
    }
}
