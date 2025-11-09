use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GalleryCategories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GalleryCategories::GalleryCategoryId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GalleryCategories::Name).string().not_null())
                    .col(
                        ColumnDef::new(GalleryCategories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(GalleryCategories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(GalleryCategories::CreatedBy)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(GalleryCategories::UpdatedBy)
                            .big_integer()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GalleryCategories::Table, GalleryCategories::CreatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(GalleryCategories::Table, GalleryCategories::UpdatedBy)
                            .to(Users::Table, Users::UserId)
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
            BEFORE UPDATE ON "gallery_categories"
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at();
            "#,
        )
        .await?;

        // add gallery_category_id foreign key to retreat_galleries model
        manager
            .alter_table(
                Table::alter()
                    .table(RetreatGalleries::Table)
                    .add_column(
                        ColumnDef::new(RetreatGalleries::GalleryCategoryId)
                            .big_integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(RetreatGalleries::Table, RetreatGalleries::GalleryCategoryId)
                    .to(
                        GalleryCategories::Table,
                        GalleryCategories::GalleryCategoryId,
                    )
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("retreat_galleries_gallery_category_id_fkey") // name it same as you used
                    .table(RetreatGalleries::Table)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        db.execute_unprepared(
            r#"DROP TRIGGER IF EXISTS trigger_set_updated_at ON "gallery_categories";"#,
        )
        .await?;

        manager
            .drop_table(Table::drop().table(GalleryCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GalleryCategories {
    Table,
    GalleryCategoryId,
    Name,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum RetreatGalleries {
    Table,
    GalleryCategoryId,
}
