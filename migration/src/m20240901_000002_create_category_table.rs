use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Category::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(ColumnDef::new(Category::UserId).uuid().not_null())
                    .col(ColumnDef::new(Category::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Category::UpdatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category-user_id")
                            .from(Category::Table, Category::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Category {
    Table,
    Id,
    Name,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
