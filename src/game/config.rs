pub fn get_display() -> amethyst::renderer::DisplayConfig {
    amethyst::renderer::DisplayConfig {
        title: "VN".to_owned(),
        dimensions: Some((1024, 768)),
        max_dimensions: None,
        min_dimensions: None,
        fullscreen: false,
        multisampling: 1,
        visibility: true,
        vsync: true,
    }
}
