use sqlx::Postgres;

use crate::arguments::postgres::PgArguments;
use crate::database::AboutDatabase;
use crate::query_builder::TargetQuery;

impl<'a> AboutDatabase<'a, Postgres, PgArguments> for Postgres {

    fn arguments() -> PgArguments {
        PgArguments::new()
    }

    fn target() -> TargetQuery {
        TargetQuery::Postgres
    }
}