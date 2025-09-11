/*
 * https://hpjansson.org/chafa/ref/chafa-ChafaFrame.html
 */

use crate::{ffi, misc};

/// A ChafaFrame contains the specific of a single frame of image data. It can be added to a ChafaImage.
pub struct Frame {
    pub raw: *mut ffi::ChafaFrame,
}

impl Frame {
    /// Creates a new ChafaFrame containing a copy of the image data pointed to by data .
    /// # Parameters:
    /// --- `pixel_type`: The ChafaPixelType of the source data;
    /// --- `data`: Image data buffer to copy from;
    /// --- `width`: Width of the image, in pixels;
    /// --- `height`: Height of the image, in pixels;
    /// --- `rowstride`: Number of bytes to advance from the start of one row to the next.
    pub fn new(
        data: &[u8],
        pixel_type: misc::PixelType,
        width: i32,
        height: i32,
        rowstride: i32,
    ) -> Result<Self, &'static str> {
        let raw = unsafe {
            ffi::chafa_frame_new(
                data.as_ptr() as *const _,
                pixel_type as u32,
                width,
                height,
                rowstride,
            )
        };
        if raw.is_null() {
            Err("Chafa -> Failed to create Frame")
        } else {
            Ok(Self { raw })
        }
    }

    /// Creates a new ChafaFrame, which takes ownership of the data buffer. The buffer will be freed with g_free() when the frame's reference count drops to zero.
    /// # Parameters:
    /// --- `pixel_type`: The ChafaPixelType of the buffer;
    /// --- `data`: Pointer to an image data buffer to assign;
    /// --- `width`: Width of the image, in pixels;
    /// --- `height`: Height of the image, in pixels;
    /// --- `rowstride`: Number of bytes to advance from the start of one row to the next.
    pub fn new_steal(
        data: *mut u8,
        pixel_type: misc::PixelType,
        width: i32,
        height: i32,
        rowstride: i32,
    ) -> Result<Self, &'static str> {
        let raw = unsafe {
            ffi::chafa_frame_new_steal(data as *mut _, pixel_type as u32, width, height, rowstride)
        };
        if raw.is_null() {
            Err("Chafa -> Failed to create Frame")
        } else {
            Ok(Self { raw })
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_frame_unref(self.raw);
            }
        }
    }
}
