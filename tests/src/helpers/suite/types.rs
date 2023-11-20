use cosmwasm_std::{Addr, Binary, StdResult};
use cw_multi_test::AppResponse;

use anyhow::Error;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use counter_base::math::P6;

pub const DEFAULT_FUNDS_AMOUNT: u128 = 1; // give each user 1 asset (1 CRD, 1 INJ, etc.)
pub const INCREASED_FUNDS_AMOUNT: u128 = 100 * P6; // give admin such amount of assets to ensure providing 1e6 of assets to each pair

pub const DEFAULT_DECIMALS: u8 = 6;
pub const INCREASED_DECIMALS: u8 = 18;

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectAccount {
    #[strum(serialize = "admin")]
    Admin,
    #[strum(serialize = "alice")]
    Alice,
    #[strum(serialize = "bob")]
    Bob,
}

impl ProjectAccount {
    pub fn get_initial_funds_amount(&self) -> u128 {
        match self {
            ProjectAccount::Admin => INCREASED_FUNDS_AMOUNT,
            ProjectAccount::Alice => DEFAULT_FUNDS_AMOUNT,
            ProjectAccount::Bob => DEFAULT_FUNDS_AMOUNT,
        }
    }
}

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectCoin {
    #[strum(serialize = "ucrd")]
    Denom,
    #[strum(serialize = "unoria")]
    Noria,
}

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectToken {
    #[strum(serialize = "contract0")]
    Atom,
    #[strum(serialize = "contract1")]
    Luna,
    #[strum(serialize = "contract2")]
    Inj,
}

pub trait GetDecimals {
    fn get_decimals(&self) -> u8;
}

impl GetDecimals for ProjectAsset {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectAsset::Coin(project_coin) => project_coin.get_decimals(),
            ProjectAsset::Token(project_token) => project_token.get_decimals(),
        }
    }
}

impl GetDecimals for ProjectCoin {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectCoin::Denom => DEFAULT_DECIMALS,
            ProjectCoin::Noria => DEFAULT_DECIMALS,
        }
    }
}

impl GetDecimals for ProjectToken {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectToken::Atom => DEFAULT_DECIMALS,
            ProjectToken::Luna => DEFAULT_DECIMALS,
            ProjectToken::Inj => INCREASED_DECIMALS,
        }
    }
}

impl From<ProjectAccount> for Addr {
    fn from(project_account: ProjectAccount) -> Self {
        Self::unchecked(project_account.to_string())
    }
}

impl From<ProjectToken> for Addr {
    fn from(project_token: ProjectToken) -> Self {
        Addr::unchecked(project_token.to_string())
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum ProjectAsset {
    Coin(ProjectCoin),
    Token(ProjectToken),
}

impl From<ProjectCoin> for ProjectAsset {
    fn from(project_coin: ProjectCoin) -> Self {
        Self::Coin(project_coin)
    }
}

impl From<ProjectToken> for ProjectAsset {
    fn from(project_token: ProjectToken) -> Self {
        Self::Token(project_token)
    }
}

#[derive(Debug)]
pub enum WrappedResponse {
    Execute(Result<AppResponse, Error>),
    Query(StdResult<Binary>),
}

impl From<Result<AppResponse, Error>> for WrappedResponse {
    fn from(exec_res: Result<AppResponse, Error>) -> Self {
        Self::Execute(exec_res)
    }
}

impl From<StdResult<Binary>> for WrappedResponse {
    fn from(query_res: StdResult<Binary>) -> Self {
        Self::Query(query_res)
    }
}
