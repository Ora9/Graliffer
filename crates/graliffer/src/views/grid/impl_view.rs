use crate::{AppAction, GridAction, GridView, View, ViewId};

impl View for GridView {
    fn view_id() -> ViewId {
        ViewId::Grid
    }

    fn input_sink_action(input: String) -> Option<AppAction> {
        Some(AppAction::GridAction(GridAction::InsertOverflow(input)))
    }
}
