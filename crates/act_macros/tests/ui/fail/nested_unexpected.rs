#[derive(Debug, Clone)]
enum Nest {
    X,
    Y,
}

#[derive(Debug, Clone, act::ActionDisplay)]
enum Default {
    #[act(nested)]
    A(Nest),
}

#[derive(Debug, Clone, act::ActionDisplay)]
enum Explicit {
    #[act(nested = "a")]
    A(Nest),
}

fn main() {

    // DefaultNamespace::A
}
