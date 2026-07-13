use uuid::Uuid;

use crate::{
    error::app_error::AppError,
    models::portfolio::{
        CreatePortfolioRequest, UpdatePortfolioRequest,
        Portfolio, PortfolioDetailResponse,
        AddPortfolioItemRequest, UpdatePortfolioItemRequest,
        PortfolioItem,
    },
    repository::portfolio_repository::PortfolioRepository,
};

pub struct PortfolioService {
    repository: PortfolioRepository,
}

impl PortfolioService {
    pub fn new(repository: PortfolioRepository) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        request: CreatePortfolioRequest,
    ) -> Result<Portfolio, AppError> {
        self.repository.create(user_id, request).await.map_err(AppError::from)
    }

    pub async fn list(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Portfolio>, AppError> {
        self.repository.list_by_user(user_id).await.map_err(AppError::from)
    }
    pub async fn summary(
    &self,
    user_id: Uuid,
) -> Result<PortfolioDetailResponse, AppError> {

    let portfolios = self
        .repository
        .list_by_user(user_id)
        .await?;

    let mut total_invested = 0.0;
    let mut total_value = 0.0;

    for portfolio in &portfolios {

        let items = self
            .repository
            .list_items(portfolio.id)
            .await?;

        total_invested += items
            .iter()
            .map(|i| i.total_invested)
            .sum::<f64>();

        total_value += items
            .iter()
            .map(|i| i.current_value)
            .sum::<f64>();
    }

    let total_gain_loss =
        total_value - total_invested;

    let total_gain_loss_pct =
        if total_invested == 0.0 {

            0.0

        } else {

            total_gain_loss
                / total_invested
                * 100.0

        };

    Ok(PortfolioDetailResponse {

        id: Uuid::nil(),

        name: "All Portfolios".to_string(),

        description: None,

        items: Vec::new(),

        total_invested,

        total_value,

        total_gain_loss,

        total_gain_loss_pct,

    })
}

    pub async fn get_detail(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<PortfolioDetailResponse, AppError> {
        let portfolio = self.repository.find_by_id(id).await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        let items = self.repository.list_items(id).await?;

        let total_invested: f64 = items.iter().map(|i| i.total_invested).sum();
        let total_value: f64   = items.iter().map(|i| i.current_value).sum();
        let total_gain_loss    = total_value - total_invested;
        let total_gain_loss_pct = if total_invested == 0.0 {
            0.0
        } else {
            (total_gain_loss / total_invested) * 100.0
        };

        Ok(PortfolioDetailResponse {
            id: portfolio.id,
            name: portfolio.name,
            description: portfolio.description,
            items,
            total_invested,
            total_value,
            total_gain_loss,
            total_gain_loss_pct,
        })
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdatePortfolioRequest,
    ) -> Result<Portfolio, AppError> {
        let portfolio = self.repository.find_by_id(id).await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        self.repository.update(id, request.name, request.description)
            .await.map_err(AppError::from)
    }

    pub async fn delete(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let portfolio = self.repository.find_by_id(id).await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        self.repository.delete(id).await.map_err(AppError::from)
    }

    pub async fn add_item(
        &self,
        portfolio_id: Uuid,
        user_id: Uuid,
        request: AddPortfolioItemRequest,
    ) -> Result<PortfolioItem, AppError> {
        let portfolio = self.repository.find_by_id(portfolio_id).await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        self.repository.add_item(portfolio_id, request).await.map_err(AppError::from)
    }

    pub async fn update_item(
        &self,
        portfolio_id: Uuid,
        item_id: Uuid,
        user_id: Uuid,
        request: UpdatePortfolioItemRequest,
    ) -> Result<PortfolioItem, AppError> {
        let portfolio = self.repository.find_by_id(portfolio_id).await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        self.repository.find_item_by_id(item_id).await?
            .ok_or(AppError::PortfolioItemNotFound)?;

        self.repository.update_item(item_id, request).await.map_err(AppError::from)
    }

    pub async fn remove_item(
        &self,
        portfolio_id: Uuid,
        item_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let portfolio = self.repository.find_by_id(portfolio_id).await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        self.repository.find_item_by_id(item_id).await?
            .ok_or(AppError::PortfolioItemNotFound)?;

        self.repository.delete_item(item_id).await.map_err(AppError::from)
    }
}
