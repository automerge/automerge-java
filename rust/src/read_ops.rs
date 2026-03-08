use automerge::{
    transaction::{OwnedTransaction, Transactable},
    Automerge, ReadDoc,
};

pub(crate) trait ReadOps: ReadDoc {
    fn heads(&self) -> Vec<automerge::ChangeHash>;
}

impl ReadOps for Automerge {
    fn heads(&self) -> Vec<automerge::ChangeHash> {
        Automerge::get_heads(self)
    }
}

impl ReadOps for OwnedTransaction {
    fn heads(&self) -> Vec<automerge::ChangeHash> {
        OwnedTransaction::base_heads(self)
    }
}
