use std::{collections::HashMap, fmt, result, sync::Arc};

use datafusion::{
    catalog::catalog::CatalogProvider,
    common::DataFusionError,
    execution::{context::SessionState, runtime_env::RuntimeEnv},
    physical_plan::ExecutionPlan,
    prelude::{SessionConfig, SessionContext},
};

use crate::{executor::DedicatedExecutor, sql::SqlQueryPlanner};

pub type Result<T> = result::Result<T, DataFusionError>;

#[derive(Clone)]
pub struct IsiphoSessionCfg {
    exec: DedicatedExecutor,
    session_config: SessionConfig,
    runtime: Arc<RuntimeEnv>,
    catalog: Option<Arc<dyn CatalogProvider>>,
}
const SIZE: usize = 1000;
// The default catalog name - this impacts what SQL queries use if not specified
pub const DEFAULT_CATALOG: &str = "public";
// The default schema name - this impacts what SQL queries use if not specified
pub const DEFAULT_SCHEMA: &str = "cnosdb";

impl IsiphoSessionCfg {
    pub(super) fn new(exec: DedicatedExecutor, runtime: Arc<RuntimeEnv>) -> Self {
        let session_config =
            SessionConfig::new().with_batch_size(SIZE)
                                .create_default_catalog_and_schema(true)
                                .with_information_schema(true)
                                .with_default_catalog_and_schema(DEFAULT_CATALOG, DEFAULT_SCHEMA);

        Self { exec, session_config, runtime, catalog: None }
    }

    pub fn with_target_partitions(mut self, target_partitions: usize) -> Self {
        self.session_config = self.session_config.with_target_partitions(target_partitions);
        self
    }

    pub fn with_default_catalog(self, catalog: Arc<dyn CatalogProvider>) -> Self {
        Self { catalog: Some(catalog), ..self }
    }

    pub fn build(self) -> IsiophoSessionCtx {
        let state = SessionState::with_config_rt(self.session_config, self.runtime)
            .with_query_planner(Arc::new(SqlQueryPlanner {}));

        let inner = SessionContext::with_state(state);

        if let Some(default_catalog) = self.catalog {
            inner.register_catalog(DEFAULT_CATALOG, default_catalog);
        }

        IsiophoSessionCtx { inner, exec: Some(self.exec) }
    }
}

#[derive(Default)]
pub struct IsiophoSessionCtx {
    inner: SessionContext,
    exec: Option<DedicatedExecutor>,
}

impl fmt::Debug for IsiophoSessionCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IsiophoSessionCtx")
         .field("inner", &"<DataFusion ExecutionContext>")
         .finish()
    }
}

impl IsiophoSessionCtx {
    pub fn inner(&self) -> &SessionContext {
        &self.inner
    }

    pub async fn run<Fut, T>(&self, fut: Fut) -> Result<T>
        where Fut: std::future::Future<Output = Result<T>> + Send + 'static,
              T: Send + 'static
    {
        match &self.exec {
            Some(exec) => {
                exec.spawn(fut)
                    .await
                    .unwrap_or_else(|e| {
                        Err(DataFusionError::Execution(format!("Join Error: {}", e)))
                    })
            },
            None => unimplemented!("spawn onto current threadpool"),
        }
    }
}
