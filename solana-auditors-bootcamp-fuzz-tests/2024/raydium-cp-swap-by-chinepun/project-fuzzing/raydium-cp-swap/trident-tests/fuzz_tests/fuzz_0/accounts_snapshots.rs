use anchor_lang::prelude::*;
// use anchor_lang::prelude::InterfaceAccount;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_2022::Token2022, token_interface::{Mint, TokenAccount, TokenInterface}};
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct CreateAmmConfigSnapshot<'info> {
    pub owner: Signer<'info>,
    pub amm_config: Option<Account<'info, raydium_cp_swap::states::config::AmmConfig>>,
    pub system_program: Program<'info, System>,
}
pub struct UpdateAmmConfigSnapshot<'info> {
    pub owner: Signer<'info>,
    pub amm_config: Account<'info, raydium_cp_swap::states::config::AmmConfig>,
}
pub struct UpdatePoolStatusSnapshot<'info> {
    pub authority: Signer<'info>,
    pub pool_state: AccountLoader<'info, raydium_cp_swap::states::pool::PoolState>,
}
pub struct CollectProtocolFeeSnapshot<'info> {
    pub owner: Signer<'info>,
    pub authority: UncheckedAccount<'info>,
    pub pool_state: AccountLoader<'info, raydium_cp_swap::states::pool::PoolState>,
    pub amm_config: Account<'info, raydium_cp_swap::states::config::AmmConfig>,
    pub token_0_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_1_vault: InterfaceAccount<'info, TokenAccount>,
    pub vault_0_mint: InterfaceAccount<'info, Mint>,
    pub vault_1_mint: InterfaceAccount<'info, Mint>,
    pub recipient_token_0_account: InterfaceAccount<'info, TokenAccount>,
    pub recipient_token_1_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
}
pub struct CollectFundFeeSnapshot<'info> {
    pub owner: Signer<'info>,
    pub authority: UncheckedAccount<'info>,
    pub pool_state: AccountLoader<'info, raydium_cp_swap::states::pool::PoolState>,
    pub amm_config: Account<'info, raydium_cp_swap::states::config::AmmConfig>,
    pub token_0_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_1_vault: InterfaceAccount<'info, TokenAccount>,
    pub vault_0_mint: InterfaceAccount<'info, Mint>,
    pub vault_1_mint: InterfaceAccount<'info, Mint>,
    pub recipient_token_0_account: InterfaceAccount<'info, TokenAccount>,
    pub recipient_token_1_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
}
pub struct InitializeSnapshot<'info> {
    pub creator: Signer<'info>,
    pub amm_config: Account<'info, raydium_cp_swap::states::config::AmmConfig>,
    pub authority: UncheckedAccount<'info>,
    pub pool_state: UncheckedAccount<'info>,
    pub token_0_mint: InterfaceAccount<'info, Mint>,
    pub token_1_mint: InterfaceAccount<'info, Mint>,
    pub lp_mint: Option<InterfaceAccount<'info, Mint>>,
    pub creator_token_0: InterfaceAccount<'info, TokenAccount>,
    pub creator_token_1: InterfaceAccount<'info, TokenAccount>,
    pub creator_lp_token: Option<InterfaceAccount<'info, TokenAccount>>,
    pub token_0_vault: UncheckedAccount<'info>,
    pub token_1_vault: UncheckedAccount<'info>,
    pub create_pool_fee: InterfaceAccount<'info, TokenAccount>,
    pub observation_state:
        Option<AccountLoader<'info, raydium_cp_swap::states::oracle::ObservationState>>,
    pub token_program: Program<'info, Token>,
    pub token_0_program: Interface<'info, TokenInterface>,
    pub token_1_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
