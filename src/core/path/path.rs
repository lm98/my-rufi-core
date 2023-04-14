use crate::core::path::path::path::Path;
use crate::core::path::slot::slot::Slot;

pub mod path {
    use crate::core::path::slot::slot::Slot;

    /// A Path is a collection of Slots that behave like a stack
    #[derive(PartialEq, Debug)]
    pub struct Path {
        pub slots: Vec<Slot>,
    }
}

impl Path {
    pub fn new() -> Self {
        Path { slots: Vec::new() }
    }

    pub fn is_root(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn head(&self) -> Option<&Slot> {
        self.slots.get(0)
    }

    pub fn tail (&self) -> Option<&Slot> {
        let last = self.slots.len() - 1;
        self.slots.get(last)
    }

    pub fn push(&mut self, slot: Slot) {
        self.slots.push(slot);
    }

    pub fn pop(&mut self) -> Option<Slot> {
        self.slots.pop()
    }

    pub fn pull(&mut self) -> Option<Slot> {
        if self.slots.is_empty() {
            None
        } else {
            Some(self.slots.remove(self.slots.len() - 1))
        }
    }

    pub fn reverse(&mut self) {
        self.slots.reverse();
    }

    pub fn to_str(&self) -> String {
        let result = &self.slots;
        "P://".to_owned() +
            &result.into_iter().map(|slot| slot.to_str()).collect::<Vec<String>>().join("/")
    }
}

#[cfg(test)]
mod tests {
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot::{Nbr, Rep, Branch};

    #[test]
    fn test_is_root() {
        let path = Path::new();
        assert!(path.is_root())
    }

    #[test]
    fn test_to_str() {
        let mut path = Path::new();
        path.push(Rep(0));
        path.push(Nbr(0));
        path.push(Nbr(1));
        path.push(Branch(0));
        assert_eq!(path.to_str(), "P://Rep(0)/Nbr(0)/Nbr(1)/Branch(0)")
    }
}