use axum::{routing::get, Router};

use crate::app::AppState;
use crate::handlers::assets::{create_asset, delete_asset, list_assets, update_asset};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/assets",
        get(list_assets)
            .post(create_asset)
            .patch(update_asset)
            .delete(delete_asset),
    )
}

#[cfg(test)]
mod tests {
    use axum::Json;
    use sqlx::PgPool;

    use crate::auth::admin::Admin;
    use crate::error::AppError;
    use crate::handlers::assets::{CreateAssetRequest, DeleteAssetRequest, UpdateAssetRequest};

    use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Bitcoin".to_string(),
            unit_value: 10.0,
        };

        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.unit_value, 10.0);

        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("success");

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");

        insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_asset(db: PgPool) {
        let request = UpdateAssetRequest {
            id: 1,
            name: Some("Ethereum".to_string()),
            unit_value: Some(20.0),
        };

        let Json(updated_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(updated_asset.id, 1);
        assert_eq!(updated_asset.name, "Ethereum");
        assert_eq!(updated_asset.unit_value, 20.0);

        insta::assert_json_snapshot!(updated_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_delete_asset(db: PgPool) {
        let request = DeleteAssetRequest { id: 1 };

        let Json(deleted_asset) = delete_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(deleted_asset.id, 1);
        assert_eq!(deleted_asset.name, "Bitcoin");
        assert_eq!(deleted_asset.unit_value, 10.0);

        insta::assert_json_snapshot!(deleted_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset_with_history"))]
    async fn test_delete_asset_with_history(db: PgPool) {
        let request = DeleteAssetRequest { id: 1 };

        let result = delete_asset(Admin, db.into(), Json(request)).await;

        assert!(matches!(result, Err(AppError::AssetCannotBeDeletedBecauseHasHistory)));
    }
}
