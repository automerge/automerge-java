use automerge::{
    transaction::{Observation, Transactable, Transaction},
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

impl<'a, Obs: Observation> ReadOps for Transaction<'a, Obs> {
    fn heads(&self) -> Vec<automerge::ChangeHash> {
        Transaction::base_heads(self)
    }
}
