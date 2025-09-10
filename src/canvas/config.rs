use crate::ffi;

pub struct Config {
    pub raw: *mut ffi::ChafaCanvasConfig,
}

impl Config {}

impl Drop for Config {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_canvas_config_unref(self.raw);
            }
        }
    }
}
