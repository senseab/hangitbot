use migration::{Migrator, MigratorTrait};
use models::prelude::*;
use sea_orm::{
    prelude::BigDecimal, ActiveModelTrait, ColumnTrait, ConnectionTrait, Database,
    DatabaseConnection, DbErr, EntityTrait, FromQueryResult, QueryFilter, QueryOrder, QuerySelect,
    QueryTrait, Set, TransactionTrait,
};
use teloxide::types::{Chat, ChatId};
use wd_log::{log_debug_ln, log_error_ln, log_info_ln};

const LIMIT: u64 = 10;

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
            return Err(err);
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

    pub async fn update_group(&self, name: String, group_id: ChatId) -> Result<(), DbErr> {
        log_debug_ln!("name={:?}", name);

        let transcation = self.db.begin().await?;
        match Stats::find().filter(StatsColumn::GroupId.eq(0)).filter(StatsColumn::Name.eq(name)).one(&transcation).await? {
            Some(one) => {
                let mut one: StatsActiveModel = one.into();
                one.group_id = Set(group_id.0);
                one.save(&transcation).await?;
            },
            None => {},
        }
        transcation.commit().await
    }

    /// stats
    pub async fn top(&self, chat: &Chat) -> Option<Vec<TopData>> {
        let transcation = match self.db.begin().await {
            Ok(t) => t,
            Err(error) => {
                log_error_ln!("{}", error);
                return None;
            }
        };

        let query = Stats::find()
            .select_only()
            .column(StatsColumn::Name)
            .column_as(StatsColumn::Counts.sum(), "counts")
            .group_by(StatsColumn::Name)
            .order_by_desc(StatsColumn::Counts.sum())
            .limit(LIMIT);

        let query = if chat.is_group() || chat.is_supergroup() {
            query.filter(StatsColumn::GroupId.eq(chat.id.0))
        } else {
            query
        };

        log_debug_ln!(
            "SQL: {:?}",
            query.build(transcation.get_database_backend()).to_string()
        );

        let result = query.into_model().all(&transcation).await;

        match result {
            Ok(result) => Some(result),
            Err(error) => {
                log_error_ln!("{}", error);
                None
            }
        }
    }

    pub async fn find_by_name(&self, name: &String) -> Option<Vec<StatsModel>> {
        let transcation = match self.db.begin().await {
            Ok(t) => t,
            Err(error) => {
                log_error_ln!("{}", error);
                return None;
            }
        };

        let result = Stats::find()
            .filter(StatsColumn::Name.contains(name))
            .limit(LIMIT).all(&transcation).await;

        match result {
            Ok(result) => Some(result),
            Err(error) => {
                log_error_ln!("{}", error);
                None
            }
        }
    }
}
