use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct CreateVestingEscrowSnapshot<'info> {
    pub base: Signer<'info>,
    pub escrow: Option<Account<'info, locker::state::vesting_escrow::VestingEscrow>>,
    pub escrow_token: Account<'info, TokenAccount>,
    pub sender: Signer<'info>,
    pub sender_token: Account<'info, TokenAccount>,
    pub recipient: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub event_authority: &'info AccountInfo<'info>,
    pub program: &'info AccountInfo<'info>,
}
pub struct ClaimSnapshot<'info> {
    pub escrow: Account<'info, locker::state::vesting_escrow::VestingEscrow>,
    pub escrow_token: Account<'info, TokenAccount>,
    pub recipient: Signer<'info>,
    pub recipient_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub event_authority: &'info AccountInfo<'info>,
    pub program: &'info AccountInfo<'info>,
}
pub struct CreateVestingEscrowMetadataSnapshot<'info> {
    pub escrow: Account<'info, locker::state::vesting_escrow::VestingEscrow>,
    pub creator: Signer<'info>,
    pub escrow_metadata:
        Option<Account<'info, locker::state::vesting_escrow_metadata::VestingEscrowMetadata>>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct UpdateVestingEscrowRecipientSnapshot<'info> {
    pub escrow: Account<'info, locker::state::vesting_escrow::VestingEscrow>,
    pub escrow_metadata:
        Option<Account<'info, locker::state::vesting_escrow_metadata::VestingEscrowMetadata>>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub event_authority: &'info AccountInfo<'info>,
    pub program: &'info AccountInfo<'info>,
}
impl<'info> CreateVestingEscrowSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let base: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("base".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("base".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("base".to_string()))?;
        let escrow: Option<
            anchor_lang::accounts::account::Account<locker::state::vesting_escrow::VestingEscrow>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "escrow".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let escrow_token: anchor_lang::accounts::account::Account<TokenAccount> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow_token".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("escrow_token".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow_token".to_string()))?;
        let sender: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("sender".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("sender".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("sender".to_string()))?;
        let sender_token: anchor_lang::accounts::account::Account<TokenAccount> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("sender_token".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("sender_token".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("sender_token".to_string()))?;
        let recipient = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("recipient".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::unchecked_account::UncheckedAccount::try_from)
            .ok_or(FuzzingError::AccountNotFound("recipient".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let event_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "event_authority".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("event_authority".to_string()))?;
        let program = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("program".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("program".to_string()))?;
        Ok(Self {
            base,
            escrow,
            escrow_token,
            sender,
            sender_token,
            recipient,
            token_program,
            system_program,
            event_authority,
            program,
        })
    }
}
impl<'info> ClaimSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let escrow: anchor_lang::accounts::account::Account<
            locker::state::vesting_escrow::VestingEscrow,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("escrow".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow".to_string()))?;
        let escrow_token: anchor_lang::accounts::account::Account<TokenAccount> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow_token".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("escrow_token".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow_token".to_string()))?;
        let recipient: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("recipient".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("recipient".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("recipient".to_string()))?;
        let recipient_token: anchor_lang::accounts::account::Account<TokenAccount> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "recipient_token".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("recipient_token".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("recipient_token".to_string()))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let event_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "event_authority".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("event_authority".to_string()))?;
        let program = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("program".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("program".to_string()))?;
        Ok(Self {
            escrow,
            escrow_token,
            recipient,
            recipient_token,
            token_program,
            event_authority,
            program,
        })
    }
}
impl<'info> CreateVestingEscrowMetadataSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let escrow: anchor_lang::accounts::account::Account<
            locker::state::vesting_escrow::VestingEscrow,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("escrow".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow".to_string()))?;
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
        let escrow_metadata: Option<
            anchor_lang::accounts::account::Account<
                locker::state::vesting_escrow_metadata::VestingEscrowMetadata,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "escrow_metadata".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("escrow_metadata".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "escrow_metadata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
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
            escrow,
            creator,
            escrow_metadata,
            payer,
            system_program,
        })
    }
}
impl<'info> UpdateVestingEscrowRecipientSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let escrow: anchor_lang::accounts::account::Account<
            locker::state::vesting_escrow::VestingEscrow,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("escrow".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("escrow".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("escrow".to_string()))?;
        let escrow_metadata: Option<
            anchor_lang::accounts::account::Account<
                locker::state::vesting_escrow_metadata::VestingEscrowMetadata,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "escrow_metadata".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("escrow_metadata".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "escrow_metadata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        let event_authority = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "event_authority".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("event_authority".to_string()))?;
        let program = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("program".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("program".to_string()))?;
        Ok(Self {
            escrow,
            escrow_metadata,
            signer,
            system_program,
            event_authority,
            program,
        })
    }
}
