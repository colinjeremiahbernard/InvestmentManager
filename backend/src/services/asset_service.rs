use uuid::Uuid;

use crate::{
    error::app_error::AppError,
    models::asset::{Asset, AssetResponse, CreateAssetRequest},
    repository::asset_repository::AssetRepository,
};

pub struct AssetService {
    repository: AssetRepository,
}

impl AssetService {
    pub fn new(repository: AssetRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, mut request: CreateAssetRequest) -> Result<AssetResponse, AppError> {
        request.symbol = request.symbol.to_uppercase();

        if self
            .repository
            .find_by_symbol(&request.symbol)
            .await?
            .is_some()
        {
            return Err(AppError::AssetAlreadyExists);
        }

        let asset = self
            .repository
            .create(request)
            .await
            .map_err(AppError::from)?;
        Ok(asset_to_response(asset))
    }

    pub async fn list(&self) -> Result<Vec<AssetResponse>, AppError> {
        let assets = self.repository.list().await.map_err(AppError::from)?;
        Ok(assets.into_iter().map(asset_to_response).collect())
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<AssetResponse>, AppError> {
        let asset = self
            .repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)?;
        Ok(asset.map(asset_to_response))
    }

    pub async fn update(
        &self,
        id: Uuid,
        request: CreateAssetRequest,
    ) -> Result<AssetResponse, AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::AssetNotFound)?;

        let asset = self
            .repository
            .update(id, request)
            .await
            .map_err(AppError::from)?;
        Ok(asset_to_response(asset))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::AssetNotFound)?;

        self.repository.delete(id).await.map_err(AppError::from)
    }
}

fn asset_to_response(asset: Asset) -> AssetResponse {
    AssetResponse {
        id: asset.id,
        symbol: asset.symbol,
        name: asset.name,
        asset_type: asset.asset_type,
        exchange: asset.exchange,
        currency: asset.currency,
        current_price: asset.current_price,
        is_active: asset.is_active,
    }
}
