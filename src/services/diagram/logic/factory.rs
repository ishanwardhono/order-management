use super::{delete, get, insert, valid_transition};
use crate::{
    cores::error::service::Error,
    services::{
        diagram::{model::model::Diagram, repo::db::DbRepo},
        state::logic::factory as StateFactory,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct LogicFactory {
    repo: Arc<dyn DbRepo>,
    state_factory: Arc<dyn StateFactory::Logic>,
}

impl LogicFactory {
    pub fn new(
        repo: Arc<dyn DbRepo>,
        state_factory: Arc<dyn StateFactory::Logic>,
    ) -> Arc<dyn Logic> {
        Arc::new(Self {
            repo,
            state_factory,
        })
    }
}

#[async_trait]
pub trait Logic {
    async fn insert(&self, req: &Diagram, actor: &Uuid) -> Result<(), Error>;
    async fn get(&self, code: &String) -> Result<Diagram, Error>;
    async fn delete(&self, code: &String) -> Result<(), Error>;
    async fn valid_transition(
        &self,
        code: &String,
        from: &String,
        to: &String,
    ) -> Result<(), Error>;
}

#[async_trait]
impl Logic for LogicFactory {
    async fn insert(&self, req: &Diagram, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Logic Execute - Insert Diagram");
        insert::execute(self.repo.clone(), self.state_factory.clone(), req, actor).await
    }

    async fn get(&self, code: &String) -> Result<Diagram, Error> {
        tracing::info!("Logic Execute - Get Diagram");
        get::execute(self.repo.clone(), code).await
    }

    async fn delete(&self, code: &String) -> Result<(), Error> {
        tracing::info!("Logic Execute - Delete Diagram");
        delete::execute(self.repo.clone(), code).await
    }

    async fn valid_transition(
        &self,
        code: &String,
        from: &String,
        to: &String,
    ) -> Result<(), Error> {
        tracing::info!("Logic Execute - Valid Transition in Diagram");
        valid_transition::execute(self.repo.clone(), code, from, to).await
    }
}
