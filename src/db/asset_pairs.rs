use sqlx::PgPool;
use crate::models::asset_pairs::{AssetPair, CreateAssetPairRequest};
use tracing::{info, error};

pub async fn create_asset_pair(
    pool: &PgPool,
    user_id: i32,
    req: &CreateAssetPairRequest,
) -> Result<AssetPair, sqlx::Error> {
    info!("Creando asset pair: base={}, quote={}, slip={}", req.base_asset, req.quote_asset, req.slip_percentage);
    
    let result = sqlx::query_as!(
        AssetPair,
        r#"
        INSERT INTO asset_pairs (
            user_id, 
            base_asset, 
            quote_asset, 
            slip_percentage,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        user_id,
        req.base_asset,
        req.quote_asset,
        req.slip_percentage,
    )
    .fetch_one(pool)
    .await;

    match &result {
        Ok(asset_pair) => info!("Asset pair creado exitosamente con ID: {}", asset_pair.id),
        Err(e) => error!("Error al crear asset pair: {:?}", e),
    }

    result
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<AssetPair, sqlx::Error> {
    info!("Obteniendo asset pair por ID: {}", id);
    
    let result = sqlx::query_as!(
        AssetPair,
        r#"
        SELECT *
        FROM asset_pairs
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await;

    match &result {
        Ok(asset_pair) => info!("Asset pair obtenido exitosamente con ID: {}", asset_pair.id),
        Err(e) => error!("Error al obtener asset pair: {:?}", e),
    }

    result
}

pub async fn get_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<AssetPair>, sqlx::Error> {
    info!("Obteniendo asset pairs por usuario ID: {}", user_id);
    
    let result = sqlx::query_as!(
        AssetPair,
        r#"
        SELECT *
        FROM asset_pairs
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await;

    match &result {
        Ok(asset_pairs) => info!("Asset pairs obtenidos exitosamente para usuario ID: {}", user_id),
        Err(e) => error!("Error al obtener asset pairs: {:?}", e),
    }

    result
}

pub async fn get_asset_pair(
    pool: &PgPool,
    id: i32,
    user_id: i32,
) -> Result<AssetPair, sqlx::Error> {
    info!("Obteniendo asset pair por ID y usuario ID: {}", id);
    
    let result = sqlx::query_as!(
        AssetPair,
        r#"
        SELECT *
        FROM asset_pairs
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .fetch_one(pool)
    .await;

    match &result {
        Ok(asset_pair) => info!("Asset pair obtenido exitosamente con ID: {}", asset_pair.id),
        Err(e) => error!("Error al obtener asset pair: {:?}", e),
    }

    result
}

pub async fn list_asset_pairs(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<AssetPair>, sqlx::Error> {
    info!("Obteniendo lista de asset pairs por usuario ID: {}", user_id);
    
    let result = sqlx::query_as!(
        AssetPair,
        r#"
        SELECT *
        FROM asset_pairs
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await;

    match &result {
        Ok(asset_pairs) => info!("Lista de asset pairs obtenida exitosamente para usuario ID: {}", user_id),
        Err(e) => error!("Error al obtener lista de asset pairs: {:?}", e),
    }

    result
}

pub async fn update_asset_pair(
    pool: &PgPool,
    id: i32,
    user_id: i32,
    req: &CreateAssetPairRequest,
) -> Result<AssetPair, sqlx::Error> {
    info!("Actualizando asset pair con ID: {}", id);
    
    let result = sqlx::query_as!(
        AssetPair,
        r#"
        UPDATE asset_pairs
        SET base_asset = $1,
            quote_asset = $2,
            slip_percentage = $3,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4 AND user_id = $5
        RETURNING *
        "#,
        req.base_asset,
        req.quote_asset,
        req.slip_percentage,
        id,
        user_id
    )
    .fetch_one(pool)
    .await;

    match &result {
        Ok(asset_pair) => info!("Asset pair actualizado exitosamente con ID: {}", asset_pair.id),
        Err(e) => error!("Error al actualizar asset pair: {:?}", e),
    }

    result
}

pub async fn delete(
    pool: &PgPool,
    id: i32,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    info!("Eliminando asset pair con ID: {}", id);
    
    let result = sqlx::query!(
        r#"
        DELETE FROM asset_pairs
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    match result.rows_affected() {
        1 => {
            info!("Asset pair eliminado exitosamente con ID: {}", id);
            Ok(())
        },
        0 => {
            error!("No se encontró el asset pair con ID: {}", id);
            Ok(())
        },
        _ => {
            error!("Se eliminaron múltiples asset pairs con ID: {}", id);
            Ok(())
        }
    }
}

pub async fn delete_asset_pair(
    pool: &PgPool,
    id: i32,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    info!("Eliminando asset pair con ID: {}", id);
    
    let result = sqlx::query!(
        r#"
        DELETE FROM asset_pairs
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_asset_pairs_admin(pool: &PgPool) -> Result<Vec<AssetPair>, sqlx::Error> {
    sqlx::query_as!(
        AssetPair,
        r#"
        SELECT id, user_id, base_asset, quote_asset, slip_percentage, created_at, updated_at
        FROM asset_pairs
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await
}