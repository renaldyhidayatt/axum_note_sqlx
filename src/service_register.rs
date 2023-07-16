use std::sync::Arc;

use crate::{
    abstract_trait::{DynNoteRepository, DynNoteService},
    connection_pool::ConnectionPool,
    repository::NoteRepository,
    service::NoteService,
};

#[derive(Clone)]
pub struct ServiceRegister {
    pub note_service: DynNoteService,
}

impl ServiceRegister {
    pub fn new(pool: ConnectionPool) -> Self {
        let note_repository = Arc::new(NoteRepository::new(pool.clone())) as DynNoteRepository;
        let note_service = Arc::new(NoteService::new(note_repository)) as DynNoteService;

        ServiceRegister { note_service }
    }
}
