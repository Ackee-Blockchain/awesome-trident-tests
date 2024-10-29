use anchor_lang::prelude::*;
use squads_multisig_program::state::ProgramConfig;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use std::convert::TryFrom;
pub struct ProgramConfigInitSnapshot<'info> {
    pub program_config: Option<
        Account<'info, ProgramConfig>,
    >,
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct ProgramConfigSetAuthoritySnapshot<'info> {
    pub program_config: Account<
        'info,
        ProgramConfig,
    >,
    pub authority: Signer<'info>,
}
pub struct MultisigCreateSnapshot<'info> {
    pub multisig: Option<Account<'info, squads_multisig_program::state::multisig::Multisig>>,
    pub create_key: Signer<'info>,
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct MultisigCreateV2Snapshot<'info> {
    pub program_config: Account<'info, ProgramConfig>,
    pub treasury: &'info AccountInfo<'info>,
    pub multisig: Option<Account<'info, squads_multisig_program::state::multisig::Multisig>>,
    pub create_key: Signer<'info>,
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct MultisigAddMemberSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub config_authority: Signer<'info>,
    pub rent_payer: Option<Signer<'info>>,
    pub system_program: Option<Program<'info, System>>,
}
pub struct MultisigAddSpendingLimitSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub config_authority: Signer<'info>,
    pub spending_limit:
        Option<Account<'info, squads_multisig_program::state::spending_limit::SpendingLimit>>,
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct MultisigRemoveSpendingLimitSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub config_authority: Signer<'info>,
    pub spending_limit:
        Option<Account<'info, squads_multisig_program::state::spending_limit::SpendingLimit>>,
    pub rent_collector: &'info AccountInfo<'info>,
}
pub struct ConfigTransactionCreateSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub transaction: Option<
        Account<'info, squads_multisig_program::state::config_transaction::ConfigTransaction>,
    >,
    pub creator: Signer<'info>,
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct VaultTransactionCreateSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub transaction:
        Option<Account<'info, squads_multisig_program::state::vault_transaction::VaultTransaction>>,
    pub creator: Signer<'info>,
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct VaultTransactionExecuteSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub proposal: Account<'info, squads_multisig_program::state::proposal::Proposal>,
    pub transaction:
        Account<'info, squads_multisig_program::state::vault_transaction::VaultTransaction>,
    pub member: Signer<'info>,
}
pub struct BatchCreateSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub batch: Option<Account<'info, squads_multisig_program::state::batch::Batch>>,
    pub creator: Signer<'info>,
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct BatchAddTransactionSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub proposal: Account<'info, squads_multisig_program::state::proposal::Proposal>,
    pub batch: Account<'info, squads_multisig_program::state::batch::Batch>,
    pub transaction:
        Option<Account<'info, squads_multisig_program::state::batch::VaultBatchTransaction>>,
    pub member: Signer<'info>,
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct BatchExecuteTransactionSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub member: Signer<'info>,
    pub proposal: Account<'info, squads_multisig_program::state::proposal::Proposal>,
    pub batch: Account<'info, squads_multisig_program::state::batch::Batch>,
    pub transaction: Account<'info, squads_multisig_program::state::batch::VaultBatchTransaction>,
}
pub struct ProposalCreateSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub proposal: Option<Account<'info, squads_multisig_program::state::proposal::Proposal>>,
    pub creator: Signer<'info>,
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct ProposalActivateSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub member: Signer<'info>,
    pub proposal: Account<'info, squads_multisig_program::state::proposal::Proposal>,
}
pub struct ProposalApproveSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub member: Signer<'info>,
    pub proposal: Account<'info, squads_multisig_program::state::proposal::Proposal>,
}
pub struct SpendingLimitUseSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub member: Signer<'info>,
    pub spending_limit:
        Account<'info, squads_multisig_program::state::spending_limit::SpendingLimit>,
    pub vault: &'info AccountInfo<'info>,
    pub destination: &'info AccountInfo<'info>,
    pub system_program: Option<Program<'info, System>>,
    pub mint: Option<InterfaceAccount<'info, Mint>>,
    pub vault_token_account: Option<InterfaceAccount<'info, TokenAccount>>,
    pub destination_token_account: Option<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Option<Interface<'info, TokenInterface>>,
}
pub struct ConfigTransactionAccountsCloseSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub proposal: Option<Account<'info, squads_multisig_program::state::proposal::Proposal>>,
    pub transaction: Option<
        Account<'info, squads_multisig_program::state::config_transaction::ConfigTransaction>,
    >,
    pub rent_collector: &'info AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
pub struct VaultTransactionAccountsCloseSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub proposal: Option<Account<'info, squads_multisig_program::state::proposal::Proposal>>,
    pub transaction:
        Option<Account<'info, squads_multisig_program::state::vault_transaction::VaultTransaction>>,
    pub rent_collector: &'info AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
pub struct VaultBatchTransactionAccountCloseSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub proposal: Account<'info, squads_multisig_program::state::proposal::Proposal>,
    pub batch: Account<'info, squads_multisig_program::state::batch::Batch>,
    pub transaction:
        Option<Account<'info, squads_multisig_program::state::batch::VaultBatchTransaction>>,
    pub rent_collector: &'info AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
pub struct BatchAccountsCloseSnapshot<'info> {
    pub multisig: Account<'info, squads_multisig_program::state::multisig::Multisig>,
    pub proposal: Option<Account<'info, squads_multisig_program::state::proposal::Proposal>>,
    pub batch: Option<Account<'info, squads_multisig_program::state::batch::Batch>>,
    pub rent_collector: &'info AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> ProgramConfigInitSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let program_config: Option<
            Account<
                ProgramConfig,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "program_config".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("program_config".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "program_config".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let initializer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("initializer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("initializer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("initializer".to_string()))?;
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
            program_config,
            initializer,
            system_program,
        })
    }
}
impl<'info> ProgramConfigSetAuthoritySnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let program_config: anchor_lang::accounts::account::Account<ProgramConfig> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "program_config".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("program_config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("program_config".to_string()))?;
        let authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("authority".to_string()))?;
        Ok(Self {
            program_config,
            authority,
        })
    }
}
impl<'info> MultisigCreateSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::multisig::Multisig,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "multisig".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let create_key: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("create_key".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("create_key".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("create_key".to_string()))?;
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
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
            multisig,
            create_key,
            creator,
            system_program,
        })
    }
}
impl<'info> MultisigCreateV2Snapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let program_config: Account<ProgramConfig> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "program_config".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("program_config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("program_config".to_string()))?;
        let treasury = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("treasury".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("treasury".to_string()))?;
        let multisig: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::multisig::Multisig,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "multisig".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let create_key: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("create_key".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("create_key".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("create_key".to_string()))?;
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
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
            program_config,
            treasury,
            multisig,
            create_key,
            creator,
            system_program,
        })
    }
}
impl<'info> MultisigAddMemberSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let config_authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "config_authority".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "config_authority".to_string(),
            ))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("config_authority".to_string()))?;
        let rent_payer: Option<Signer<'_>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent_payer".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::signer::Signer::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("rent_payer".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "rent_payer".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::program::Program::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("system_program".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "system_program".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        Ok(Self {
            multisig,
            config_authority,
            rent_payer,
            system_program,
        })
    }
}
impl<'info> MultisigAddSpendingLimitSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let config_authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "config_authority".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "config_authority".to_string(),
            ))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("config_authority".to_string()))?;
        let spending_limit: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::spending_limit::SpendingLimit,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "spending_limit".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("spending_limit".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "spending_limit".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let rent_payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent_payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("rent_payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent_payer".to_string()))?;
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
            multisig,
            config_authority,
            spending_limit,
            rent_payer,
            system_program,
        })
    }
}
impl<'info> MultisigRemoveSpendingLimitSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let config_authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "config_authority".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "config_authority".to_string(),
            ))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("config_authority".to_string()))?;
        let spending_limit: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::spending_limit::SpendingLimit,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "spending_limit".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("spending_limit".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "spending_limit".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let rent_collector = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "rent_collector".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("rent_collector".to_string()))?;
        Ok(Self {
            multisig,
            config_authority,
            spending_limit,
            rent_collector,
        })
    }
}
impl<'info> ConfigTransactionCreateSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let transaction: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::config_transaction::ConfigTransaction,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("transaction".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "transaction".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
        let rent_payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent_payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("rent_payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent_payer".to_string()))?;
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
            multisig,
            transaction,
            creator,
            rent_payer,
            system_program,
        })
    }
}
impl<'info> VaultTransactionCreateSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let transaction: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::vault_transaction::VaultTransaction,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("transaction".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "transaction".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
        let rent_payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent_payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("rent_payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent_payer".to_string()))?;
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
            multisig,
            transaction,
            creator,
            rent_payer,
            system_program,
        })
    }
}
impl<'info> VaultTransactionExecuteSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let proposal: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::proposal::Proposal,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("proposal".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))?;
        let transaction: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::vault_transaction::VaultTransaction,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("transaction".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("transaction".to_string()))?;
        let member: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("member".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("member".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("member".to_string()))?;
        Ok(Self {
            multisig,
            proposal,
            transaction,
            member,
        })
    }
}
impl<'info> BatchCreateSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let batch: Option<
            anchor_lang::accounts::account::Account<squads_multisig_program::state::batch::Batch>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("batch".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("batch".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "batch".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
        let rent_payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent_payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("rent_payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent_payer".to_string()))?;
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
            multisig,
            batch,
            creator,
            rent_payer,
            system_program,
        })
    }
}
impl<'info> BatchAddTransactionSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let proposal: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::proposal::Proposal,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("proposal".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))?;
        let batch: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::batch::Batch,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("batch".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("batch".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("batch".to_string()))?;
        let transaction: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::batch::VaultBatchTransaction,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("transaction".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "transaction".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let member: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("member".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("member".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("member".to_string()))?;
        let rent_payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent_payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("rent_payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent_payer".to_string()))?;
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
            multisig,
            proposal,
            batch,
            transaction,
            member,
            rent_payer,
            system_program,
        })
    }
}
impl<'info> BatchExecuteTransactionSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let member: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("member".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("member".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("member".to_string()))?;
        let proposal: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::proposal::Proposal,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("proposal".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))?;
        let batch: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::batch::Batch,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("batch".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("batch".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("batch".to_string()))?;
        let transaction: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::batch::VaultBatchTransaction,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("transaction".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("transaction".to_string()))?;
        Ok(Self {
            multisig,
            member,
            proposal,
            batch,
            transaction,
        })
    }
}
impl<'info> ProposalCreateSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let proposal: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::proposal::Proposal,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "proposal".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let creator: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("creator".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("creator".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("creator".to_string()))?;
        let rent_payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("rent_payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("rent_payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("rent_payer".to_string()))?;
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
            multisig,
            proposal,
            creator,
            rent_payer,
            system_program,
        })
    }
}
impl<'info> ProposalActivateSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let member: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("member".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("member".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("member".to_string()))?;
        let proposal: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::proposal::Proposal,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("proposal".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))?;
        Ok(Self {
            multisig,
            member,
            proposal,
        })
    }
}
impl<'info> ProposalApproveSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let member: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("member".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("member".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("member".to_string()))?;
        let proposal: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::proposal::Proposal,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("proposal".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))?;
        Ok(Self {
            multisig,
            member,
            proposal,
        })
    }
}
impl<'info> SpendingLimitUseSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let member: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("member".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("member".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("member".to_string()))?;
        let spending_limit: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::spending_limit::SpendingLimit,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "spending_limit".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("spending_limit".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("spending_limit".to_string()))?;
        let vault = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("vault".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("vault".to_string()))?;
        let destination = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("destination".to_string()))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("destination".to_string()))?;
        let system_program: Option<anchor_lang::accounts::program::Program<System>> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::program::Program::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("system_program".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "system_program".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let mint: Option<InterfaceAccount<Mint>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        InterfaceAccount::try_from(acc)
                            .map_err(|_| FuzzingError::CannotDeserializeAccount("mint".to_string()))
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided("mint".to_string()))
                    }
                })
                .transpose()
                .unwrap_or(None);
        let vault_token_account: Option<
            InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "vault_token_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount(
                                "vault_token_account".to_string(),
                            )
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "vault_token_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let destination_token_account: Option<
            InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "destination_token_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount(
                                "destination_token_account".to_string(),
                            )
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "destination_token_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let token_program: Option<anchor_lang::accounts::interface::Interface<TokenInterface>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        anchor_lang::accounts::interface::Interface::try_from(acc).map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("token_program".to_string())
                        })
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided(
                            "token_program".to_string(),
                        ))
                    }
                })
                .transpose()
                .unwrap_or(None);
        Ok(Self {
            multisig,
            member,
            spending_limit,
            vault,
            destination,
            system_program,
            mint,
            vault_token_account,
            destination_token_account,
            token_program,
        })
    }
}
impl<'info> ConfigTransactionAccountsCloseSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let proposal: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::proposal::Proposal,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "proposal".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let transaction: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::config_transaction::ConfigTransaction,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("transaction".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "transaction".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let rent_collector = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "rent_collector".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("rent_collector".to_string()))?;
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
            multisig,
            proposal,
            transaction,
            rent_collector,
            system_program,
        })
    }
}
impl<'info> VaultTransactionAccountsCloseSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let proposal: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::proposal::Proposal,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "proposal".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let transaction: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::vault_transaction::VaultTransaction,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("transaction".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "transaction".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let rent_collector = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "rent_collector".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("rent_collector".to_string()))?;
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
            multisig,
            proposal,
            transaction,
            rent_collector,
            system_program,
        })
    }
}
impl<'info> VaultBatchTransactionAccountCloseSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let proposal: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::proposal::Proposal,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("proposal".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))?;
        let batch: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::batch::Batch,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("batch".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("batch".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("batch".to_string()))?;
        let transaction: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::batch::VaultBatchTransaction,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("transaction".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("transaction".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "transaction".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let rent_collector = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "rent_collector".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("rent_collector".to_string()))?;
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
            multisig,
            proposal,
            batch,
            transaction,
            rent_collector,
            system_program,
        })
    }
}
impl<'info> BatchAccountsCloseSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let multisig: anchor_lang::accounts::account::Account<
            squads_multisig_program::state::multisig::Multisig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("multisig".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("multisig".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("multisig".to_string()))?;
        let proposal: Option<
            anchor_lang::accounts::account::Account<
                squads_multisig_program::state::proposal::Proposal,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("proposal".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("proposal".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "proposal".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let batch: Option<
            anchor_lang::accounts::account::Account<squads_multisig_program::state::batch::Batch>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("batch".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("batch".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "batch".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let rent_collector = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "rent_collector".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound("rent_collector".to_string()))?;
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
            multisig,
            proposal,
            batch,
            rent_collector,
            system_program,
        })
    }
}
pub type ProgramConfigSetMultisigCreationFeeSnapshot<'info> =
    ProgramConfigSetAuthoritySnapshot<'info>;
pub type ProgramConfigSetTreasurySnapshot<'info> = ProgramConfigSetAuthoritySnapshot<'info>;
pub type MultisigRemoveMemberSnapshot<'info> = MultisigAddMemberSnapshot<'info>;
pub type MultisigSetTimeLockSnapshot<'info> = MultisigAddMemberSnapshot<'info>;
pub type MultisigChangeThresholdSnapshot<'info> = MultisigAddMemberSnapshot<'info>;
pub type MultisigSetConfigAuthoritySnapshot<'info> = MultisigAddMemberSnapshot<'info>;
pub type MultisigSetRentCollectorSnapshot<'info> = MultisigAddMemberSnapshot<'info>;
pub type ProposalRejectSnapshot<'info> = ProposalApproveSnapshot<'info>;
pub type ProposalCancelSnapshot<'info> = ProposalApproveSnapshot<'info>;
