use act::{Action, ActionDisplay};

#[derive(Debug, Clone, Action)]
enum FrogAction {
    #[act(action(human_name = "jump"))]
    Jump,
    CroaCroa,
    UseTongue,
}

#[derive(Debug, Clone, ActionDisplay)]
enum DuckAction {
    #[act(pouet)]
    Walk,
    Swim,
    Fly,
    SayHiToTheCrowFriend,
}

#[derive(Debug, Clone, ActionDisplay)]
enum CrowAction {
    #[act(action(unknown = "oui"))]
    CroaCroa,
    Fly,
    SpeakFluentIntelligence,
    LandOnTree,
}

#[derive(Debug, Clone, ActionDisplay)]
enum TreeAction {
    #[act(action(human_name = 5))]
    Grow,
    MakeLeaves,
}

#[derive(Debug, Clone, ActionDisplay)]
enum LeavesAction {
    #[act(action(human_name))]
    Fall,
    Float,
}

#[derive(Debug, Clone, ActionDisplay)]
enum WaterAction {
    #[act(action(human_name = "pouet", human_name = "oui"))]
    Ripple,
    MakeBubbles,
}

fn main() {}
