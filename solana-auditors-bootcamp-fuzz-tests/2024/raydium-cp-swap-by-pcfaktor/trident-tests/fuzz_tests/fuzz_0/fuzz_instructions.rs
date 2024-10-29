pub mod raydium_cp_swap_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use raydium_cp_swap::states::{AMM_CONFIG_SEED, OBSERVATION_SEED, POOL_VAULT_SEED};
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        CreateAmmConfig(CreateAmmConfig),
        UpdateAmmConfig(UpdateAmmConfig),
        UpdatePoolStatus(UpdatePoolStatus),
        CollectProtocolFee(CollectProtocolFee),
        CollectFundFee(CollectFundFee),
        Initialize(Initialize),
        Deposit(Deposit),
        Withdraw(Withdraw),
        SwapBaseInput(SwapBaseInput),
        SwapBaseOutput(SwapBaseOutput),
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfig {
        pub accounts: CreateAmmConfigAccounts,
        pub data: CreateAmmConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfigAccounts {
        pub owner: AccountId,
        pub amm_config: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreateAmmConfigData {
        pub index: u16,
        pub trade_fee_rate: u64,
        pub protocol_fee_rate: u64,
        pub fund_fee_rate: u64,
        pub create_pool_fee: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfig {
        pub accounts: UpdateAmmConfigAccounts,
        pub data: UpdateAmmConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfigAccounts {
        pub owner: AccountId,
        pub amm_config: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdateAmmConfigData {
        pub param: u8,
        pub value: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatus {
        pub accounts: UpdatePoolStatusAccounts,
        pub data: UpdatePoolStatusData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatusAccounts {
        pub authority: AccountId,
        pub pool_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpdatePoolStatusData {
        pub status: u8,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFee {
        pub accounts: CollectProtocolFeeAccounts,
        pub data: CollectProtocolFeeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFeeAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub amm_config: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub recipient_token_0_account: AccountId,
        pub recipient_token_1_account: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectProtocolFeeData {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFee {
        pub accounts: CollectFundFeeAccounts,
        pub data: CollectFundFeeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFeeAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub amm_config: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub recipient_token_0_account: AccountId,
        pub recipient_token_1_account: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CollectFundFeeData {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub creator: AccountId,
        pub amm_config: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub token_0_mint: AccountId,
        pub token_1_mint: AccountId,
        pub lp_mint: AccountId,
        pub creator_token_0: AccountId,
        pub creator_token_1: AccountId,
        pub creator_lp_token: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub create_pool_fee: AccountId,
        pub observation_state: AccountId,
        pub token_program: AccountId,
        pub token_0_program: AccountId,
        pub token_1_program: AccountId,
        pub associated_token_program: AccountId,
        pub system_program: AccountId,
        pub rent: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        pub init_amount_0: u64,
        pub init_amount_1: u64,
        pub open_time: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Deposit {
        pub accounts: DepositAccounts,
        pub data: DepositData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub owner_lp_token: AccountId,
        pub token_0_account: AccountId,
        pub token_1_account: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub lp_mint: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct DepositData {
        pub lp_token_amount: u64,
        pub maximum_token_0_amount: u64,
        pub maximum_token_1_amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Withdraw {
        pub accounts: WithdrawAccounts,
        pub data: WithdrawData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawAccounts {
        pub owner: AccountId,
        pub authority: AccountId,
        pub pool_state: AccountId,
        pub owner_lp_token: AccountId,
        pub token_0_account: AccountId,
        pub token_1_account: AccountId,
        pub token_0_vault: AccountId,
        pub token_1_vault: AccountId,
        pub token_program: AccountId,
        pub token_program_2022: AccountId,
        pub vault_0_mint: AccountId,
        pub vault_1_mint: AccountId,
        pub lp_mint: AccountId,
        pub memo_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawData {
        pub lp_token_amount: u64,
        pub minimum_token_0_amount: u64,
        pub minimum_token_1_amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInput {
        pub accounts: SwapBaseInputAccounts,
        pub data: SwapBaseInputData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInputAccounts {
        pub payer: AccountId,
        pub authority: AccountId,
        pub amm_config: AccountId,
        pub pool_state: AccountId,
        pub input_token_account: AccountId,
        pub output_token_account: AccountId,
        pub input_vault: AccountId,
        pub output_vault: AccountId,
        pub input_token_program: AccountId,
        pub output_token_program: AccountId,
        pub input_token_mint: AccountId,
        pub output_token_mint: AccountId,
        pub observation_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseInputData {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutput {
        pub accounts: SwapBaseOutputAccounts,
        pub data: SwapBaseOutputData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutputAccounts {
        pub payer: AccountId,
        pub authority: AccountId,
        pub amm_config: AccountId,
        pub pool_state: AccountId,
        pub input_token_account: AccountId,
        pub output_token_account: AccountId,
        pub input_vault: AccountId,
        pub output_vault: AccountId,
        pub input_token_program: AccountId,
        pub output_token_program: AccountId,
        pub input_token_mint: AccountId,
        pub output_token_mint: AccountId,
        pub observation_state: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapBaseOutputData {
        pub max_amount_in: u64,
        pub amount_out: u64,
    }
    impl<'info> IxOps<'info> for CreateAmmConfig {
        type IxData = raydium_cp_swap::instruction::CreateAmmConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreateAmmConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CreateAmmConfig {
                index: self.data.index,
                trade_fee_rate: self.data.trade_fee_rate,
                protocol_fee_rate: self.data.protocol_fee_rate,
                fund_fee_rate: self.data.fund_fee_rate,
                create_pool_fee: self.data.create_pool_fee,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.owner.get_or_create_account(
                self.accounts.owner,
                client,
                LAMPORTS_PER_SOL,
            );

            let amm_config = fuzz_accounts
                .amm_config
                .get_or_create_account(
                    self.accounts.amm_config,
                    &[AMM_CONFIG_SEED.as_bytes(), &self.data.index.to_be_bytes()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::CreateAmmConfig {
                owner: signer.pubkey(),
                amm_config: amm_config.pubkey,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UpdateAmmConfig {
        type IxData = raydium_cp_swap::instruction::UpdateAmmConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateAmmConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::UpdateAmmConfig {
                param: self.data.param,
                value: self.data.value,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.owner.get_or_create_account(
                self.accounts.owner,
                client,
                LAMPORTS_PER_SOL,
            );

            let amm_config = fuzz_accounts
                .amm_config
                .get_or_create_account(
                    self.accounts.amm_config,
                    &[AMM_CONFIG_SEED.as_bytes(), &0u16.to_be_bytes()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::UpdateAmmConfig {
                owner: signer.pubkey(),
                amm_config: amm_config.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UpdatePoolStatus {
        type IxData = raydium_cp_swap::instruction::UpdatePoolStatus;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdatePoolStatusSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::UpdatePoolStatus {
                status: self.data.status,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.owner.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::UpdatePoolStatus {
                authority: signer.pubkey(),
                pool_state: pool_state.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CollectProtocolFee {
        type IxData = raydium_cp_swap::instruction::CollectProtocolFee;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CollectProtocolFeeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CollectProtocolFee {
                amount_0_requested: self.data.amount_0_requested,
                amount_1_requested: self.data.amount_1_requested,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.owner.get_or_create_account(
                self.accounts.owner,
                client,
                LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let amm_config = fuzz_accounts
                .amm_config
                .get_or_create_account(
                    self.accounts.amm_config,
                    &[AMM_CONFIG_SEED.as_bytes(), &0u16.to_be_bytes()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let vault_0_mint = fuzz_accounts
                .vault_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_mint = fuzz_accounts
                .token_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_vault = fuzz_accounts
                .token_0_vault
                .get_or_create_account(
                    self.accounts.token_0_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        &token_0_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let recipient_token_0_account = fuzz_accounts
                .recipient_token_0_account
                .get_or_create_account(
                    self.accounts.recipient_token_0_account,
                    client,
                    LAMPORTS_PER_SOL,
                );

            let vault_1_mint = fuzz_accounts
                .vault_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_1_mint = fuzz_accounts
                .token_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_1_vault = fuzz_accounts
                .token_1_vault
                .get_or_create_account(
                    self.accounts.token_1_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_1_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let recipient_token_1_account = fuzz_accounts
                .recipient_token_1_account
                .get_or_create_account(
                    self.accounts.recipient_token_1_account,
                    client,
                    LAMPORTS_PER_SOL,
                );

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::CollectProtocolFee {
                owner: signer.pubkey(),
                authority: authority.pubkey(),
                pool_state: pool_state.pubkey(),
                amm_config: amm_config.pubkey,
                token_0_vault: token_0_vault.pubkey,
                token_1_vault: token_1_vault.pubkey,
                vault_0_mint,
                vault_1_mint,
                recipient_token_0_account: recipient_token_0_account.pubkey(),
                recipient_token_1_account: recipient_token_1_account.pubkey(),
                token_program: anchor_spl::token::ID,
                token_program_2022: anchor_spl::token_2022::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CollectFundFee {
        type IxData = raydium_cp_swap::instruction::CollectFundFee;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CollectFundFeeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::CollectFundFee {
                amount_0_requested: self.data.amount_0_requested,
                amount_1_requested: self.data.amount_1_requested,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.owner.get_or_create_account(
                self.accounts.owner,
                client,
                LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let amm_config = fuzz_accounts
                .amm_config
                .get_or_create_account(
                    self.accounts.amm_config,
                    &[AMM_CONFIG_SEED.as_bytes(), &0u16.to_be_bytes()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let vault_0_mint = fuzz_accounts
                .vault_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_mint = fuzz_accounts
                .token_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_vault = fuzz_accounts
                .token_0_vault
                .get_or_create_account(
                    self.accounts.token_0_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_0_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let recipient_token_0_account = fuzz_accounts
                .recipient_token_0_account
                .get_or_create_account(
                    self.accounts.recipient_token_0_account,
                    client,
                    LAMPORTS_PER_SOL,
                );

            let vault_1_mint = fuzz_accounts
                .vault_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_1_mint = fuzz_accounts
                .token_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_1_vault = fuzz_accounts
                .token_1_vault
                .get_or_create_account(
                    self.accounts.token_1_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_1_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let recipient_token_1_account = fuzz_accounts
                .recipient_token_1_account
                .get_or_create_account(
                    self.accounts.recipient_token_1_account,
                    client,
                    LAMPORTS_PER_SOL,
                );

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::CollectFundFee {
                owner: signer.pubkey(),
                authority: authority.pubkey(),
                pool_state: pool_state.pubkey(),
                amm_config: amm_config.pubkey,
                token_0_vault: token_0_vault.pubkey,
                token_1_vault: token_1_vault.pubkey,
                vault_0_mint,
                vault_1_mint,
                recipient_token_0_account: recipient_token_0_account.pubkey(),
                recipient_token_1_account: recipient_token_1_account.pubkey(),
                token_program: anchor_spl::token::ID,
                token_program_2022: anchor_spl::token_2022::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = raydium_cp_swap::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Initialize {
                init_amount_0: self.data.init_amount_0,
                init_amount_1: self.data.init_amount_1,
                open_time: self.data.open_time,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let create_pool_fee = raydium_cp_swap::create_pool_fee_reveiver::ID;

            let signer = fuzz_accounts.creator.get_or_create_account(
                self.accounts.creator,
                client,
                LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let amm_config = fuzz_accounts
                .amm_config
                .get_or_create_account(
                    self.accounts.amm_config,
                    &[AMM_CONFIG_SEED.as_bytes(), &0u16.to_be_bytes()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let token_0_mint = fuzz_accounts
                .token_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_vault = fuzz_accounts
                .token_0_vault
                .get_or_create_account(
                    self.accounts.token_0_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_0_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let creator_token_0 = fuzz_accounts
                .creator_token_0
                .get_or_create_account(
                    self.accounts.creator_token_0,
                    client,
                    token_0_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let token_1_mint = fuzz_accounts
                .token_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_1_vault = fuzz_accounts
                .token_1_vault
                .get_or_create_account(
                    self.accounts.token_1_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_1_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let creator_token_1 = fuzz_accounts
                .creator_token_1
                .get_or_create_account(
                    self.accounts.creator_token_1,
                    client,
                    token_1_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let lp_mint = fuzz_accounts
                .lp_mint
                .get_or_create_account(2, client, 9, &signer.pubkey(), None)
                .unwrap();

            let creator_lp_token = fuzz_accounts
                .creator_lp_token
                .get_or_create_account(
                    self.accounts.creator_lp_token,
                    client,
                    lp_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let observation_state = fuzz_accounts
                .observation_state
                .get_or_create_account(
                    self.accounts.observation_state,
                    &[OBSERVATION_SEED.as_bytes(), pool_state.pubkey().as_ref()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::Initialize {
                creator: signer.pubkey(),
                amm_config: amm_config.pubkey,
                authority: authority.pubkey(),
                pool_state: pool_state.pubkey(),
                token_0_mint,
                token_1_mint,
                lp_mint,
                creator_token_0,
                creator_token_1,
                creator_lp_token,
                token_0_vault: token_0_vault.pubkey,
                token_1_vault: token_1_vault.pubkey,
                create_pool_fee,
                observation_state: observation_state.pubkey,
                token_program: anchor_spl::token::ID,
                token_0_program: anchor_spl::token_2022::ID,
                token_1_program: anchor_spl::token_2022::ID,
                associated_token_program: anchor_spl::associated_token::ID,
                system_program: solana_sdk::system_program::ID,
                rent: solana_sdk::sysvar::rent::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Deposit {
        type IxData = raydium_cp_swap::instruction::Deposit;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = DepositSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Deposit {
                lp_token_amount: self.data.lp_token_amount,
                maximum_token_0_amount: self.data.maximum_token_0_amount,
                maximum_token_1_amount: self.data.maximum_token_1_amount,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.owner.get_or_create_account(
                self.accounts.owner,
                client,
                LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let vault_0_mint = fuzz_accounts
                .vault_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_mint = fuzz_accounts
                .token_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_vault = fuzz_accounts
                .token_0_vault
                .get_or_create_account(
                    self.accounts.token_0_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_0_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let vault_1_mint = fuzz_accounts
                .vault_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let lp_mint = fuzz_accounts
                .lp_mint
                .get_or_create_account(2, client, 9, &signer.pubkey(), None)
                .unwrap();

            let owner_lp_token = fuzz_accounts
                .owner_lp_token
                .get_or_create_account(
                    self.accounts.owner_lp_token,
                    client,
                    lp_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let token_0_account = fuzz_accounts
                .token_0_account
                .get_or_create_account(
                    self.accounts.token_0_account,
                    client,
                    vault_0_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let token_1_account = fuzz_accounts
                .token_1_account
                .get_or_create_account(
                    self.accounts.token_1_account,
                    client,
                    vault_1_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::Deposit {
                owner: signer.pubkey(),
                authority: authority.pubkey(),
                pool_state: pool_state.pubkey(),
                owner_lp_token,
                token_0_account,
                token_1_account,
                token_0_vault: token_0_vault.pubkey,
                token_1_vault: token_0_vault.pubkey,
                token_program: anchor_spl::token::ID,
                token_program_2022: anchor_spl::token_2022::ID,
                vault_0_mint,
                vault_1_mint,
                lp_mint,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Withdraw {
        type IxData = raydium_cp_swap::instruction::Withdraw;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::Withdraw {
                lp_token_amount: self.data.lp_token_amount,
                minimum_token_0_amount: self.data.minimum_token_0_amount,
                minimum_token_1_amount: self.data.minimum_token_1_amount,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.owner.get_or_create_account(
                self.accounts.owner,
                client,
                LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let vault_0_mint = fuzz_accounts
                .vault_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_mint = fuzz_accounts
                .token_0_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_0_vault = fuzz_accounts
                .token_0_vault
                .get_or_create_account(
                    self.accounts.token_0_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_0_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let vault_1_mint = fuzz_accounts
                .vault_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_1_mint = fuzz_accounts
                .token_1_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let token_1_vault = fuzz_accounts
                .token_1_vault
                .get_or_create_account(
                    self.accounts.token_1_vault,
                    &[
                        POOL_VAULT_SEED.as_bytes(),
                        pool_state.pubkey().as_ref(),
                        token_1_mint.as_ref(),
                    ],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let lp_mint = fuzz_accounts
                .lp_mint
                .get_or_create_account(2, client, 9, &signer.pubkey(), None)
                .unwrap();

            let owner_lp_token = fuzz_accounts
                .owner_lp_token
                .get_or_create_account(
                    self.accounts.owner_lp_token,
                    client,
                    lp_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let token_0_account = fuzz_accounts
                .token_0_account
                .get_or_create_account(
                    self.accounts.token_0_account,
                    client,
                    vault_0_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let token_1_account = fuzz_accounts
                .token_1_account
                .get_or_create_account(
                    self.accounts.token_1_account,
                    client,
                    vault_1_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::Withdraw {
                owner: signer.pubkey(),
                authority: authority.pubkey(),
                pool_state: pool_state.pubkey(),
                owner_lp_token,
                token_0_account,
                token_1_account,
                token_0_vault: token_0_vault.pubkey,
                token_1_vault: token_1_vault.pubkey,
                token_program: anchor_spl::token::ID,
                token_program_2022: anchor_spl::token_2022::ID,
                vault_0_mint,
                vault_1_mint,
                lp_mint,
                memo_program: spl_memo::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SwapBaseInput {
        type IxData = raydium_cp_swap::instruction::SwapBaseInput;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SwapBaseInputSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::SwapBaseInput {
                amount_in: self.data.amount_in,
                minimum_amount_out: self.data.minimum_amount_out,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let amm_config = fuzz_accounts
                .amm_config
                .get_or_create_account(
                    self.accounts.amm_config,
                    &[AMM_CONFIG_SEED.as_bytes(), &0u16.to_be_bytes()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let input_token_mint = fuzz_accounts
                .input_token_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let output_token_mint = fuzz_accounts
                .output_token_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let input_token_account = fuzz_accounts
                .input_token_account
                .get_or_create_account(
                    self.accounts.input_token_account,
                    client,
                    input_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let output_token_account = fuzz_accounts
                .output_token_account
                .get_or_create_account(
                    self.accounts.output_token_account,
                    client,
                    output_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let input_vault = fuzz_accounts
                .input_vault
                .get_or_create_account(
                    self.accounts.input_vault,
                    client,
                    input_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let output_vault = fuzz_accounts
                .output_vault
                .get_or_create_account(
                    self.accounts.output_vault,
                    client,
                    output_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let observation_state = fuzz_accounts
                .observation_state
                .get_or_create_account(
                    self.accounts.observation_state,
                    &[OBSERVATION_SEED.as_bytes(), pool_state.pubkey().as_ref()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: signer.pubkey(),
                authority: authority.pubkey(),
                amm_config: amm_config.pubkey,
                pool_state: pool_state.pubkey(),
                input_token_account,
                output_token_account,
                input_vault,
                output_vault,
                input_token_program: anchor_spl::token::ID,
                output_token_program: anchor_spl::token::ID,
                input_token_mint,
                output_token_mint,
                observation_state: observation_state.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SwapBaseOutput {
        type IxData = raydium_cp_swap::instruction::SwapBaseOutput;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SwapBaseOutputSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = raydium_cp_swap::instruction::SwapBaseOutput {
                max_amount_in: self.data.max_amount_in,
                amount_out: self.data.amount_out,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                LAMPORTS_PER_SOL,
            );

            let amm_config = fuzz_accounts
                .amm_config
                .get_or_create_account(
                    self.accounts.amm_config,
                    &[AMM_CONFIG_SEED.as_bytes(), &0u16.to_be_bytes()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state,
                client,
                LAMPORTS_PER_SOL,
            );

            let input_token_mint = fuzz_accounts
                .input_token_mint
                .get_or_create_account(0, client, 9, &signer.pubkey(), None)
                .unwrap();

            let output_token_mint = fuzz_accounts
                .output_token_mint
                .get_or_create_account(1, client, 9, &signer.pubkey(), None)
                .unwrap();

            let input_token_account = fuzz_accounts
                .input_token_account
                .get_or_create_account(
                    self.accounts.input_token_account,
                    client,
                    input_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let output_token_account = fuzz_accounts
                .output_token_account
                .get_or_create_account(
                    self.accounts.output_token_account,
                    client,
                    output_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let input_vault = fuzz_accounts
                .output_token_account
                .get_or_create_account(
                    self.accounts.input_vault,
                    client,
                    input_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let output_vault = fuzz_accounts
                .output_token_account
                .get_or_create_account(
                    self.accounts.output_vault,
                    client,
                    output_token_mint,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let observation_state = fuzz_accounts
                .observation_state
                .get_or_create_account(
                    self.accounts.observation_state,
                    &[OBSERVATION_SEED.as_bytes(), pool_state.pubkey().as_ref()],
                    &raydium_cp_swap::ID,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: signer.pubkey(),
                authority: authority.pubkey(),
                amm_config: amm_config.pubkey,
                pool_state: pool_state.pubkey(),
                input_token_account,
                output_token_account,
                input_vault,
                output_vault,
                input_token_program: anchor_spl::token::ID,
                output_token_program: anchor_spl::token::ID,
                input_token_mint,
                output_token_mint,
                observation_state: observation_state.pubkey,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        amm_config: AccountsStorage<PdaStore>,
        // associated_token_program: AccountsStorage<todo!()>,
        authority: AccountsStorage<Keypair>,
        // create_pool_fee: AccountsStorage<Keypair>,
        creator: AccountsStorage<Keypair>,
        creator_lp_token: AccountsStorage<TokenStore>,
        creator_token_0: AccountsStorage<TokenStore>,
        creator_token_1: AccountsStorage<TokenStore>,
        input_token_account: AccountsStorage<TokenStore>,
        input_token_mint: AccountsStorage<MintStore>,
        // input_token_program: AccountsStorage<todo!()>,
        input_vault: AccountsStorage<TokenStore>,
        lp_mint: AccountsStorage<MintStore>,
        // memo_program: AccountsStorage<todo!()>,
        observation_state: AccountsStorage<PdaStore>,
        output_token_account: AccountsStorage<TokenStore>,
        output_token_mint: AccountsStorage<MintStore>,
        // output_token_program: AccountsStorage<todo!()>,
        output_vault: AccountsStorage<TokenStore>,
        owner: AccountsStorage<Keypair>,
        owner_lp_token: AccountsStorage<TokenStore>,
        payer: AccountsStorage<Keypair>,
        pool_state: AccountsStorage<Keypair>,
        recipient_token_0_account: AccountsStorage<Keypair>,
        recipient_token_1_account: AccountsStorage<Keypair>,
        // rent: AccountsStorage<todo!()>,
        // system_program: AccountsStorage<todo!()>,
        token_0_account: AccountsStorage<TokenStore>,
        token_0_mint: AccountsStorage<MintStore>,
        // token_0_program: AccountsStorage<todo!()>,
        token_0_vault: AccountsStorage<PdaStore>,
        token_1_account: AccountsStorage<TokenStore>,
        token_1_mint: AccountsStorage<MintStore>,
        // token_1_program: AccountsStorage<todo!()>,
        token_1_vault: AccountsStorage<PdaStore>,
        // token_program: AccountsStorage<todo!()>,
        // token_program_2022: AccountsStorage<todo!()>,
        vault_0_mint: AccountsStorage<MintStore>,
        vault_1_mint: AccountsStorage<MintStore>,
    }
}