pub struct DepositSnapshot<'info> {
    pub owner: Signer<'info>,
    pub authority: UncheckedAccount<'info>,
    pub pool_state: AccountLoader<'info, raydium_cp_swap::states::pool::PoolState>,
    pub owner_lp_token: InterfaceAccount<'info, TokenAccount>,
    pub token_0_account: InterfaceAccount<'info, TokenAccount>,
    pub token_1_account: InterfaceAccount<'info, TokenAccount>,
    pub token_0_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_1_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub vault_0_mint: InterfaceAccount<'info, Mint>,
    pub vault_1_mint: InterfaceAccount<'info, Mint>,
    pub lp_mint: InterfaceAccount<'info, Mint>,
}
pub struct WithdrawSnapshot<'info> {
    pub owner: Signer<'info>,
    pub authority: UncheckedAccount<'info>,
    pub pool_state: AccountLoader<'info, raydium_cp_swap::states::pool::PoolState>,
    pub owner_lp_token: InterfaceAccount<'info, TokenAccount>,
    pub token_0_account: InterfaceAccount<'info, TokenAccount>,
    pub token_1_account: InterfaceAccount<'info, TokenAccount>,
    pub token_0_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_1_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub vault_0_mint: InterfaceAccount<'info, Mint>,
    pub vault_1_mint: InterfaceAccount<'info, Mint>,
    pub lp_mint: InterfaceAccount<'info, Mint>,
    pub memo_program: UncheckedAccount<'info>,
}
pub struct SwapBaseInputSnapshot<'info> {
    pub payer: Signer<'info>,
    pub authority: UncheckedAccount<'info>,
    pub amm_config: Account<'info, raydium_cp_swap::states::config::AmmConfig>,
    pub pool_state: AccountLoader<'info, raydium_cp_swap::states::pool::PoolState>,
    pub input_token_account: InterfaceAccount<'info, TokenAccount>,
    pub output_token_account: InterfaceAccount<'info, TokenAccount>,
    pub input_vault: InterfaceAccount<'info, TokenAccount>,
    pub output_vault: InterfaceAccount<'info, TokenAccount>,
    pub input_token_program: Interface<'info, TokenInterface>,
    pub output_token_program: Interface<'info, TokenInterface>,
    pub input_token_mint: InterfaceAccount<'info, Mint>,
    pub output_token_mint: InterfaceAccount<'info, Mint>,
    pub observation_state: AccountLoader<'info, raydium_cp_swap::states::oracle::ObservationState>,
}


impl<'info> CreateAmmConfigSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let owner: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("owner".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner".to_string()))?;
        let amm_config: Option<
            anchor_lang::accounts::account::Account<raydium_cp_swap::states::config::AmmConfig>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("amm_config".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("amm_config".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "amm_config".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            owner,
            amm_config,
            system_program,
        })
    }
}
impl<'info> UpdateAmmConfigSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let owner: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("owner".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner".to_string()))?;
        let amm_config: anchor_lang::accounts::account::Account<
            raydium_cp_swap::states::config::AmmConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("amm_config".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("amm_config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("amm_config".to_string()))?;
        Ok(Self { owner, amm_config })
    }
}

impl<'info> UpdatePoolStatusSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("authority".to_string()))?;
        let pool_state: anchor_lang::accounts::account_loader::AccountLoader<
            raydium_cp_swap::states::pool::PoolState,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account_loader::AccountLoader::try_from)
            .ok_or(FuzzingError::AccountNotFound("pool_state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_state".to_string()))?;
        Ok(Self {
            authority,
            pool_state,
        })
    }
}
impl<'info> CollectProtocolFeeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let owner: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("owner".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner".to_string()))?;
        let authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?;
        let pool_state: anchor_lang::accounts::account_loader::AccountLoader<
            raydium_cp_swap::states::pool::PoolState,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account_loader::AccountLoader::try_from)
            .ok_or(FuzzingError::AccountNotFound("pool_state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_state".to_string()))?;
        let amm_config: anchor_lang::accounts::account::Account<
            raydium_cp_swap::states::config::AmmConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("amm_config".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("amm_config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("amm_config".to_string()))?;
        let token_0_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_0_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_0_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_0_vault".to_string()))?;
        let token_1_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_1_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_1_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_1_vault".to_string()))?;
        let vault_0_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_0_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_0_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_0_mint".to_string()))?;
        let vault_1_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_1_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_1_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_1_mint".to_string()))?;
        let recipient_token_0_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "recipient_token_0_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "recipient_token_0_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("recipient_token_0_account".to_string())
            })?;
        let recipient_token_1_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "recipient_token_1_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "recipient_token_1_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("recipient_token_1_account".to_string())
            })?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let token_program_2022: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_program_2022".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "token_program_2022".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("token_program_2022".to_string())
            })?;
        Ok(Self {
            owner,
            authority,
            pool_state,
            amm_config,
            token_0_vault,
            token_1_vault,
            vault_0_mint,
            vault_1_mint,
            recipient_token_0_account,
            recipient_token_1_account,
            token_program,
            token_program_2022,
        })
    }
}


