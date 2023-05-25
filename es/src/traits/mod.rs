use anyhow::Result;
use uuid::Uuid;

pub trait Command<R, E> {
    fn idempotence_id(&self) -> Uuid;
}

pub trait BoundedContext {
    fn execute<C, R, E>(command: C) -> Result<R, E>
    where
        C: Command<R, E>;
}
