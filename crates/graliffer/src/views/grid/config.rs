#[derive(Debug, Default, Clone, Copy)]
pub enum FollowCursorMode {
    #[default]
    Sticky,
    Centered,
}

#[derive(Debug, Clone, Copy)]
pub struct FollowCursorConfig {
    pub follow_cursor_mode: FollowCursorMode,
    pub follow_cursor_sticky_margin: u32,
}

impl Default for FollowCursorConfig {
    fn default() -> Self {
        Self {
            follow_cursor_mode: FollowCursorMode::default(),
            follow_cursor_sticky_margin: 2,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum GutterSizeConfig {
    #[default]
    Proportional,
    Minimal,
}

#[derive(Debug, Clone, Copy)]
pub struct GutterConfig {
    pub show: bool,
    pub size: GutterSizeConfig,
}

impl Default for GutterConfig {
    fn default() -> Self {
        Self {
            show: true,
            size: GutterSizeConfig::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GridConfig {
    pub follow_cursor: FollowCursorConfig,
    pub gutter: GutterConfig,
}
