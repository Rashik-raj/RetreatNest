use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RetreatGalleries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RetreatGalleries::GalleryId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::RetreatId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::ImagePath)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::Caption)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::Order)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::CreatedBy)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RetreatGalleries::UpdatedBy)
                            .big_integer()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gallery_retreat")
                            .from(RetreatGalleries::Table, RetreatGalleries::RetreatId)
                            .to(Retreats::Table, Retreats::RetreatId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gallery_created_by")
                            .from(RetreatGalleries::Table, RetreatGalleries::CreatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gallery_updated_by")
                            .from(RetreatGalleries::Table, RetreatGalleries::UpdatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::SetNull)
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
            BEFORE UPDATE ON "retreat_galleries"
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at();
            "#
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
         let db = manager.get_connection();

        db.execute_unprepared(
            r#"DROP TRIGGER IF EXISTS trigger_set_updated_at ON "retreat_galleries";"#
        ).await?;
        manager
            .drop_table(Table::drop().table(RetreatGalleries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RetreatGalleries {
    Table,
    GalleryId,
    RetreatId,
    ImagePath,
    Caption,
    Order,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum Retreats {
    Table,
    RetreatId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}
