use uuid::Uuid;

use crate::{
    error::app_error::AppError,
    models::transaction::{
        CreateTransactionRequest,
        UpdateTransactionRequest,
        TransactionResponse,
    },
    repository::{
        portfolio_repository::PortfolioRepository,
        transaction_repository::TransactionRepository,
    },
};

pub struct TransactionService {
    transaction_repository: TransactionRepository,
    portfolio_repository: PortfolioRepository,
}

impl TransactionService {
    pub fn new(
        transaction_repository: TransactionRepository,
        portfolio_repository: PortfolioRepository,
    ) -> Self {
        Self {
            transaction_repository,
            portfolio_repository,
        }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        request: CreateTransactionRequest,
    ) -> Result<TransactionResponse, AppError> {

        let portfolio = self
            .portfolio_repository
            .find_by_id(request.portfolio_id)
            .await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        let transaction = self
            .transaction_repository
            .create(request)
            .await?;

        Ok(transaction.into())
    }

  pub async fn list(
    &self,
    user_id: Uuid,
) -> Result<Vec<TransactionResponse>, AppError> {

    let portfolios = self
        .portfolio_repository
        .list_by_user(user_id)
        .await?;

    let mut all_transactions = Vec::new();

    for portfolio in portfolios {
        let mut transactions = self
            .transaction_repository
            .list(portfolio.id)
            .await?;

        all_transactions.append(&mut transactions);
    }

    Ok(all_transactions.into_iter().map(Into::into).collect())
}
pub async fn get(
    &self,
    id: Uuid,
    user_id: Uuid,
) -> Result<TransactionResponse, AppError> {

    let transaction = self
        .transaction_repository
        .find_by_id(id)
        .await?
        .ok_or(AppError::TransactionNotFound)?;

    let portfolio = self
        .portfolio_repository
        .find_by_id(transaction.portfolio_id)
        .await?
        .ok_or(AppError::PortfolioNotFound)?;

    if portfolio.user_id != user_id {
        return Err(AppError::NotAuthorized);
    }

    Ok(transaction.into())
}

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateTransactionRequest,
    ) -> Result<TransactionResponse, AppError> {

        let transaction = self
            .transaction_repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::TransactionNotFound)?;

        let portfolio = self
            .portfolio_repository
            .find_by_id(transaction.portfolio_id)
            .await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        let transaction = self
            .transaction_repository
            .update(id, request)
            .await?;

        Ok(transaction.into())
    }

    pub async fn delete(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {

        let transaction = self
            .transaction_repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::TransactionNotFound)?;

        let portfolio = self
            .portfolio_repository
            .find_by_id(transaction.portfolio_id)
            .await?
            .ok_or(AppError::PortfolioNotFound)?;

        if portfolio.user_id != user_id {
            return Err(AppError::NotAuthorized);
        }

        self.transaction_repository.delete(id).await?;

        Ok(())
    }
}