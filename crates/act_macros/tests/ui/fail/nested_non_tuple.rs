#[derive(Debug, Clone, act::ActionDisplay)]
enum Unit {
    #[act(nested)]
    Unit,
}

#[derive(Debug, Clone, act::ActionDisplay)]
enum Struct {
    #[act(nested)]
    Struct { a: usize, b: bool },
}

fn main() {}
