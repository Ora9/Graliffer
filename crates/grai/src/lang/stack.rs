use crate::Operand;

#[derive(Debug, Default)]
pub struct Stack(Vec<Operand>);

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, operand: Operand) {
        self.0.push(operand);
    }

    pub fn pop(&mut self) -> Option<Operand> {
        self.0.pop()
    }

    pub fn get_last(&self) -> Option<&Operand> {
        self.0.last()
    }

    // pub fn iter(&self) -> Iter<'_, Operand> {
    //     self.0.iter()
    // }
}
