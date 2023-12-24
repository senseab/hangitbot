use migration::{Migrator, MigratorTrait};
use models::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use teloxide::types::ChatId;
use wd_log::{log_info_ln, log_warn_ln};

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
    pub async fn top(&self, group_id: ChatId) -> Result<Option<Vec<StatsModel>>, DbErr> {
        const LIMIT: u64 = 10;
        let transcation = self.db.begin().await?;

        let query = match group_id.is_group() {
            true => {
                Stats::find()
                    .filter(StatsColumn::GroupId.eq(group_id.0))
                    .order_by_desc(StatsColumn::Counts)
                    .paginate(&transcation, LIMIT)
                    .fetch()
                    .await?
            }
            false => {
                Stats::find()
                    .order_by_desc(StatsColumn::Counts)
                    .paginate(&transcation, LIMIT)
                    .fetch()
                    .await?
            }
        };

        Ok(Some(query))
    }
}
