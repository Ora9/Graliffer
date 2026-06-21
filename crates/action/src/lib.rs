use std::{
    any::Any,
    cell::RefCell,
    fmt::Debug,
    ops::{Add, Deref},
    rc::Rc,
};

pub trait State: Debug {
    type Action: Action;
    type Error;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error>;
}

pub trait Action: Any + ActionClone + Debug {}

pub trait ActionClone {
    fn dyn_clone(&self) -> Box<dyn Action>;
}

impl<T: Clone + Action> ActionClone for T {
    fn dyn_clone(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

impl Action for Box<dyn Action> {}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}

#[derive(Debug, Clone)]
pub struct AnyAction(Box<dyn Action>);

impl Action for AnyAction {}

impl Deref for AnyAction {
    type Target = Box<dyn Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AnyAction {
    pub fn new(action: impl Action) -> Self {
        Self(Box::new(action))
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (self.0.deref() as &dyn Any).downcast_ref()
    }
}

#[derive(Debug)]
pub enum Revert {
    Action(Apply),
    None,
}

impl Revert {
    pub fn new(action: impl Action) -> Self {
        Self::Action(Apply::new(action))
    }
}

#[derive(Debug)]
pub struct Apply(Box<dyn Action>);

impl Apply {
    pub fn new(action: impl Action) -> Self {
        Self(Box::new(action))
    }
}

#[derive(Debug)]
pub struct Undoable {
    apply: Apply,
    revert: Revert,
}

pub enum TimelineError<E> {
    ActionError(E),
    NothingToUndo,
}

#[derive(Debug)]
pub struct Timeline<S>
where
    S: State,
{
    state: Rc<RefCell<S>>,
    timeline: Vec<Undoable>,
    cursor: usize,
}

impl<S: State> Timeline<S> {
    pub fn new(state: Rc<RefCell<S>>) -> Self {
        Self {
            state,
            timeline: Vec::new(),
            cursor: 0,
        }
    }

    pub fn act(&mut self, action: S::Action) -> Result<(), TimelineError<S::Error>> {
        let res = {
            let mut state = self.state.try_borrow_mut().unwrap();
            state.act(&action)
        };

        match res {
            Ok(revert) => {
                self.append(Undoable {
                    apply: Apply::new(action),
                    revert,
                });

                Ok(())
            }
            Err(err) => Err(TimelineError::ActionError(err)),
        }
    }

    fn append(&mut self, undoable: Undoable) {
        self.timeline.truncate(self.cursor);
        self.timeline.push(undoable);
        self.cursor = self.cursor.add(1);

        dbg!(self);
    }
}
