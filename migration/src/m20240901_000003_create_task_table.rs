use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;
use sea_orm_migration::sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Create enum type
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE task_status AS ENUM ('TODO', 'IN_PROGRESS', 'DONE')"#.to_owned(),
        ))
        .await?;

        // Create task table
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Task::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Task::Title).string().not_null())
                    .col(ColumnDef::new(Task::Description).text().not_null())
                    .col(ColumnDef::new(Task::Status).custom(TaskStatus::Table).not_null())
                    .col(ColumnDef::new(Task::DueDate).date_time())
                    .col(ColumnDef::new(Task::UserId).uuid().not_null())
                    .col(ColumnDef::new(Task::CategoryId).uuid())
                    .col(ColumnDef::new(Task::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Task::UpdatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-user_id")
                            .from(Task::Table, Task::UserId)
                            .to(User::Table, User::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-category_id")
                            .from(Task::Table, Task::CategoryId)
                            .to(Category::Table, Category::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Drop task table
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;

        // Drop enum type
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            "DROP TYPE task_status".to_owned(),
        ))
        .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Task {
    Table,
    Id,
    Title,
    Description,
    Status,
    DueDate,
    UserId,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum TaskStatus {
    Table,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum Category {
    Table,
    Id,
}