impl<'info> CollectFundFeeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let owner: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("owner".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner".to_string()))?;
        let authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?;
        let pool_state: anchor_lang::accounts::account_loader::AccountLoader<
            raydium_cp_swap::states::pool::PoolState,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account_loader::AccountLoader::try_from)
            .ok_or(FuzzingError::AccountNotFound("pool_state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_state".to_string()))?;
        let amm_config: anchor_lang::accounts::account::Account<
            raydium_cp_swap::states::config::AmmConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("amm_config".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("amm_config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("amm_config".to_string()))?;
        let token_0_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_0_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_0_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_0_vault".to_string()))?;
        let token_1_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_1_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_1_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_1_vault".to_string()))?;
        let vault_0_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_0_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_0_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_0_mint".to_string()))?;
        let vault_1_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_1_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_1_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_1_mint".to_string()))?;
        let recipient_token_0_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "recipient_token_0_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "recipient_token_0_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("recipient_token_0_account".to_string())
            })?;
        let recipient_token_1_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "recipient_token_1_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "recipient_token_1_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("recipient_token_1_account".to_string())
            })?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let token_program_2022: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_program_2022".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "token_program_2022".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("token_program_2022".to_string())
            })?;
        Ok(Self {
            owner,
            authority,
            pool_state,
            amm_config,
            token_0_vault,
            token_1_vault,
            vault_0_mint,
            vault_1_mint,
            recipient_token_0_account,
            recipient_token_1_account,
            token_program,
            token_program_2022,
        })
    }
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
        let amm_config: anchor_lang::accounts::account::Account<
            raydium_cp_swap::states::config::AmmConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("amm_config".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("amm_config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("amm_config".to_string()))?;
        let authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?;
        let pool_state = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("pool_state".to_string()))?;
        let token_0_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("token_0_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("token_0_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("token_0_mint".to_string()))?;
        let token_1_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("token_1_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("token_1_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("token_1_mint".to_string()))?;
        let lp_mint: Option<anchor_lang::accounts::interface_account::InterfaceAccount<Mint>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("lp_mint".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                            .map_err(|_| {
                                FuzzingError::CannotDeserializeAccount("lp_mint".to_string())
                            })
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided(
                            "lp_mint".to_string(),
                        ))
                    }
                })
                .transpose()
                .unwrap_or(None);
        let creator_token_0: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "creator_token_0".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator_token_0".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator_token_0".to_string()))?;
        let creator_token_1: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "creator_token_1".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator_token_1".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator_token_1".to_string()))?;
        let creator_lp_token: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "creator_lp_token".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("creator_lp_token".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "creator_lp_token".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let token_0_vault = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_0_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_0_vault".to_string()))?;
        let token_1_vault = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_1_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_1_vault".to_string()))?;
        let create_pool_fee: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "create_pool_fee".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("create_pool_fee".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("create_pool_fee".to_string()))?;
        let observation_state: Option<
            anchor_lang::accounts::account_loader::AccountLoader<
                raydium_cp_swap::states::oracle::ObservationState,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "observation_state".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account_loader::AccountLoader::try_from(acc).map_err(
                        |_| FuzzingError::CannotDeserializeAccount("observation_state".to_string()),
                    )
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "observation_state".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let token_0_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "token_0_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound("token_0_program".to_string()))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("token_0_program".to_string())
                })?;
        let token_1_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "token_1_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound("token_1_program".to_string()))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("token_1_program".to_string())
                })?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let rent: Sysvar<Rent> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::sysvar::Sysvar::from_account_info)
            .ok_or(FuzzingError::AccountNotFound("rent".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent".to_string()))?;
        Ok(Self {
            creator,
            amm_config,
            authority,
            pool_state,
            token_0_mint,
            token_1_mint,
            lp_mint,
            creator_token_0,
            creator_token_1,
            creator_lp_token,
            token_0_vault,
            token_1_vault,
            create_pool_fee,
            observation_state,
            token_program,
            token_0_program,
            token_1_program,
            associated_token_program,
            system_program,
            rent,
        })
    }
}
impl<'info> DepositSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let owner: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("owner".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner".to_string()))?;
        let authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?;
        let pool_state: anchor_lang::accounts::account_loader::AccountLoader<
            raydium_cp_swap::states::pool::PoolState,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account_loader::AccountLoader::try_from)
            .ok_or(FuzzingError::AccountNotFound("pool_state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_state".to_string()))?;
        let owner_lp_token: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "owner_lp_token".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner_lp_token".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner_lp_token".to_string()))?;
        let token_0_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_0_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_0_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_0_account".to_string()))?;
        let token_1_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_1_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_1_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_1_account".to_string()))?;
        let token_0_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_0_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_0_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_0_vault".to_string()))?;
        let token_1_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_1_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_1_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_1_vault".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let token_program_2022: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_program_2022".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "token_program_2022".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("token_program_2022".to_string())
            })?;
        let vault_0_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_0_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_0_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_0_mint".to_string()))?;
        let vault_1_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_1_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_1_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_1_mint".to_string()))?;
        let lp_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("lp_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("lp_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("lp_mint".to_string()))?;
        Ok(Self {
            owner,
            authority,
            pool_state,
            owner_lp_token,
            token_0_account,
            token_1_account,
            token_0_vault,
            token_1_vault,
            token_program,
            token_program_2022,
            vault_0_mint,
            vault_1_mint,
            lp_mint,
        })
    }
}
impl<'info> WithdrawSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let owner: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("owner".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner".to_string()))?;
        let authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?;
        let pool_state: anchor_lang::accounts::account_loader::AccountLoader<
            raydium_cp_swap::states::pool::PoolState,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account_loader::AccountLoader::try_from)
            .ok_or(FuzzingError::AccountNotFound("pool_state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_state".to_string()))?;
        let owner_lp_token: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "owner_lp_token".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("owner_lp_token".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("owner_lp_token".to_string()))?;
        let token_0_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_0_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_0_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_0_account".to_string()))?;
        let token_1_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_1_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_1_account".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_1_account".to_string()))?;
        let token_0_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_0_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_0_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_0_vault".to_string()))?;
        let token_1_vault: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_1_vault".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_1_vault".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_1_vault".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let token_program_2022: anchor_lang::accounts::program::Program<Token2022> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "token_program_2022".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "token_program_2022".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("token_program_2022".to_string())
            })?;
        let vault_0_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_0_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_0_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_0_mint".to_string()))?;
        let vault_1_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("vault_1_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("vault_1_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("vault_1_mint".to_string()))?;
        let lp_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("lp_mint".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("lp_mint".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("lp_mint".to_string()))?;
        let memo_program = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("memo_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("memo_program".to_string()))?;
        Ok(Self {
            owner,
            authority,
            pool_state,
            owner_lp_token,
            token_0_account,
            token_1_account,
            token_0_vault,
            token_1_vault,
            token_program,
            token_program_2022,
            vault_0_mint,
            vault_1_mint,
            lp_mint,
            memo_program,
        })
    }
}
impl<'info> SwapBaseInputSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?;
        let amm_config: anchor_lang::accounts::account::Account<
            raydium_cp_swap::states::config::AmmConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("amm_config".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("amm_config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("amm_config".to_string()))?;
        let pool_state: anchor_lang::accounts::account_loader::AccountLoader<
            raydium_cp_swap::states::pool::PoolState,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_state".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account_loader::AccountLoader::try_from)
            .ok_or(FuzzingError::AccountNotFound("pool_state".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_state".to_string()))?;
        let input_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "input_token_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "input_token_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("input_token_account".to_string())
            })?;
        let output_token_account: anchor_lang::accounts::interface_account::InterfaceAccount<
            TokenAccount,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "output_token_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "output_token_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("output_token_account".to_string())
            })?;
        let input_vault: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("input_vault".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("input_vault".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("input_vault".to_string()))?;
        let output_vault: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("output_vault".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("output_vault".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("output_vault".to_string()))?;
        let input_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "input_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "input_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("input_token_program".to_string())
                })?;
        let output_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "output_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "output_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("output_token_program".to_string())
                })?;
        let input_token_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "input_token_mint".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "input_token_mint".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("input_token_mint".to_string())
                })?;
        let output_token_mint: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "output_token_mint".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "output_token_mint".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("output_token_mint".to_string())
                })?;
        let observation_state: anchor_lang::accounts::account_loader::AccountLoader<
            raydium_cp_swap::states::oracle::ObservationState,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "observation_state".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account_loader::AccountLoader::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "observation_state".to_string(),
            ))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("observation_state".to_string()))?;
        Ok(Self {
            payer,
            authority,
            amm_config,
            pool_state,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            input_token_program,
            output_token_program,
            input_token_mint,
            output_token_mint,
            observation_state,
        })
    }
}
pub type SwapBaseOutputSnapshot<'info> = SwapBaseInputSnapshot<'info>;
