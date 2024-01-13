use migration::{Migrator, MigratorTrait};
use models::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseConnection,
    DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set, TransactionTrait, FromQueryResult, prelude::BigDecimal,
};
use teloxide::types::{Chat, ChatId};
use wd_log::{log_debug_ln, log_error_ln, log_info_ln, log_warn_ln};


#[derive(Debug, FromQueryResult)]
pub struct TopData {
    pub name: String,
    pub counts: BigDecimal,
}


#[derive(Debug, Clone)]
pub struct Controller {
    db: DatabaseConnection,
}

impl Controller {
    /// Create controller
    pub async fn new(config: String) -> Result<Self, DbErr> {
        Ok(Self {
            db: Database::connect(config).await?,
        })
    }

    /// Do migrate
    pub async fn migrate(&self) -> Result<(), DbErr> {
        if let Err(err) = Migrator::install(&self.db).await {
            log_warn_ln!("{}", err)
        }
        if let Err(err) = Migrator::up(&self.db, None).await {
            Err(err)
        } else {
            log_info_ln!("database initialized.");
            Ok(())
        }
    }

    /// hang someone
    pub async fn hangit(&self, name: &String, group_id: ChatId) -> Result<(), DbErr> {
        let transcation = self.db.begin().await?;
        match Stats::find()
            .filter(StatsColumn::GroupId.eq(group_id.0))
            .filter(StatsColumn::Name.eq(name))
            .one(&transcation)
            .await?
        {
            Some(one) => {
                let counts = one.counts;
                let mut one_active: StatsActiveModel = one.into();
                one_active.counts = Set(counts + 1);
                one_active.save(&transcation).await?;
            }
            None => {
                StatsActiveModel {
                    group_id: Set(group_id.0),
                    name: Set(name.to_string()),
                    counts: Set(1),
                    ..Default::default()
                }
                .save(&transcation)
                .await?;
            }
        }
        transcation.commit().await
    }

    /// stats
    pub async fn top(&self, chat: &Chat) -> Option<Vec<TopData>> {
        const LIMIT: u64 = 10;
        let transcation = match self.db.begin().await {
            Ok(t) => t,
            Err(error) => {
                log_error_ln!("{}", error);
                return None;
            }
        };

        let query = match chat.is_group() || chat.is_supergroup() {
            true => {
                Stats::find().select_only()
                    .column(StatsColumn::Name)
                    .column_as(StatsColumn::Counts.sum(), "counts")
                    .filter(StatsColumn::GroupId.eq(chat.id.0))
                    .group_by(StatsColumn::Name)
                    .order_by_desc(StatsColumn::Counts.sum())
                    .limit(LIMIT).into_model()
                    .all(&transcation)
                    .await
            }
            false => {
                let query = Stats::find()
                    .select_only()
                    .column(StatsColumn::Name)
                    .column_as(StatsColumn::Counts.sum(), "counts")
                    .group_by(StatsColumn::Name)
                    .order_by_desc(StatsColumn::Counts.sum())
                    .limit(LIMIT);

                log_debug_ln!(
                    "SQL: {:?}",
                    query.build(transcation.get_database_backend()).to_string()
                );

                query.into_model().all(&transcation).await
            }
        };

        match query {
            Ok(query) => Some(query),
            Err(error) => {
                log_error_ln!("{}", error);
                None
            }
        }
    }
}
