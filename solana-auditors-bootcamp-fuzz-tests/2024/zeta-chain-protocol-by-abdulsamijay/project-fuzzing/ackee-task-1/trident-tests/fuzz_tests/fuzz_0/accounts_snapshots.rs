use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct InitializeSnapshot<'info> {
    pub signer: Signer<'info>,
    pub pda: Option<Account<'info, solana_project_fuzz::Pda>>,
    pub system_program: Program<'info, System>,
}
pub struct DepositSnapshot<'info> {
    pub signer: Signer<'info>,
    pub pda: Account<'info, solana_project_fuzz::Pda>,
    pub system_program: Program<'info, System>,
}
pub struct WithdrawSnapshot<'info> {
    pub signer: Signer<'info>,
    pub pda: Account<'info, solana_project_fuzz::Pda>,
    pub to: SystemAccount<'info>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let pda: Option<anchor_lang::accounts::account::Account<solana_project_fuzz::Pda>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pda".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        anchor_lang::accounts::account::Account::try_from(acc)
                            .map_err(|_| FuzzingError::CannotDeserializeAccount("pda".to_string()))
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided("pda".to_string()))
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
            signer,
            pda,
            system_program,
        })
    }
}
impl<'info> DepositSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let pda: anchor_lang::accounts::account::Account<solana_project_fuzz::Pda> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pda".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("pda".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pda".to_string()))?;
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
            signer,
            pda,
            system_program,
        })
    }
}
impl<'info> WithdrawSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let pda: anchor_lang::accounts::account::Account<solana_project_fuzz::Pda> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pda".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("pda".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("pda".to_string()))?;
        let to: SystemAccount<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("to".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::system_account::SystemAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("to".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("to".to_string()))?;
        Ok(Self { signer, pda, to })
    }
}
