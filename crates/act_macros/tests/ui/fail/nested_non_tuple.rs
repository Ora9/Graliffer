#[derive(Debug, Clone, act::ActionDisplay)]
enum Unit {
    #[act(nested(namespace = "unit"))]
    Unit,
}

#[derive(Debug, Clone, act::ActionDisplay)]
enum Tuple2Size {
    #[act(nested(namespace = "tuple 2 size"))]
    Tuple2Size(usize, bool),
}

#[derive(Debug, Clone, act::ActionDisplay)]
enum Struct {
    #[act(nested(namespace = "struct"))]
    Struct { a: usize, b: bool },
}

fn main() {}
