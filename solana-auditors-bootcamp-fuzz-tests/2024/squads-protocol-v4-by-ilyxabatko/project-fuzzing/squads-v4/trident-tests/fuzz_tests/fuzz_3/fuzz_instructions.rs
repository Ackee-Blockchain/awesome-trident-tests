pub mod squads_multisig_program_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::{message::{v0::Message, VersionedMessage}, native_token::LAMPORTS_PER_SOL, system_instruction::transfer};
    use squads_multisig_program::{SEED_MULTISIG, SEED_PREFIX, SEED_PROGRAM_CONFIG, SEED_PROPOSAL, SEED_SPENDING_LIMIT, SEED_TRANSACTION, SEED_VAULT};
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        ProgramConfigInit(ProgramConfigInit),
        ProgramConfigSetAuthority(ProgramConfigSetAuthority),
        ProgramConfigSetMultisigCreationFee(ProgramConfigSetMultisigCreationFee),
        ProgramConfigSetTreasury(ProgramConfigSetTreasury),
        MultisigCreateV2(MultisigCreateV2),
        MultisigAddMember(MultisigAddMember),
        MultisigRemoveMember(MultisigRemoveMember),
        MultisigSetTimeLock(MultisigSetTimeLock),
        MultisigChangeThreshold(MultisigChangeThreshold),
        MultisigSetConfigAuthority(MultisigSetConfigAuthority),
        MultisigSetRentCollector(MultisigSetRentCollector),
        MultisigAddSpendingLimit(MultisigAddSpendingLimit),
        MultisigRemoveSpendingLimit(MultisigRemoveSpendingLimit),
        ConfigTransactionCreate(ConfigTransactionCreate),
        VaultTransactionCreate(VaultTransactionCreate),
        VaultTransactionExecute(VaultTransactionExecute),
        BatchCreate(BatchCreate),
        BatchAddTransaction(BatchAddTransaction),
        BatchExecuteTransaction(BatchExecuteTransaction),
        ProposalCreate(ProposalCreate),
        ProposalActivate(ProposalActivate),
        ProposalApprove(ProposalApprove),
        ProposalReject(ProposalReject),
        ProposalCancel(ProposalCancel),
        SpendingLimitUse(SpendingLimitUse),
        ConfigTransactionAccountsClose(ConfigTransactionAccountsClose),
        VaultTransactionAccountsClose(VaultTransactionAccountsClose),
        VaultBatchTransactionAccountClose(VaultBatchTransactionAccountClose),
        BatchAccountsClose(BatchAccountsClose),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Permissions {
        pub mask: u8,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Member {
        pub key: AccountId,
        pub permissions: Permissions,
    }
    #[derive(Arbitrary, Debug, Clone)]
    pub enum Period {
        OneTime = 0,
        Day = 1,
        Week = 2,
        Month = 3,
    }
    #[derive(Arbitrary, Debug)]
    pub enum ConfigAction {
        AddMember { new_member: Member },
        RemoveMember { old_member: AccountId },
        ChangeThreshold { new_threshold: u16 },
        SetTimeLock { new_time_lock: u32 },
        AddSpendingLimit {
            create_key: AccountId,
            vault_index: u8,
            mint: AccountId,
            amount: u64,
            period: Period,
            members: Vec<AccountId>,
            destinations: Vec<AccountId>,
        },
        RemoveSpendingLimit { spending_limit: AccountId },
        SetRentCollector { new_rent_collector: Option<AccountId> },
    }
    
    // Program Config Init
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigInit {
        pub accounts: ProgramConfigInitAccounts,
        pub data: ProgramConfigInitData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigInitAccounts {
        pub program_config: AccountId,
        pub initializer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigInitArgs {
        authority: AccountId,
        multisig_creation_fee: u64,
        treasury: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigInitData {
        args: ProgramConfigInitArgs
    }

    // Program Config Set Authority
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetAuthority {
        pub accounts: ProgramConfigSetAuthorityAccounts,
        pub data: ProgramConfigSetAuthorityData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetAuthorityAccounts {
        pub program_config: AccountId,
        pub authority: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetAuthorityData {
        new_authority: AccountId
    }

    // Program Config Set Multisig Creation Fee
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetMultisigCreationFee {
        pub accounts: ProgramConfigSetMultisigCreationFeeAccounts,
        pub data: ProgramConfigSetMultisigCreationFeeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetMultisigCreationFeeAccounts {
        pub program_config: AccountId,
        pub authority: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetMultisigCreationFeeData { new_multisig_creation_fee: u64 }

    // Program Config Set Treasury
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetTreasury {
        pub accounts: ProgramConfigSetTreasuryAccounts,
        pub data: ProgramConfigSetTreasuryData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetTreasuryAccounts {
        pub program_config: AccountId,
        pub authority: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProgramConfigSetTreasuryData {
        new_treasury: AccountId
    }

    // Multisig Create v2
    #[derive(Arbitrary, Debug)]
    pub struct MultisigCreateV2 {
        pub accounts: MultisigCreateV2Accounts,
        pub data: MultisigCreateV2Data,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigCreateV2Accounts {
        pub program_config: AccountId,
        pub treasury: AccountId,
        pub multisig: AccountId,
        pub create_key: AccountId,
        pub creator: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigCreateV2Data {      
        config_authority: Option<AccountId>,
        threshold: u16,
        members: Vec<Member>,
        time_lock: u32,
        rent_collector: Option<AccountId>,
        memo: Option<String>,
    }

    // Multisig Add Member
    #[derive(Arbitrary, Debug)]
    pub struct MultisigAddMember {
        pub accounts: MultisigAddMemberAccounts,
        pub data: MultisigAddMemberData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigAddMemberAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigAddMemberData {
        new_member: Member,
        memo: Option<String>,
    }

    // Multisig Remove Member
    #[derive(Arbitrary, Debug)]
    pub struct MultisigRemoveMember {
        pub accounts: MultisigRemoveMemberAccounts,
        pub data: MultisigRemoveMemberData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigRemoveMemberAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigRemoveMemberData {
        old_member: AccountId,
        memo: Option<String>,
    }

    // Set Time Lock
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetTimeLock {
        pub accounts: MultisigSetTimeLockAccounts,
        pub data: MultisigSetTimeLockData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetTimeLockAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetTimeLockData {
        time_lock: u32,
        memo: Option<String>,
    }

    // Multisig Change Threshold
    #[derive(Arbitrary, Debug)]
    pub struct MultisigChangeThreshold {
        pub accounts: MultisigChangeThresholdAccounts,
        pub data: MultisigChangeThresholdData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigChangeThresholdAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigChangeThresholdData {
        new_threshold: u16,
        memo: Option<String>,
    }

    // Multisig Set Config Authority
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetConfigAuthority {
        pub accounts: MultisigSetConfigAuthorityAccounts,
        pub data: MultisigSetConfigAuthorityData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetConfigAuthorityAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetConfigAuthorityData {
        config_authority: AccountId,
        memo: Option<String>,
    }

    // Multisig Set Rent Collector
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetRentCollector {
        pub accounts: MultisigSetRentCollectorAccounts,
        pub data: MultisigSetRentCollectorData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetRentCollectorAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigSetRentCollectorData {
        rent_collector: Option<AccountId>,
        memo: Option<String>,
    }

    // Multisig Add Spending Limit
    #[derive(Arbitrary, Debug)]
    pub struct MultisigAddSpendingLimit {
        pub accounts: MultisigAddSpendingLimitAccounts,
        pub data: MultisigAddSpendingLimitData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigAddSpendingLimitAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub spending_limit: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigAddSpendingLimitData { 
        create_key: AccountId,
        vault_index: u8,
        mint: AccountId,
        amount: u64,
        period: Period,
        members: Vec<AccountId>,
        _destinations: Vec<AccountId>,
        memo: Option<String>,
    }

    // Multisig Remove Spending Limit
    #[derive(Arbitrary, Debug)]
    pub struct MultisigRemoveSpendingLimit {
        pub accounts: MultisigRemoveSpendingLimitAccounts,
        pub data: MultisigRemoveSpendingLimitData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigRemoveSpendingLimitAccounts {
        pub multisig: AccountId,
        pub config_authority: AccountId,
        pub spending_limit: AccountId,
        pub rent_collector: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MultisigRemoveSpendingLimitData { memo: Option<String> }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigTransactionCreate {
        pub accounts: ConfigTransactionCreateAccounts,
        pub data: ConfigTransactionCreateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigTransactionCreateAccounts {
        pub multisig: AccountId,
        pub transaction: AccountId,
        pub creator: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigTransactionCreateData { 
        actions: Vec<ConfigAction>,
        memo: Option<String> 
    }

    // Vault Transaction Create
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionCreate {
        pub accounts: VaultTransactionCreateAccounts,
        pub data: VaultTransactionCreateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionCreateAccounts {
        pub multisig: AccountId,
        pub transaction: AccountId,
        pub creator: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionCreateData { 
        vault_index: u8,
        ephemeral_signers: u8,
        _transaction_message: Vec<u8>,
        memo: Option<String>,
     }

    // Vault Transaction Execute
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionExecute {
        pub accounts: VaultTransactionExecuteAccounts,
        pub data: VaultTransactionExecuteData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionExecuteAccounts {
        pub multisig: AccountId,
        pub proposal: AccountId,
        pub transaction: AccountId,
        pub member: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionExecuteData {}

    // Batch Create
    #[derive(Arbitrary, Debug)]
    pub struct BatchCreate {
        pub accounts: BatchCreateAccounts,
        pub data: BatchCreateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchCreateAccounts {
        pub multisig: AccountId,
        pub batch: AccountId,
        pub creator: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchCreateData {
        vault_index: u8,
        memo: Option<String>,
    }

    // Batch Add Transaction
    #[derive(Arbitrary, Debug)]
    pub struct BatchAddTransaction {
        pub accounts: BatchAddTransactionAccounts,
        pub data: BatchAddTransactionData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchAddTransactionAccounts {
        pub multisig: AccountId,
        pub proposal: AccountId,
        pub batch: AccountId,
        pub transaction: AccountId,
        pub member: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchAddTransactionData {
        ephemeral_signers: u8,
        transaction_message: Vec<u8>,
    }

    // Batch Execute Tx
    #[derive(Arbitrary, Debug)]
    pub struct BatchExecuteTransaction {
        pub accounts: BatchExecuteTransactionAccounts,
        pub data: BatchExecuteTransactionData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchExecuteTransactionAccounts {
        pub multisig: AccountId,
        pub member: AccountId,
        pub proposal: AccountId,
        pub batch: AccountId,
        pub transaction: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchExecuteTransactionData {}

    // Proposal Create
    #[derive(Arbitrary, Debug)]
    pub struct ProposalCreate {
        pub accounts: ProposalCreateAccounts,
        pub data: ProposalCreateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalCreateAccounts {
        pub multisig: AccountId,
        pub proposal: AccountId,
        pub creator: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalCreateData {
        transaction_index: u64,
        draft: bool,
    }

    // ProposalActivate
    #[derive(Arbitrary, Debug)]
    pub struct ProposalActivate {
        pub accounts: ProposalActivateAccounts,
        pub data: ProposalActivateData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalActivateAccounts {
        pub multisig: AccountId,
        pub member: AccountId,
        pub proposal: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalActivateData {}

    // Proposal Approve
    #[derive(Arbitrary, Debug)]
    pub struct ProposalApprove {
        pub accounts: ProposalApproveAccounts,
        pub data: ProposalApproveData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalApproveAccounts {
        pub multisig: AccountId,
        pub member: AccountId,
        pub proposal: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalApproveData {
        memo: Option<String>,
    }

    // Proposal Reject
    #[derive(Arbitrary, Debug)]
    pub struct ProposalReject {
        pub accounts: ProposalRejectAccounts,
        pub data: ProposalRejectData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalRejectAccounts {
        pub multisig: AccountId,
        pub member: AccountId,
        pub proposal: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalRejectData {
        memo: Option<String>,
    }

    // Proposal Cancel
    #[derive(Arbitrary, Debug)]
    pub struct ProposalCancel {
        pub accounts: ProposalCancelAccounts,
        pub data: ProposalCancelData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalCancelAccounts {
        pub multisig: AccountId,
        pub member: AccountId,
        pub proposal: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ProposalCancelData {
        memo: Option<String>,
    }

    // Spending Limit Use
    #[derive(Arbitrary, Debug)]
    pub struct SpendingLimitUse {
        pub accounts: SpendingLimitUseAccounts,
        pub data: SpendingLimitUseData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SpendingLimitUseAccounts {
        pub multisig: AccountId,
        pub member: AccountId,
        pub spending_limit: AccountId,
        pub vault: AccountId,
        pub destination: AccountId,
        pub system_program: AccountId,
        pub mint: AccountId,
        pub vault_token_account: AccountId,
        pub destination_token_account: AccountId,
        pub token_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SpendingLimitUseData {
        amount: u64,
        decimals: u8,
        memo: Option<String>,
    }

    // Config Transaction Accounts Close
    #[derive(Arbitrary, Debug)]
    pub struct ConfigTransactionAccountsClose {
        pub accounts: ConfigTransactionAccountsCloseAccounts,
        pub data: ConfigTransactionAccountsCloseData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigTransactionAccountsCloseAccounts {
        pub multisig: AccountId,
        pub proposal: AccountId,
        pub transaction: AccountId,
        pub rent_collector: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigTransactionAccountsCloseData {}

    // Vault Transaction Accounts Close
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionAccountsClose {
        pub accounts: VaultTransactionAccountsCloseAccounts,
        pub data: VaultTransactionAccountsCloseData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionAccountsCloseAccounts {
        pub multisig: AccountId,
        pub proposal: AccountId,
        pub transaction: AccountId,
        pub rent_collector: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultTransactionAccountsCloseData {}

    // Vault Batch Transaction Account Close
    #[derive(Arbitrary, Debug)]
    pub struct VaultBatchTransactionAccountClose {
        pub accounts: VaultBatchTransactionAccountCloseAccounts,
        pub data: VaultBatchTransactionAccountCloseData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultBatchTransactionAccountCloseAccounts {
        pub multisig: AccountId,
        pub proposal: AccountId,
        pub batch: AccountId,
        pub transaction: AccountId,
        pub rent_collector: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct VaultBatchTransactionAccountCloseData {}

    // Batch Accounts Close
    #[derive(Arbitrary, Debug)]
    pub struct BatchAccountsClose {
        pub accounts: BatchAccountsCloseAccounts,
        pub data: BatchAccountsCloseData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchAccountsCloseAccounts {
        pub multisig: AccountId,
        pub proposal: AccountId,
        pub batch: AccountId,
        pub rent_collector: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct BatchAccountsCloseData {}

    impl<'info> IxOps<'info> for ProgramConfigInit {
        type IxData = squads_multisig_program::instruction::ProgramConfigInit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProgramConfigInitSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let authority = fuzz_accounts.authority.get_or_create_account(self.data.args.authority, client, 5 * LAMPORTS_PER_SOL);
            let treasury = fuzz_accounts.treasury.get_or_create_account(self.data.args.treasury, client, 5 * LAMPORTS_PER_SOL);
            let data = squads_multisig_program::instruction::ProgramConfigInit {
                args: squads_multisig_program::instructions::program_config_init::ProgramConfigInitArgs {
                    authority: authority.pubkey(),
                    multisig_creation_fee: self.data.args.multisig_creation_fee,
                    treasury: treasury.pubkey()
                }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.initializer.get_or_create_account(self.accounts.initializer, client, 5 * LAMPORTS_PER_SOL);
            let signers = vec![signer.clone()];
            let program_config = fuzz_accounts.program_config.get_or_create_account(self.accounts.program_config, &[SEED_PREFIX, SEED_PROGRAM_CONFIG], &squads_multisig_program::ID).unwrap();

            let acc_meta = squads_multisig_program::accounts::ProgramConfigInit {
                program_config: program_config.pubkey,
                initializer: signer.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProgramConfigSetAuthority {
        type IxData = squads_multisig_program::instruction::ProgramConfigSetAuthority;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProgramConfigSetAuthoritySnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let new_authority = fuzz_accounts.initializer.get_or_create_account(self.data.new_authority, client, 5 * LAMPORTS_PER_SOL);
            let data =
                squads_multisig_program::instruction::ProgramConfigSetAuthority { 
                    args: squads_multisig_program::ProgramConfigSetAuthorityArgs { 
                        new_authority: new_authority.pubkey()
                    } 
                };

            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let program_config = fuzz_accounts.program_config.get_or_create_account(self.accounts.program_config, &[SEED_PREFIX, SEED_PROGRAM_CONFIG], &squads_multisig_program::ID).unwrap();
            let authority = fuzz_accounts.initializer.get_or_create_account(self.accounts.authority, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![authority.clone()];
            let acc_meta = squads_multisig_program::accounts::ProgramConfig {
                program_config: program_config.pubkey,
                authority: authority.pubkey(),
            }
            .to_account_metas(None);

            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProgramConfigSetMultisigCreationFee {
        type IxData = squads_multisig_program::instruction::ProgramConfigSetMultisigCreationFee;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProgramConfigSetMultisigCreationFeeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::ProgramConfigSetMultisigCreationFee {
                args: squads_multisig_program::ProgramConfigSetMultisigCreationFeeArgs {
                    new_multisig_creation_fee: self.data.new_multisig_creation_fee
                }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let program_config = fuzz_accounts.program_config.get_or_create_account(self.accounts.program_config, &[SEED_PREFIX, SEED_PROGRAM_CONFIG], &squads_multisig_program::ID).unwrap();
            let authority = fuzz_accounts.initializer.get_or_create_account(self.accounts.authority, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![authority.clone()];
            let acc_meta = squads_multisig_program::accounts::ProgramConfig {
                program_config: program_config.pubkey,
                authority: authority.pubkey(),
            }
            .to_account_metas(None);

            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProgramConfigSetTreasury {
        type IxData = squads_multisig_program::instruction::ProgramConfigSetTreasury;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProgramConfigSetTreasurySnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let treasury = fuzz_accounts.treasury.get_or_create_account(self.data.new_treasury, client, 5 * LAMPORTS_PER_SOL);
            let data =
                squads_multisig_program::instruction::ProgramConfigSetTreasury { 
                    args: squads_multisig_program::ProgramConfigSetTreasuryArgs {
                        new_treasury: treasury.pubkey()
                    }
                };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let program_config = fuzz_accounts.program_config.get_or_create_account(self.accounts.program_config, &[SEED_PREFIX, SEED_PROGRAM_CONFIG], &squads_multisig_program::ID).unwrap();
            let authority = fuzz_accounts.initializer.get_or_create_account(self.accounts.authority, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![authority.clone()];
            let acc_meta = squads_multisig_program::accounts::ProgramConfig {
                program_config: program_config.pubkey,
                authority: authority.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigCreateV2 {
        type IxData = squads_multisig_program::instruction::MultisigCreateV2;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigCreateV2Snapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let config_authority_pubkey_opt = match self.data.config_authority {
                Some(id) => Some(fuzz_accounts.config_authority.get_or_create_account(id, client, 5 * LAMPORTS_PER_SOL)),
                None => None
            };

            let rent_collector_pubkey_opt = match self.data.rent_collector {
                Some(id) => Some(fuzz_accounts.rent_collector.get_or_create_account(id, client, 5 * LAMPORTS_PER_SOL)),
                None => None
            };

            let members = self.data.members.iter().map(|member| {
                squads_multisig_program::Member {
                    key: fuzz_accounts.member.get_or_create_account(member.key, client, 5 * LAMPORTS_PER_SOL).pubkey(),
                    permissions: squads_multisig_program::Permissions { mask: member.permissions.mask }
                }
            }).collect();

            let data = squads_multisig_program::instruction::MultisigCreateV2 { 
                args: squads_multisig_program::MultisigCreateArgsV2 { 
                    config_authority: match config_authority_pubkey_opt {
                        Some(acc) => Some(acc.pubkey()),
                        None => None
                    }, 
                    threshold: self.data.threshold, 
                    members: members, 
                    time_lock: self.data.time_lock, 
                    rent_collector: match rent_collector_pubkey_opt {
                        Some(acc) => Some(acc.pubkey()),
                        None => None
                    }, 
                    memo: self.data.memo.clone()
                } 
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let program_config = fuzz_accounts.program_config.get_or_create_account(self.accounts.program_config, &[SEED_PREFIX, SEED_PROGRAM_CONFIG], &squads_multisig_program::ID).unwrap();
            let creator = fuzz_accounts.creator.get_or_create_account(self.accounts.creator, client, 5 * LAMPORTS_PER_SOL);
            let create_key = fuzz_accounts.create_key.get_or_create_account(self.accounts.create_key, client, 1 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let treasury = fuzz_accounts.treasury.get_or_create_account(self.accounts.treasury, client, 1 * LAMPORTS_PER_SOL);

            let signers = vec![creator.clone(), create_key.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigCreateV2 {
                program_config: program_config.pubkey,
                treasury: treasury.pubkey(),
                multisig: multisig.pubkey,
                create_key: create_key.pubkey(),
                creator: creator.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigAddMember {
        type IxData = squads_multisig_program::instruction::MultisigAddMember;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigAddMemberSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::MultisigAddMember { 
                args: squads_multisig_program::MultisigAddMemberArgs { 
                    new_member: squads_multisig_program::Member { 
                        key: fuzz_accounts.member.get_or_create_account(self.data.new_member.key, client, 5 * LAMPORTS_PER_SOL).pubkey(),
                        permissions: squads_multisig_program::Permissions { 
                            mask: self.data.new_member.permissions.mask
                        } 
                    }, 
                    memo: self.data.memo.clone()
                }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, Keypair::new().pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 10 * LAMPORTS_PER_SOL);

            let signers = vec![rent_payer.clone(), config_authority.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigConfig {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                rent_payer: Some(rent_payer.pubkey()),
                system_program: Some(anchor_lang::system_program::ID),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigRemoveMember {
        type IxData = squads_multisig_program::instruction::MultisigRemoveMember;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigRemoveMemberSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::MultisigRemoveMember { 
                args: squads_multisig_program::MultisigRemoveMemberArgs { 
                    old_member: fuzz_accounts.member.get_or_create_account(self.data.old_member, client, 5 * LAMPORTS_PER_SOL).pubkey(), 
                    memo: self.data.memo.clone() 
                } 
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, Keypair::new().pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 10 * LAMPORTS_PER_SOL);

            let signers = vec![rent_payer.clone(), config_authority.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigConfig {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                rent_payer: Some(rent_payer.pubkey()),
                system_program: Some(anchor_lang::system_program::ID),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigSetTimeLock {
        type IxData = squads_multisig_program::instruction::MultisigSetTimeLock;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigSetTimeLockSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::MultisigSetTimeLock { 
                args: squads_multisig_program::MultisigSetTimeLockArgs { 
                    time_lock: self.data.time_lock,
                    memo: self.data.memo.clone() 
                } 
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, Keypair::new().pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 10 * LAMPORTS_PER_SOL);

            let signers = vec![rent_payer.clone(), config_authority.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigConfig {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                rent_payer: Some(rent_payer.pubkey()),
                system_program: Some(anchor_lang::system_program::ID),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigChangeThreshold {
        type IxData = squads_multisig_program::instruction::MultisigChangeThreshold;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigChangeThresholdSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data =
                squads_multisig_program::instruction::MultisigChangeThreshold { 
                    args: squads_multisig_program::MultisigChangeThresholdArgs { 
                        new_threshold: self.data.new_threshold,
                        memo: self.data.memo.clone() 
                    }
                };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, Keypair::new().pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 10 * LAMPORTS_PER_SOL);

            let signers = vec![rent_payer.clone(), config_authority.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigConfig {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                rent_payer: Some(rent_payer.pubkey()),
                system_program: Some(anchor_lang::system_program::ID),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigSetConfigAuthority {
        type IxData = squads_multisig_program::instruction::MultisigSetConfigAuthority;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigSetConfigAuthoritySnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.data.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let data =
                squads_multisig_program::instruction::MultisigSetConfigAuthority { 
                    args: squads_multisig_program::MultisigSetConfigAuthorityArgs { 
                        config_authority: config_authority.pubkey(), 
                        memo: self.data.memo.clone()
                    }
                };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, Keypair::new().pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 10 * LAMPORTS_PER_SOL);

            let signers = vec![rent_payer.clone(), config_authority.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigConfig {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                rent_payer: Some(rent_payer.pubkey()),
                system_program: Some(anchor_lang::system_program::ID),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigSetRentCollector {
        type IxData = squads_multisig_program::instruction::MultisigSetRentCollector;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigSetRentCollectorSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let rent_collector = match self.data.rent_collector {
                Some(id) => Some(fuzz_accounts.rent_collector.get_or_create_account(id, client, 5 * LAMPORTS_PER_SOL).pubkey()),
                None => None
            };
            let data =
                squads_multisig_program::instruction::MultisigSetRentCollector { 
                    args: squads_multisig_program::MultisigSetRentCollectorArgs { 
                        rent_collector, 
                        memo: self.data.memo.clone()
                    } 
                };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, Keypair::new().pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 10 * LAMPORTS_PER_SOL);

            let signers = vec![rent_payer.clone(), config_authority.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigConfig {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                rent_payer: Some(rent_payer.pubkey()),
                system_program: Some(anchor_lang::system_program::ID),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigAddSpendingLimit {
        type IxData = squads_multisig_program::instruction::MultisigAddSpendingLimit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigAddSpendingLimitSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let create_key = fuzz_accounts.create_key.get_or_create_account(self.data.create_key, client, 5 * LAMPORTS_PER_SOL);
            let mint = fuzz_accounts.mint.get_or_create_account(self.data.mint, client, 6, &Keypair::new().pubkey(), None).unwrap();

            let members_vec = self.data.members.iter().map(|acc_id| {
                fuzz_accounts.member.get_or_create_account(*acc_id, client, 5 * LAMPORTS_PER_SOL).pubkey()
            }).collect();

            let data =
                squads_multisig_program::instruction::MultisigAddSpendingLimit { 
                    args: squads_multisig_program::MultisigAddSpendingLimitArgs { 
                        create_key: create_key.pubkey(), 
                        vault_index: self.data.vault_index, 
                        mint,
                        amount: self.data.amount, 
                        period: squads_multisig_program::Period::from(self.data.period.clone()), 
                        members: members_vec, 
                        destinations: vec![], 
                        memo: self.data.memo.clone() 
                    }
                };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = fuzz_accounts.create_key.get_or_create_account(self.data.create_key, client, 5 * LAMPORTS_PER_SOL);
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let spending_limit = fuzz_accounts.spending_limit.get_or_create_account(self.accounts.spending_limit, &[
                SEED_PREFIX,
                multisig.pubkey.as_ref(),
                SEED_SPENDING_LIMIT,
                create_key.pubkey().as_ref()
            ], &squads_multisig_program::ID).unwrap();
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![config_authority.clone(), rent_payer.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigAddSpendingLimit {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                spending_limit: spending_limit.pubkey,
                rent_payer: rent_payer.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MultisigRemoveSpendingLimit {
        type IxData = squads_multisig_program::instruction::MultisigRemoveSpendingLimit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MultisigRemoveSpendingLimitSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data =
                squads_multisig_program::instruction::MultisigRemoveSpendingLimit { 
                    args: squads_multisig_program::MultisigRemoveSpendingLimitArgs { memo: self.data.memo.clone() }
                };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let config_authority = fuzz_accounts.config_authority.get_or_create_account(self.accounts.config_authority, client, 5 * LAMPORTS_PER_SOL);
            let spending_limit = fuzz_accounts.spending_limit.get_or_create_account(self.accounts.spending_limit, &[ 
                    SEED_PREFIX,
                    multisig.pubkey().as_ref(),
                    SEED_VAULT,
                    &Keypair::new().pubkey().as_ref()
                ], &squads_multisig_program::ID).unwrap();            

            let rent_collector = fuzz_accounts.rent_collector.get_or_create_account(self.accounts.rent_collector, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![config_authority.clone()];
            let acc_meta = squads_multisig_program::accounts::MultisigRemoveSpendingLimit {
                multisig: multisig.pubkey,
                config_authority: config_authority.pubkey(),
                spending_limit: spending_limit.pubkey,
                rent_collector: rent_collector.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ConfigTransactionCreate {
        type IxData = squads_multisig_program::instruction::ConfigTransactionCreate;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ConfigTransactionCreateSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let actions = self.data.actions.iter().map(|action| {
                match action {
                    // AddMember
                    ConfigAction::AddMember { new_member } => squads_multisig_program::ConfigAction::AddMember { 
                        new_member: squads_multisig_program::Member { 
                            key: fuzz_accounts.member.get_or_create_account(new_member.key, client, 5 * LAMPORTS_PER_SOL).pubkey(), 
                            permissions: squads_multisig_program::Permissions { 
                                mask: new_member.permissions.mask 
                            } 
                        } 
                    },

                    // RemoveMember
                    ConfigAction::RemoveMember { old_member } => squads_multisig_program::ConfigAction::RemoveMember { 
                        old_member: fuzz_accounts.member.get_or_create_account(*old_member, client, 5 * LAMPORTS_PER_SOL).pubkey(),
                    },

                    // ChangeThreshold
                    ConfigAction::ChangeThreshold { new_threshold } => squads_multisig_program::ConfigAction::ChangeThreshold { 
                        new_threshold: *new_threshold,
                    },

                    // SetTimeLock
                    ConfigAction::SetTimeLock { new_time_lock } => squads_multisig_program::ConfigAction::SetTimeLock { 
                        new_time_lock: *new_time_lock,
                    },

                    // AddSpendingLimit
                    ConfigAction::AddSpendingLimit { 
                        create_key, 
                        vault_index, 
                        mint, 
                        amount, 
                        period, 
                        members, 
                        destinations 
                    } => squads_multisig_program::ConfigAction::AddSpendingLimit { 
                        create_key: fuzz_accounts.create_key.get_or_create_account(*create_key, client, 5 * LAMPORTS_PER_SOL).pubkey(),
                        vault_index: *vault_index,
                        mint: fuzz_accounts.mint.get_or_create_account(*mint, client, 6, &Keypair::new().pubkey(), None).unwrap(),
                        amount: *amount,
                        period: squads_multisig_program::Period::from(period.clone()),
                        members: members.iter().map(|member| fuzz_accounts.member.get_or_create_account(*member, client, 5 * LAMPORTS_PER_SOL).pubkey()).collect(),
                        destinations: destinations.iter().map(|dest| fuzz_accounts.destination.get_or_create_account(*dest, client, 5 * LAMPORTS_PER_SOL).pubkey()).collect(),
                    },

                    // RemoveSpendingLimit
                    ConfigAction::RemoveSpendingLimit { spending_limit } => {
                        let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, Keypair::new().pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
                        let spending_limit = fuzz_accounts.spending_limit.get_or_create_account(*spending_limit, &[ 
                                SEED_PREFIX,
                                multisig.pubkey().as_ref(),
                                SEED_VAULT,
                                &Keypair::new().pubkey().as_ref()
                            ], &squads_multisig_program::ID).unwrap();
                        
                        squads_multisig_program::ConfigAction::RemoveSpendingLimit { 
                            spending_limit: spending_limit.pubkey
                        }
                    },

                    // SetRentCollector
                    ConfigAction::SetRentCollector { new_rent_collector } => squads_multisig_program::ConfigAction::SetRentCollector { 
                        new_rent_collector: new_rent_collector.map(|rent_collector| 
                            fuzz_accounts.rent_collector.get_or_create_account(rent_collector, client, 5 * LAMPORTS_PER_SOL).pubkey()
                        ),
                    },
                }
            }).collect();

            let data =
                squads_multisig_program::instruction::ConfigTransactionCreate { 
                    args: squads_multisig_program::ConfigTransactionCreateArgs { 
                        actions, 
                        memo: self.data.memo.clone()
                    }
                };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
        
            let transaction_index = 0u64;
            let rent_payer = fuzz_accounts.rent_payer.get_or_create_account(self.accounts.rent_payer, client, 5 * LAMPORTS_PER_SOL);
            let member = fuzz_accounts.member.get_or_create_account(self.accounts.creator, client, 5 * LAMPORTS_PER_SOL);
            let transaction = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &transaction_index.checked_add(1).unwrap().to_le_bytes()
            ], &squads_multisig_program::ID).unwrap();

            let signers = vec![rent_payer.clone(), member.clone()];
            let acc_meta = squads_multisig_program::accounts::ConfigTransactionCreate {
                multisig: multisig.pubkey,
                transaction: transaction.pubkey,
                creator: member.pubkey(),
                rent_payer: rent_payer.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for VaultTransactionCreate {
        type IxData = squads_multisig_program::instruction::VaultTransactionCreate;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = VaultTransactionCreateSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let signer_kp = fuzz_accounts.member.get_or_create_account(self.accounts.creator, client, 5 * LAMPORTS_PER_SOL);

            let receiver = Keypair::new();
            let recent_blockhash = client.get_last_blockhash();
            let instruction = transfer(&signer_kp.pubkey(), &receiver.pubkey(), 1000);
            let dummy_transfer_message = VersionedMessage::V0(
                Message::try_compile(&signer_kp.pubkey(), &[instruction], &[], recent_blockhash).unwrap(),
            );

            let data = squads_multisig_program::instruction::VaultTransactionCreate { 
                args: squads_multisig_program::VaultTransactionCreateArgs { 
                    vault_index: self.data.vault_index, 
                    ephemeral_signers: self.data.ephemeral_signers, 
                    transaction_message: dummy_transfer_message.serialize(), 
                    memo: self.data.memo.clone() 
                } 
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let signer_kp = fuzz_accounts.member.get_or_create_account(self.accounts.creator, client, 5 * LAMPORTS_PER_SOL);
            let tx_acc = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes()], &squads_multisig_program::ID).unwrap();
            let signers = vec![signer_kp.clone()];

            let acc_meta = squads_multisig_program::accounts::VaultTransactionCreate {
                multisig: multisig.pubkey,
                transaction: tx_acc.pubkey,
                creator: signer_kp.pubkey(),
                rent_payer: signer_kp.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for VaultTransactionExecute {
        type IxData = squads_multisig_program::instruction::VaultTransactionExecute;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = VaultTransactionExecuteSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::VaultTransactionExecute {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                    SEED_PREFIX,
                    multisig.pubkey().as_ref(),
                    SEED_TRANSACTION,
                    &0u64.to_le_bytes(),
                    SEED_PROPOSAL
                ], &squads_multisig_program::ID).unwrap();

            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);
            let tx_acc = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes()], &squads_multisig_program::ID).unwrap();

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::VaultTransactionExecute {
                multisig: multisig.pubkey,
                proposal: proposal.pubkey,
                transaction: tx_acc.pubkey,
                member: member.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for BatchCreate {
        type IxData = squads_multisig_program::instruction::BatchCreate;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = BatchCreateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::BatchCreate { 
                args: squads_multisig_program::BatchCreateArgs { vault_index: self.data.vault_index, memo: self.data.memo.clone() }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let creator = fuzz_accounts.creator.get_or_create_account(self.accounts.creator, client, 5 * LAMPORTS_PER_SOL);

            let batch = fuzz_accounts.batch.get_or_create_account(self.accounts.batch, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes(),], &squads_multisig_program::ID).unwrap();

            let signers = vec![creator.clone()];
            let acc_meta = squads_multisig_program::accounts::BatchCreate {
                multisig: multisig.pubkey,
                batch: batch.pubkey,
                creator: creator.pubkey(),
                rent_payer: creator.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for BatchAddTransaction {
        type IxData = squads_multisig_program::instruction::BatchAddTransaction;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = BatchAddTransactionSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::BatchAddTransaction { 
                args: squads_multisig_program::BatchAddTransactionArgs { ephemeral_signers: self.data.ephemeral_signers, transaction_message: self.data.transaction_message.clone() }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let tx_acc = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                    SEED_PREFIX,
                    multisig.pubkey().as_ref(),
                    SEED_TRANSACTION,
                    &0u64.to_le_bytes(),
                    SEED_PROPOSAL
                ], &squads_multisig_program::ID).unwrap();

            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);

            let batch = fuzz_accounts.batch.get_or_create_account(self.accounts.batch, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes(),], &squads_multisig_program::ID).unwrap();

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::BatchAddTransaction {
                multisig: multisig.pubkey,
                proposal: proposal.pubkey,
                batch: batch.pubkey,
                transaction: tx_acc.pubkey,
                member: member.pubkey(),
                rent_payer: member.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for BatchExecuteTransaction {
        type IxData = squads_multisig_program::instruction::BatchExecuteTransaction;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = BatchExecuteTransactionSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::BatchExecuteTransaction {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let tx_acc = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                    SEED_PREFIX,
                    multisig.pubkey().as_ref(),
                    SEED_TRANSACTION,
                    &0u64.to_le_bytes(),
                    SEED_PROPOSAL
                ], &squads_multisig_program::ID).unwrap();

            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);

            let batch = fuzz_accounts.batch.get_or_create_account(self.accounts.batch, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes(),], &squads_multisig_program::ID).unwrap();

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::BatchExecuteTransaction {
                multisig: multisig.pubkey,
                member: member.pubkey(),
                proposal: proposal.pubkey,
                batch: batch.pubkey,
                transaction: tx_acc.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProposalCreate {
        type IxData = squads_multisig_program::instruction::ProposalCreate;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProposalCreateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::ProposalCreate { 
                args: squads_multisig_program::ProposalCreateArgs { 
                    transaction_index: self.data.transaction_index, 
                    draft: self.data.draft 
                }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                    SEED_PREFIX,
                    multisig.pubkey().as_ref(),
                    SEED_TRANSACTION,
                    &self.data.transaction_index.to_le_bytes(),
                    SEED_PROPOSAL
                ], &squads_multisig_program::ID).unwrap();

            let creator = fuzz_accounts.creator.get_or_create_account(self.accounts.creator, client, 5 * LAMPORTS_PER_SOL);
            
            let signers = vec![creator.clone()];
            let acc_meta = squads_multisig_program::accounts::ProposalCreate {
                multisig: multisig.pubkey,
                proposal: proposal.pubkey,
                creator: creator.pubkey(),
                rent_payer: creator.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProposalActivate {
        type IxData = squads_multisig_program::instruction::ProposalActivate;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProposalActivateSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::ProposalActivate {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::ProposalActivate {
                multisig: multisig.pubkey,
                member: member.pubkey(),
                proposal: proposal.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProposalApprove {
        type IxData = squads_multisig_program::instruction::ProposalApprove;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProposalApproveSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::ProposalApprove { 
                args: squads_multisig_program::ProposalVoteArgs { memo: self.data.memo.clone() }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::ProposalVote {
                multisig: multisig.pubkey,
                member: member.pubkey(),
                proposal: proposal.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProposalReject {
        type IxData = squads_multisig_program::instruction::ProposalReject;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProposalRejectSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::ProposalReject { args: 
                squads_multisig_program::ProposalVoteArgs { memo: self.data.memo.clone() } 
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::ProposalVote {
                multisig: multisig.pubkey,
                member: member.pubkey(),
                proposal: proposal.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ProposalCancel {
        type IxData = squads_multisig_program::instruction::ProposalCancel;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ProposalCancelSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::ProposalCancel { 
                args:  squads_multisig_program::ProposalVoteArgs { memo: self.data.memo.clone() }
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();
            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::ProposalVote {
                multisig: multisig.pubkey,
                member: member.pubkey(),
                proposal: proposal.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SpendingLimitUse {
        type IxData = squads_multisig_program::instruction::SpendingLimitUse;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SpendingLimitUseSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::SpendingLimitUse { 
                args: squads_multisig_program::SpendingLimitUseArgs { 
                    amount: self.data.amount, 
                    decimals: self.data.decimals, 
                    memo: self.data.memo.clone() 
                } 
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let spending_limit = fuzz_accounts.spending_limit.get_or_create_account(self.accounts.spending_limit, &[ 
                    SEED_PREFIX,
                    multisig.pubkey().as_ref(),
                    SEED_VAULT,
                    &Keypair::new().pubkey().as_ref()
                ], &squads_multisig_program::ID).unwrap();

            let vault = fuzz_accounts.vault.get_or_create_account(self.accounts.vault, &[
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_VAULT,
                &0u64.to_le_bytes(),
            ], &squads_multisig_program::ID).unwrap();


            let member = fuzz_accounts.member.get_or_create_account(self.accounts.member, client, 5 * LAMPORTS_PER_SOL);

            let signers = vec![member.clone()];
            let acc_meta = squads_multisig_program::accounts::SpendingLimitUse {
                multisig: multisig.pubkey,
                member: member.pubkey(),
                spending_limit: spending_limit.pubkey,
                vault: vault.pubkey,
                destination: Keypair::new().pubkey(),
                system_program: Some(anchor_lang::system_program::ID),
                mint: None,
                vault_token_account: None,
                destination_token_account: None,
                token_program: None,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ConfigTransactionAccountsClose {
        type IxData = squads_multisig_program::instruction::ConfigTransactionAccountsClose;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ConfigTransactionAccountsCloseSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::ConfigTransactionAccountsClose {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            _client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let tx_acc = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes()], &squads_multisig_program::ID).unwrap();

            let signers = vec![];
            let acc_meta = squads_multisig_program::accounts::ConfigTransactionAccountsClose {
                multisig: multisig.pubkey,
                proposal: proposal.pubkey,
                transaction: tx_acc.pubkey,
                rent_collector: Keypair::new().pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for VaultTransactionAccountsClose {
        type IxData = squads_multisig_program::instruction::VaultTransactionAccountsClose;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = VaultTransactionAccountsCloseSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::VaultTransactionAccountsClose {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            _client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let tx_acc = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes()], &squads_multisig_program::ID).unwrap();

            let signers = vec![];
            let acc_meta = squads_multisig_program::accounts::VaultTransactionAccountsClose {
                multisig: multisig.pubkey,
                proposal: proposal.pubkey,
                transaction: tx_acc.pubkey,
                rent_collector: Keypair::new().pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for VaultBatchTransactionAccountClose {
        type IxData = squads_multisig_program::instruction::VaultBatchTransactionAccountClose;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = VaultBatchTransactionAccountCloseSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::VaultBatchTransactionAccountClose {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            _client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let tx_acc = fuzz_accounts.transaction.get_or_create_account(self.accounts.transaction, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes()], &squads_multisig_program::ID).unwrap();

            let batch = fuzz_accounts.batch.get_or_create_account(self.accounts.batch, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes(),], &squads_multisig_program::ID).unwrap();

            let signers = vec![];
            let acc_meta = squads_multisig_program::accounts::VaultBatchTransactionAccountClose {
                multisig: multisig.pubkey,
                proposal: proposal.pubkey,
                batch: batch.pubkey,
                transaction: tx_acc.pubkey,
                rent_collector: Keypair::new().pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for BatchAccountsClose {
        type IxData = squads_multisig_program::instruction::BatchAccountsClose;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = BatchAccountsCloseSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = squads_multisig_program::instruction::BatchAccountsClose {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            _client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_key = Keypair::new();
            let multisig = fuzz_accounts.multisig.get_or_create_account(self.accounts.multisig, &[SEED_PREFIX, SEED_MULTISIG, create_key.pubkey().as_ref()], &squads_multisig_program::ID).unwrap();

            let proposal = fuzz_accounts.proposal.get_or_create_account(self.accounts.proposal, &[ 
                SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.to_le_bytes(),
                SEED_PROPOSAL
            ], &squads_multisig_program::ID).unwrap();

            let batch = fuzz_accounts.batch.get_or_create_account(self.accounts.batch, &[SEED_PREFIX,
                multisig.pubkey().as_ref(),
                SEED_TRANSACTION,
                &0u64.checked_add(1).unwrap().to_le_bytes(),], &squads_multisig_program::ID).unwrap();

            let signers = vec![];
            let acc_meta = squads_multisig_program::accounts::BatchAccountsClose {
                multisig: multisig.pubkey,
                proposal: proposal.pubkey,
                batch: batch.pubkey,
                rent_collector: Keypair::new().pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        authority: AccountsStorage<Keypair>,
        batch: AccountsStorage<PdaStore>,
        config_authority: AccountsStorage<Keypair>,
        create_key: AccountsStorage<Keypair>,
        creator: AccountsStorage<Keypair>,
        destination: AccountsStorage<Keypair>,
        // destination_token_account: AccountsStorage<TokenStore>,
        initializer: AccountsStorage<Keypair>,
        member: AccountsStorage<Keypair>,
        mint: AccountsStorage<MintStore>,
        multisig: AccountsStorage<PdaStore>,
        program_config: AccountsStorage<PdaStore>,
        proposal: AccountsStorage<PdaStore>,
        rent_collector: AccountsStorage<Keypair>,
        rent_payer: AccountsStorage<Keypair>,
        spending_limit: AccountsStorage<PdaStore>,
        // system_program: AccountsStorage<todo!()>,
        // token_program: AccountsStorage<todo!()>,
        transaction: AccountsStorage<PdaStore>,
        treasury: AccountsStorage<Keypair>,
        vault: AccountsStorage<PdaStore>,
        // vault_token_account: AccountsStorage<TokenStore>,
    }

    impl From<Period> for squads_multisig_program::Period {
        fn from(p: Period) -> Self {
            match p {
                Period::OneTime => squads_multisig_program::Period::OneTime,
                Period::Day => squads_multisig_program::Period::Day,
                Period::Week => squads_multisig_program::Period::Week,
                Period::Month => squads_multisig_program::Period::Month,
            }
        }
    }
}
