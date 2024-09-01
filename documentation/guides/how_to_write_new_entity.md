This guide covers how to create a new entity from scratch

### Hand-write the migration

Create a new file in your migrations folder (/migration/src) (e.g., m20230901_000001_create_new_table.rs):

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NewTable::Table)
                    .col(ColumnDef::new(NewTable::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(NewTable::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NewTable::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum NewTable {
    Table,
    Id,
    Name,
}
```

## Run the migration

Run the migration:

`sea-orm-cli migrate up`

Ensure you have an entity module in your project. Then run:

`sea-orm-cli generate entity -o entity/src`

This will generate the entity files based on your database schema.

The generated entity will be available in your entity module. You can now use it in your application code.