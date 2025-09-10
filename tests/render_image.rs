#[cfg(test)]
mod tests {

    #[test]
    fn render_image() {
        use libchafa::canvas::ChafaCanvas;

        config = ChafaCanvasConfig::new();
        config.set_canvas_mode();
        config.set_pixel_mode();
        config.set_geometry();
        config.set_passthrough();

        canvas = ChafaCanvas::new(config);
    }
}
