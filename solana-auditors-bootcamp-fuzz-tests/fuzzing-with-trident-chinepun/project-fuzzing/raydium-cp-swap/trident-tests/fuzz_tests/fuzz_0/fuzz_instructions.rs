pub mod raydium_cp_swap_fuzz_instructions {

    use std::borrow::Borrow;
    use std::str::FromStr;

    use crate::accounts_snapshots::*;
    use crate::utils::{get_admin_path, get_create_pool_receiver_path, return_index};

    use anchor_lang::{AccountDeserialize, ToAccountInfos};
    use anchor_spl::token::{spl_token, TokenAccount};

    use anchor_spl::token::spl_token::state::{AccountState, GenericTokenAccount};
    use anchor_spl::token_interface;
    use raydium_cp_swap::states::{OBSERVATION_SEED, POOL_LP_MINT_SEED, POOL_VAULT_SEED};
    use raydium_cp_swap::{create_pool_fee_reveiver, AUTH_SEED};
    use solana_sdk::account::Account;
    use solana_sdk::msg;
    use solana_sdk::rent::Rent;
    use solana_sdk::sysvar::{Sysvar, SysvarId};
    use solana_sdk::{account::AccountSharedData, native_token::LAMPORTS_PER_SOL, signer::EncodableKey};
    use raydium_cp_swap::{curve::FEE_RATE_DENOMINATOR_VALUE, states::AMM_CONFIG_SEED};
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        CreateAmmConfig(CreateAmmConfig),
        UpdateAmmConfig(UpdateAmmConfig),
        // UpdatePoolStatus(UpdatePoolStatus),
        // CollectProtocolFee(CollectProtocolFee),
        // CollectFundFee(CollectFundFee),
        Initialize(Initialize),
        // Deposit(Deposit),
        // Withdraw(Withdraw),
        // SwapBaseInput(SwapBaseInput),
        // SwapBaseOutput(SwapBaseOutput),
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct UpdateAmmConfigData {
        pub param: u8,
        pub index: u16,
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
    #[derive(Debug)]
    pub struct InitializeData {
        pub index: u16,
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
            let path = get_admin_path();
            let admin_keypair = Keypair::read_from_file(path.clone())
            .map_err(|_| trident_client::test::anyhow::anyhow!("failed to read keypair from {}", path)).unwrap();

            client.set_account_custom(
                &admin_keypair.pubkey(), 
                &AccountSharedData::new(
                    50000 * LAMPORTS_PER_SOL, 
                    0, 
                    &solana_sdk::system_program::ID
                )
            );

            let amm_config = fuzz_accounts.amm_config.get_or_create_account(
                self.accounts.amm_config, 
                &[
                AMM_CONFIG_SEED.as_bytes(),
                &self.data.index.to_be_bytes(),
            ],
             &raydium_cp_swap::ID
            ).unwrap();
            let signers = vec![admin_keypair.clone()];
            let acc_meta = raydium_cp_swap::accounts::CreateAmmConfig {
                owner: admin_keypair.pubkey(),
                amm_config: amm_config.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);


            Ok((signers, acc_meta))
        }
        fn check(
                &self,
                pre_ix: Self::IxSnapshot,
                post_ix: Self::IxSnapshot,
                ix_data: Self::IxData,
            ) -> Result<(), FuzzingError> {
            let path = get_admin_path();
            let admin_keypair = Keypair::read_from_file(path.clone())
            .map_err(|_| trident_client::test::anyhow::anyhow!("failed to read keypair from {}", path)).unwrap();

            if &admin_keypair.pubkey() != pre_ix.owner.key
            {
                return Err(FuzzingError::DataMismatch);
            }

            Ok(())
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
            _client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let path = get_admin_path();
            let admin_keypair = Keypair::read_from_file(path.clone())
            .map_err(|_| trident_client::test::anyhow::anyhow!("failed to read keypair from {}", path)).unwrap();

            // let val = return_index();
            let amm_config = fuzz_accounts.amm_config.get_or_create_account(
                self.accounts.amm_config, 
                &[
                AMM_CONFIG_SEED.as_bytes(),
                &self.data.index.to_be_bytes()//Not our expected PDA
            ],
             &raydium_cp_swap::ID
            ).unwrap();
            let signers = vec![admin_keypair.clone()];

            let owner_acc_meta = AccountMeta{ pubkey: admin_keypair.pubkey(), is_signer: true, is_writable: true };
            let amm_config_acc_meta = AccountMeta{ pubkey: amm_config.pubkey(), is_signer: false, is_writable: true };
            let protocol_owner_acc_meta = AccountMeta{ pubkey: Pubkey::new_unique(), is_signer: false, is_writable: false };

            Ok((signers, vec![owner_acc_meta, amm_config_acc_meta, protocol_owner_acc_meta]))
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
            let data = raydium_cp_swap::instruction::UpdatePoolStatus { status: todo!() };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::UpdatePoolStatus {
                authority: todo!(),
                pool_state: todo!(),
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
                amount_0_requested: todo!(),
                amount_1_requested: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::CollectProtocolFee {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                amm_config: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                recipient_token_0_account: todo!(),
                recipient_token_1_account: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
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
                amount_0_requested: todo!(),
                amount_1_requested: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::CollectFundFee {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                amm_config: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                recipient_token_0_account: todo!(),
                recipient_token_1_account: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
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
            let creator = fuzz_accounts.creator.get_or_create_account(
                self.accounts.creator, 
                client, 
                10 * LAMPORTS_PER_SOL
            );

            let amm_config = fuzz_accounts.amm_config.get_or_create_account(
                self.accounts.amm_config, 
                &[
                    AMM_CONFIG_SEED.as_bytes(),
                    &self.data.index.to_be_bytes()//Not our expected PDA
                ],
                &raydium_cp_swap::ID
            ).unwrap();

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority, 
                &[
                    AUTH_SEED.as_bytes(),
                ], 
                &raydium_cp_swap::ID
            ).unwrap();

            let pool_state = fuzz_accounts.pool_state.get_or_create_account(
                self.accounts.pool_state, 
                client, 
                0 * LAMPORTS_PER_SOL
            );

            let mut token_0_mint = fuzz_accounts.token_0_mint.get_or_create_account(
                self.accounts.token_0_mint, 
                client, 
                9, 
                &creator.pubkey(), 
                None
            ).unwrap();

            let mut token_1_mint = fuzz_accounts.token_1_mint.get_or_create_account(
                self.accounts.token_1_mint, 
                client, 
                9, 
                &creator.pubkey(), 
                None
            ).unwrap();

            if token_0_mint.key() > token_1_mint.key()
            {
                let temp = token_0_mint;
                token_0_mint = token_1_mint;
                token_1_mint = temp;
            }

            let (lp_mint_address, _) = Pubkey::try_find_program_address(
                &[
                    POOL_LP_MINT_SEED.as_bytes(),
                    pool_state.pubkey().as_ref(),
                ], 
                &raydium_cp_swap::ID
            ).unwrap();

            let creator_token_0 = fuzz_accounts.creator_token_0.get_or_create_account(
                self.accounts.creator_token_0, 
                client, 
                token_0_mint, 
                creator.pubkey(), 
                self.data.init_amount_0,
                None,
                None,
                0,
                None
            ).unwrap();

            let creator_token_1 = fuzz_accounts.creator_token_1.get_or_create_account(
                self.accounts.creator_token_1, 
                client, 
                token_1_mint,
                creator.pubkey(), 
                self.data.init_amount_1, 
                None, 
                None, 
                0, 
                None
            ).unwrap();

            let creator_lp_token = spl_associated_token_account::get_associated_token_address(
                &creator.pubkey(), 
                &lp_mint_address
            );

            

            let (token_0_vault, _) = Pubkey::find_program_address(
                &[
                    POOL_VAULT_SEED.as_bytes(),
                    pool_state.pubkey().as_ref(),
                    token_0_mint.key().as_ref()
                ], 
                &raydium_cp_swap::ID
            );

            let (token_1_vault, _) = Pubkey::find_program_address(
                &[
                    POOL_VAULT_SEED.as_bytes(),
                    pool_state.pubkey().as_ref(),
                    token_1_mint.key().as_ref()
                ], 
                &raydium_cp_swap::ID
            );

            let cpf_path = get_create_pool_receiver_path();
            let create_pool_fee_reveiver_keypair = Keypair::read_from_file(cpf_path.clone())
            .map_err(|_| trident_client::test::anyhow::anyhow!("failed to read keypair from {}", cpf_path)).unwrap();

            let admin_path = get_admin_path();
            let admin_keypair = Keypair::read_from_file(admin_path.clone())
            .map_err(|_| trident_client::test::anyhow::anyhow!("failed to read keypair from {}", admin_path)).unwrap();
            let create_pool_fee_reveiver = create_pool_fee_reveiver_keypair.pubkey();

            let rent = Rent::default();//::minimum_balance(&self, TokenAccount::LEN);
            let wsol_min_balance  = rent.minimum_balance(TokenAccount::LEN);

            client.set_token_account_custom(
            create_pool_fee_reveiver,
            Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
            admin_keypair.pubkey(),
            500 * LAMPORTS_PER_SOL,
            None,
            Some(wsol_min_balance),
            0,
            None
            );  

            let final_account:solana_sdk::account::Account;

            match  client.get_account(&create_pool_fee_reveiver) {
                Ok(some_account) => 
                { final_account = some_account.unwrap(); },
                
                Err(_) => return Err(FuzzingError::Custom(0)),
            };
            let mut data:&[u8] = final_account.data.borrow();
            let create_pool_fee_reveiver_state = TokenAccount::try_deserialize(&mut data);
            
            match create_pool_fee_reveiver_state {
                Ok(state) => {
                    if state.state != AccountState::Initialized{
                        return Err(FuzzingError::Custom(1));
                    }
                },
                Err(_e) => {return Err(FuzzingError::Custom(2))}
            };

            let observation_state = fuzz_accounts.observation_state.get_or_create_account(
                self.accounts.observation_state, 
                &[
                    OBSERVATION_SEED.as_bytes(),
                    pool_state.pubkey().as_ref(),
                ], 
                &raydium_cp_swap::ID
            ).unwrap();

            let token_0_opt_account: Option<Account>;
            let token_1_opt_account: Option<Account>;
            match client.get_account(&token_0_mint) {
                Ok(val) => { token_0_opt_account = val; },
                Err(_) => {return Err(FuzzingError::Custom(1));},
            }
            let token_0_program = token_0_opt_account.unwrap();

            match client.get_account(&token_1_mint) {
                Ok(val) => { token_1_opt_account = val; },
                Err(_) => {return Err(FuzzingError::Custom(1));},
            }
            let token_1_program = token_1_opt_account.unwrap();

            let signers = vec![creator.clone(), pool_state.clone()];

            let acc_meta = vec![
                AccountMeta{ pubkey: creator.pubkey(), is_signer: true, is_writable: true }, // creator
                AccountMeta{ pubkey: amm_config.pubkey(), is_signer: false, is_writable: false }, // ammConfig
                AccountMeta{ pubkey: authority.pubkey(), is_signer: false, is_writable: false }, // authority
                AccountMeta{ pubkey: pool_state.pubkey(), is_signer: true, is_writable: true }, // poolState
                AccountMeta{ pubkey: token_0_mint, is_signer: false, is_writable: false }, // token0Mint
                AccountMeta{ pubkey: token_1_mint, is_signer: false, is_writable: false }, // token1Mint
                AccountMeta{ pubkey: lp_mint_address, is_signer: false, is_writable: true }, // lpMint
                AccountMeta{ pubkey: creator_token_0, is_signer: false, is_writable: true }, // creatorToken0
                AccountMeta{ pubkey: creator_token_1, is_signer: false, is_writable: true }, // creatorToken1
                AccountMeta{ pubkey: creator_lp_token, is_signer: false, is_writable: true }, // creatorLpToken
                AccountMeta{ pubkey: token_0_vault, is_signer: false, is_writable: true }, // token0Vault
                AccountMeta{ pubkey: token_1_vault, is_signer: false, is_writable: true }, // token1Vault
                AccountMeta{ pubkey: create_pool_fee_reveiver, is_signer: false, is_writable: true }, // createPoolFee
                AccountMeta{ pubkey: observation_state.pubkey(), is_signer: false, is_writable: true }, // observationState
                AccountMeta{ pubkey: spl_token::id(), is_signer: false, is_writable: false }, // tokenProgram
                AccountMeta{ pubkey: token_0_program.owner, is_signer: false, is_writable: false }, // token0Program
                AccountMeta{ pubkey: token_1_program.owner, is_signer: false, is_writable: false }, // token1Program
                AccountMeta{ pubkey: spl_associated_token_account::ID, is_signer: false, is_writable: false }, // associatedTokenProgram
                AccountMeta{ pubkey: solana_sdk::system_program::ID, is_signer: false, is_writable: false }, // systemProgram
                AccountMeta{ pubkey: solana_sdk::rent::Rent::id(), is_signer: false, is_writable: false }, // rent
            ];


            Ok((signers, acc_meta))
        }
    
        fn check(
                &self,
                pre_ix: Self::IxSnapshot,
                post_ix: Self::IxSnapshot,
                ix_data: Self::IxData,
            ) -> Result<(), FuzzingError> {
            
            if pre_ix.creator_token_0.amount - ix_data.init_amount_0 != post_ix.creator_token_0.amount ||
               pre_ix.creator_token_1.amount - ix_data.init_amount_1 != post_ix.creator_token_1.amount
            {
                return Err(FuzzingError::BalanceMismatch);
            }


            if let Some(lp_before) = pre_ix.creator_lp_token {
                if let Some(lp_after) = post_ix.creator_lp_token {
                    if lp_after.amount < lp_before.amount {
                        return Err(FuzzingError::BalanceMismatch);
                    }
                }                
            }

            Ok(())
        }

        fn tx_error_handler(
                &self,
                e: FuzzClientErrorWithOrigin,
                ix_data: Self::IxData,
                pre_ix_acc_infos: &'info mut [Option<AccountInfo<'info>>],
            ) -> Result<(), FuzzClientErrorWithOrigin> {
            Ok(())
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
                lp_token_amount: todo!(),
                maximum_token_0_amount: todo!(),
                maximum_token_1_amount: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Deposit {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                owner_lp_token: todo!(),
                token_0_account: todo!(),
                token_1_account: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                lp_mint: todo!(),
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
                lp_token_amount: todo!(),
                minimum_token_0_amount: todo!(),
                minimum_token_1_amount: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Withdraw {
                owner: todo!(),
                authority: todo!(),
                pool_state: todo!(),
                owner_lp_token: todo!(),
                token_0_account: todo!(),
                token_1_account: todo!(),
                token_0_vault: todo!(),
                token_1_vault: todo!(),
                token_program: todo!(),
                token_program_2022: todo!(),
                vault_0_mint: todo!(),
                vault_1_mint: todo!(),
                lp_mint: todo!(),
                memo_program: todo!(),
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
                amount_in: todo!(),
                minimum_amount_out: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: todo!(),
                authority: todo!(),
                amm_config: todo!(),
                pool_state: todo!(),
                input_token_account: todo!(),
                output_token_account: todo!(),
                input_vault: todo!(),
                output_vault: todo!(),
                input_token_program: todo!(),
                output_token_program: todo!(),
                input_token_mint: todo!(),
                output_token_mint: todo!(),
                observation_state: todo!(),
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
                max_amount_in: todo!(),
                amount_out: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = raydium_cp_swap::accounts::Swap {
                payer: todo!(),
                authority: todo!(),
                amm_config: todo!(),
                pool_state: todo!(),
                input_token_account: todo!(),
                output_token_account: todo!(),
                input_vault: todo!(),
                output_vault: todo!(),
                input_token_program: todo!(),
                output_token_program: todo!(),
                input_token_mint: todo!(),
                output_token_mint: todo!(),
                observation_state: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }

    impl<'a> Arbitrary<'a> for CreateAmmConfigData {
        fn arbitrary(
            u: &mut arbitrary::Unstructured<'a>
        ) -> arbitrary::Result<Self> {

            let index: u16 = return_index();//u.int_in_range(0..=MAX_INDEX_VALUE_FOR_AMM_CONFIG)?;//reduce the range so we can pass more instructions
            let trade_fee_rate: u64 = u.int_in_range(0..=FEE_RATE_DENOMINATOR_VALUE / 2)?;
            let protocol_fee_rate: u64 = u.int_in_range(0..=FEE_RATE_DENOMINATOR_VALUE / 2)?;
            let fund_fee_rate: u64 = u.int_in_range(0..=FEE_RATE_DENOMINATOR_VALUE / 2)?;
            let create_pool_fee: u64 = u.int_in_range(0..=FEE_RATE_DENOMINATOR_VALUE)?;

            if index > u16::MAX { return Err(arbitrary::Error::IncorrectFormat); }
            if trade_fee_rate >= FEE_RATE_DENOMINATOR_VALUE { return Err(arbitrary::Error::IncorrectFormat) }
            if protocol_fee_rate > FEE_RATE_DENOMINATOR_VALUE { return Err(arbitrary::Error::IncorrectFormat) }
            if fund_fee_rate > FEE_RATE_DENOMINATOR_VALUE { return Err(arbitrary::Error::IncorrectFormat) }

            match fund_fee_rate.checked_add(protocol_fee_rate) {
                Some(val) => {
                    if val > FEE_RATE_DENOMINATOR_VALUE {
                        return Err(arbitrary::Error::IncorrectFormat);
                    }
                }
                None => return Err(arbitrary::Error::IncorrectFormat),
            }
    
            Ok(CreateAmmConfigData { index, trade_fee_rate, protocol_fee_rate, fund_fee_rate, create_pool_fee })
        }
    }

    impl<'a> Arbitrary<'a> for UpdateAmmConfigData {
        fn arbitrary(
            u: &mut arbitrary::Unstructured<'a>
        ) -> arbitrary::Result<Self> {
            let index = return_index();
            let param: u8 = u.int_in_range(0..=8)?; // match statement only available for 8 paths
            let index: u16 = index;//u.int_in_range(0..=MAX_INDEX_VALUE_FOR_AMM_CONFIG)?;
            let value: u64 = u.int_in_range(0..=FEE_RATE_DENOMINATOR_VALUE / 2)?;
            
            if value > FEE_RATE_DENOMINATOR_VALUE { return Err(arbitrary::Error::IncorrectFormat) }

            Ok(UpdateAmmConfigData { param, index, value })
        }
    }

    impl<'a> Arbitrary<'a> for InitializeData {
        fn arbitrary(
            u: &mut arbitrary::Unstructured<'a>
        ) -> arbitrary::Result<Self> {
            let index = return_index();
            let init_amount_0: u64 = u.int_in_range(0..=u64::MAX)?; 
            let init_amount_1: u64 = u.int_in_range(0..=u64::MAX)?;
            let open_time: u64 = u.int_in_range(0..=u64::MAX)?;

            Ok(InitializeData { index, init_amount_0, init_amount_1, open_time })
        }
    }

    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        amm_config: AccountsStorage<PdaStore>,
        associated_token_program: AccountsStorage<ProgramStore>,
        authority: AccountsStorage<PdaStore>,
        create_pool_fee: AccountsStorage<TokenStore>,
        creator: AccountsStorage<Keypair>,
        creator_lp_token: AccountsStorage<TokenStore>,
        creator_token_0: AccountsStorage<TokenStore>,
        creator_token_1: AccountsStorage<TokenStore>,
        // input_token_account: AccountsStorage<todo!()>,
        // input_token_mint: AccountsStorage<todo!()>,
        // input_token_program: AccountsStorage<todo!()>,
        // input_vault: AccountsStorage<todo!()>,
        lp_mint: AccountsStorage<MintStore>,
        // memo_program: AccountsStorage<todo!()>,
        observation_state: AccountsStorage<PdaStore>,
        // output_token_account: AccountsStorage<todo!()>,
        // output_token_mint: AccountsStorage<todo!()>,
        // output_token_program: AccountsStorage<todo!()>,
        // output_vault: AccountsStorage<todo!()>,
        owner: AccountsStorage<Keypair>,
        // owner_lp_token: AccountsStorage<todo!()>,
        // payer: AccountsStorage<todo!()>,
        pool_state: AccountsStorage<Keypair>, // Can be  a Keypair
        // recipient_token_0_account: AccountsStorage<todo!()>,
        // recipient_token_1_account: AccountsStorage<todo!()>,
        rent: AccountsStorage<ProgramStore>,
        system_program: AccountsStorage<ProgramStore>,
        // token_0_account: AccountsStorage<todo!()>,
        token_0_mint: AccountsStorage<MintStore>,
        token_0_program: AccountsStorage<ProgramStore>,
        token_0_vault: AccountsStorage<PdaStore>,
        // token_1_account: AccountsStorage<todo!()>,
        token_1_mint: AccountsStorage<MintStore>,
        token_1_program: AccountsStorage<ProgramStore>,
        token_1_vault: AccountsStorage<PdaStore>,
        token_program: AccountsStorage<ProgramStore>,
        // token_program_2022: AccountsStorage<todo!()>,
        // vault_0_mint: AccountsStorage<todo!()>,
        // vault_1_mint: AccountsStorage<todo!()>,
    }
}
