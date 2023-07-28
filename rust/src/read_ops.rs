use automerge::{
    transaction::{Transactable, Transaction},
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

impl<'a> ReadOps for Transaction<'a> {
    fn heads(&self) -> Vec<automerge::ChangeHash> {
        Transaction::base_heads(self)
    }
}
