use std::sync::Arc;

use async_trait::async_trait;
use datafusion::{
    error::Result,
    execution::context::{QueryPlanner, SessionState},
    logical_expr::LogicalPlan,
    physical_plan::ExecutionPlan,
};

pub struct SqlQueryPlanner {}

#[async_trait]
impl QueryPlanner for SqlQueryPlanner {
    async fn create_physical_plan(&self,
                                  logical_plan: &LogicalPlan,
                                  session_state: &SessionState)
                                  -> Result<Arc<dyn ExecutionPlan>> {
        let physical_planner = SqlQueryPlanner {};
        physical_planner.create_physical_plan(logical_plan, session_state).await
    }
}
