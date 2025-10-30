use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wishlists::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Wishlists::WishlistId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Wishlists::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Wishlists::RetreatId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Wishlists::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_unique_user_retreat_wishlist")
                            .col(Wishlists::UserId)
                            .col(Wishlists::RetreatId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wishlist_user")
                            .from(Wishlists::Table, Wishlists::UserId)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wishlist_retreat")
                            .from(Wishlists::Table, Wishlists::RetreatId)
                            .to(Retreats::Table, Retreats::RetreatId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wishlists::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Wishlists {
    Table,
    WishlistId,
    UserId,
    RetreatId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum Retreats {
    Table,
    RetreatId,
}
