pub mod vm_status {
    use crate::core::path::path::path::Path;
    use crate::core::path::slot::slot::Slot;

    type Stack = Vec<(Path, i32, Option<i32>)>;

    #[derive(Debug, Clone)]
    pub struct VMStatus {
        pub path:Path,
        pub index: i32,
        pub neighbour: Option<i32>,
        pub stack: Stack,
    }
}

use crate::core::path::path::path::Path;
use crate::core::path::slot::slot::Slot;
use crate::core::vm::vm_status::vm_status::VMStatus;


impl VMStatus {
    fn path(&self) -> &Path {
        &self.path
    }

    fn index(&self) -> i32 {
        self.index
    }

    fn neighbour(&self) -> Option<i32> {
        self.neighbour
    }

    fn is_folding(&self) -> bool {
        self.neighbour.is_some()
    }

    fn fold_into(&self, id: Option<i32>) -> Self {
        Self {
            neighbour: id,
            ..self.clone()
        }
    }

    fn foldout(&self) -> Self {
        Self {
            neighbour: None,
            ..self.clone()
        }
    }

    fn nest(&self, slot: Slot) -> Self {
        Self {
            path: self.path.push(slot),
            ..self.clone()
        }
    }

    fn inc_index(&self) -> Self {
        Self {
            index: self.index + 1,
            ..self.clone()
        }
    }

    fn push(&self) -> Self {
        Self {
            stack: [&self.stack[..], &[(self.path.clone(), self.index, self.neighbour)]].concat(),
            ..self.clone()
        }
    }

    fn pop(&self) -> Self {
        Self {
            stack: self.stack[..self.stack.len() - 1].to_vec(),
            ..self.clone()
        }
    }
}