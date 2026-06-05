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

    // fn pop_err(&mut self) -> Result<Operand, anyhow::Error> {
    //     self.data.pop().ok_or(anyhow::anyhow!("Could not pop an element from the stack"))
    // }

    pub fn get_last(&self) -> Option<&Operand> {
        self.0.last()
    }

    // pub fn get_last_err(&self) -> Result<&Operand, anyhow::Error> {
    //     self.data.last().ok_or(anyhow::anyhow!("Could not pop an element from the stack"))
    // }

    // pub fn iter(&self) -> Iter<'_, Operand> {
    //     self.0.iter()
    // }
}
