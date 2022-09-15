use crate::{
    cores::error::Error,
    services::state::{
        model::{entity::State, request::StateCreateRequest},
        repo::db::DbRepo,
    },
    utils::common::FieldValidation,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, req: &StateCreateRequest) -> Result<State, Error> {
    tracing::debug!("executing ...");
    validate(&req)?;
    repo.insert(req).await
}

fn validate(req: &StateCreateRequest) -> Result<(), Error> {
    let mut validation = FieldValidation::new();
    if req.code == "" {
        validation.add("Code is empty");
    }

    validation.check()
}
