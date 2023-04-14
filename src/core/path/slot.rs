pub mod slot {
    /// A Slot can either be Nbr, Rep or Branch
    #[derive(PartialEq, Debug)]
    pub enum Slot {
        Nbr(i32),
        Rep(i32),
        Branch(i32),
    }
}

use std::fmt::Display;
use crate::core::path::slot::slot::Slot;

impl Slot {
    pub fn to_str(&self) -> String {
        match self {
            Slot::Nbr(nbr) => "Nbr(".to_owned()+&nbr.to_string()+")",
            Slot::Rep(rep) => "Rep(".to_owned()+&rep.to_string()+")",
            Slot::Branch(branch) => "Branch(".to_owned()+&branch.to_string()+")",
        }
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::path::slot::slot::Slot;

    #[test]
    fn test_slot_creation() {
        let nbr = Slot::Nbr(0);
        assert_eq!(nbr,Slot::Nbr(0))
    }

    #[test]
    fn test_slot_to_str() {
        let slot = Slot::Nbr(0);
        assert_eq!(slot.to_str(), "Nbr(0)")
    }
}