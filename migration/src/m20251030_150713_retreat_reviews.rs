use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RetreatReviews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RetreatReviews::ReviewId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RetreatReviews::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RetreatReviews::RetreatId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RetreatReviews::Rating)
                            .double()
                            .not_null()
                            .check(
                                Expr::col(RetreatReviews::Rating)
                                    .gte(0)
                                    .and(Expr::col(RetreatReviews::Rating).lte(5)),
                            ),
                    )
                    .col(
                        ColumnDef::new(RetreatReviews::Review)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RetreatReviews::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RetreatReviews::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_unique_user_retreat_review")
                            .col(RetreatReviews::UserId)
                            .col(RetreatReviews::RetreatId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_review_user")
                            .from(RetreatReviews::Table, RetreatReviews::UserId)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_review_retreat")
                            .from(RetreatReviews::Table, RetreatReviews::RetreatId)
                            .to(Retreats::Table, Retreats::RetreatId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        // Attach trigger to user table
        db.execute_unprepared(
            r#"
            CREATE TRIGGER trigger_set_updated_at
            BEFORE UPDATE ON "retreat_reviews"
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at();
            "#
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"DROP TRIGGER IF EXISTS trigger_set_updated_at ON "retreat_reviews";"#
        ).await?;

        manager
            .drop_table(Table::drop().table(RetreatReviews::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum RetreatReviews {
    Table,
    ReviewId,
    UserId,
    RetreatId,
    Rating,
    Review,
    CreatedAt,
    UpdatedAt,
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
