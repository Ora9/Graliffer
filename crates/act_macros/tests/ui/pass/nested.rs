#[derive(Debug, Clone)]
enum Nest {
    A,
    B,
}

#[derive(Debug, Clone, act::ActionDisplay)]
enum Tuple {
    #[act(nested(namespace = "pouet"))]
    Tuple(Nest),
}

fn main() {}
