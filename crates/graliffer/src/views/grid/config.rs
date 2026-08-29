#[derive(Debug, Clone, Copy)]
pub struct FollowCursorStickyMargin {
    pub margin: u32,
}

impl Default for FollowCursorStickyMargin {
    fn default() -> Self {
        Self { margin: 5 }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum FollowCursorMode {
    Centered,
    #[default]
    Sticky,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FollowCursorConfig {
    pub follow_cursor_mode: FollowCursorMode,
    pub follow_cursor_sticky_margin: FollowCursorStickyMargin,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GridConfig {
    pub follow_cursor: FollowCursorConfig,
}
