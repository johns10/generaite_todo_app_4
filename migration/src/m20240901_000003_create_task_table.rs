use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TaskStatus::Table)
                    .values(vec![
                        TaskStatus::Todo,
                        TaskStatus::InProgress,
                        TaskStatus::Done,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Task::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Task::Title).string().not_null())
                    .col(ColumnDef::new(Task::Description).text().not_null())
                    .col(ColumnDef::new(Task::Status).enumeration(TaskStatus::Table, vec![
                        TaskStatus::Todo.to_string(),
                        TaskStatus::InProgress.to_string(),
                        TaskStatus::Done.to_string(),
                    ]).not_null())
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
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TaskStatus::Table).to_owned())
            .await
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
    Todo,
    InProgress,
    Done,
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
