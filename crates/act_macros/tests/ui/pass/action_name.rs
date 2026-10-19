use act::{Action, ActionDisplay};

#[derive(Debug, Clone, Action, ActionDisplay)]
enum FrogAction {
    #[act(action(human_name = "jump high!", machine_name = "JumpHigh"))]
    Jump,
    CroaCroa,
    UseTongue,
}

fn main() {
    assert_eq!(FrogAction::Jump.human_name(), "jump high!");
    assert_eq!(FrogAction::Jump.machine_name(), "JumpHigh");
}
