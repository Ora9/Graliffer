use act::ActionDisplay;

#[derive(Debug, Clone, ActionDisplay)]
enum WaterAction {
    #[act(action(human_name = "pouet", human_name = "oui"))]
    Ripple,
    MakeBubbles,
}

fn main() {}
