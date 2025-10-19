use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1️⃣ Create Categories Table
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::CategoryId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::Description).text().null())
                    .col(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Categories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Categories::CreatedBy).big_integer().null())
                    .col(ColumnDef::new(Categories::UpdatedBy).big_integer().null())
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(Categories::Table, Categories::CreatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(Categories::Table, Categories::UpdatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 2️⃣ Create Retreats Table
        manager
            .create_table(
                Table::create()
                    .table(Retreats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Retreats::RetreatId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Retreats::Name).string().not_null())
                    .col(ColumnDef::new(Retreats::Description).text().null())
                    .col(ColumnDef::new(Retreats::CategoryId).big_integer().not_null())
                    .col(ColumnDef::new(Retreats::Slug).string().unique_key().not_null())
                    .col(ColumnDef::new(Retreats::SocialLinks).json().not_null())
                    .col(ColumnDef::new(Retreats::Email).string().null())
                    .col(ColumnDef::new(Retreats::Phone).string().null())
                    .col(ColumnDef::new(Retreats::Logo).string().null())
                    .col(ColumnDef::new(Retreats::Latitude).decimal().null())
                    .col(ColumnDef::new(Retreats::Longitude).decimal().null())
                    .col(ColumnDef::new(Retreats::Address).text().null())
                    .col(ColumnDef::new(Retreats::BudgetMin).decimal().null())
                    .col(ColumnDef::new(Retreats::BudgetMax).decimal().null())
                    .col(
                        ColumnDef::new(Retreats::IsPublished)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Retreats::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Retreats::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Retreats::CreatedBy).big_integer().null())
                    .col(ColumnDef::new(Retreats::UpdatedBy).big_integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Retreats::Table, Retreats::CategoryId)
                            .to(Categories::Table, Categories::CategoryId)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(Retreats::Table, Retreats::CreatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(Retreats::Table, Retreats::UpdatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 3️⃣ Create RetreatUsers Table
        manager
            .create_table(
                Table::create()
                    .table(RetreatUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RetreatUsers::RetreatUserId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RetreatUsers::RetreatId).big_integer().not_null())
                    .col(ColumnDef::new(RetreatUsers::UserId).big_integer().not_null())
                    .col(
                        ColumnDef::new(RetreatUsers::IsOwner)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(RetreatUsers::Role).string().null())
                    .col(
                        ColumnDef::new(RetreatUsers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RetreatUsers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(RetreatUsers::CreatedBy).big_integer().null())
                    .col(ColumnDef::new(RetreatUsers::UpdatedBy).big_integer().null())
                    // FK -> Retreats
                    .foreign_key(
                        ForeignKey::create()
                            .from(RetreatUsers::Table, RetreatUsers::RetreatId)
                            .to(Retreats::Table, Retreats::RetreatId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(RetreatUsers::Table, RetreatUsers::UserId)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(RetreatUsers::Table, RetreatUsers::CreatedBy)
                            .to(Users::Table, Users::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    // FK -> Users
                    .foreign_key(
                        ForeignKey::create()
                            .from(RetreatUsers::Table, RetreatUsers::UpdatedBy)
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
            BEFORE UPDATE ON "categories"
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at();
            "#
        ).await?;

        // Attach trigger to user table
        db.execute_unprepared(
            r#"
            CREATE TRIGGER trigger_set_updated_at
            BEFORE UPDATE ON "retreats"
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at();
            "#
        ).await?;

        // Attach trigger to user table
        db.execute_unprepared(
            r#"
            CREATE TRIGGER trigger_set_updated_at
            BEFORE UPDATE ON "retreat_users"
            FOR EACH ROW
            EXECUTE FUNCTION set_updated_at();
            "#
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"DROP TRIGGER IF EXISTS trigger_set_updated_at ON "retreat_users";"#
        ).await?;

        db.execute_unprepared(
            r#"DROP TRIGGER IF EXISTS trigger_set_updated_at ON "retreats";"#
        ).await?;

        db.execute_unprepared(
            r#"DROP TRIGGER IF EXISTS trigger_set_updated_at ON "categories";"#
        ).await?;

        // Drop in reverse dependency order
        manager
            .drop_table(Table::drop().table(RetreatUsers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Retreats::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    CategoryId,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum Retreats {
    Table,
    RetreatId,
    Name,
    Description,
    CategoryId,
    Slug,
    SocialLinks,
    Email,
    Phone,
    Logo,
    Latitude,
    Longitude,
    Address,
    BudgetMin,
    BudgetMax,
    IsPublished,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum RetreatUsers {
    Table,
    RetreatUserId,
    RetreatId,
    UserId,
    IsOwner,
    Role,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}

// Assuming a global users table exists elsewhere in your system
#[derive(DeriveIden)]
enum Users {
    Table,
    UserId
}
