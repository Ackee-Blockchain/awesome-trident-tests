pub mod locker_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use locker::CreateVestingEscrowParameters;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        CreateVestingEscrow(CreateVestingEscrow),
        Claim(Claim),
        CreateVestingEscrowMetadata(CreateVestingEscrowMetadata),
        UpdateVestingEscrowRecipient(UpdateVestingEscrowRecipient),
    }

    #[derive(Arbitrary, Debug)]
    pub struct CreateVestingEscrow {
        pub accounts: CreateVestingEscrowAccounts,
        pub data: CreateVestingEscrowData,
    }

    #[derive(Arbitrary, Debug)]
    pub struct CreateVestingEscrowAccounts {
        pub base: AccountId,
        pub escrow: AccountId,
        pub escrow_token: AccountId,
        pub sender: AccountId,
        pub sender_token: AccountId,
        pub recipient: AccountId,
        pub token_program: AccountId,
        pub system_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,

        // Manually added
        pub token_mint: AccountId,
    }

    #[derive(Arbitrary, Debug)]
    pub struct CreateVestingEscrowData {
        pub params: locker::instructions::create_vesting_escrow::CreateVestingEscrowParameters,
    }

    #[derive(Arbitrary, Debug)]
    pub struct Claim {
        pub accounts: ClaimAccounts,
        pub data: ClaimData,
    }

    #[derive(Arbitrary, Debug)]
    pub struct ClaimAccounts {
        pub escrow: AccountId,
        pub escrow_token: AccountId,
        pub recipient: AccountId,
        pub recipient_token: AccountId,
        pub token_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,

        // Manually added
        pub base: AccountId,
        pub token_mint: AccountId,
    }

    #[derive(Arbitrary, Debug)]
    pub struct ClaimData {
        pub max_amount: u64,
    }

    #[derive(Arbitrary, Debug)]
    pub struct CreateVestingEscrowMetadata {
        pub accounts: CreateVestingEscrowMetadataAccounts,
        pub data: CreateVestingEscrowMetadataData,
    }

    #[derive(Arbitrary, Debug)]
    pub struct CreateVestingEscrowMetadataAccounts {
        pub escrow: AccountId,
        pub creator: AccountId,
        pub escrow_metadata: AccountId,
        pub payer: AccountId,
        pub system_program: AccountId,

        // Manually added
        pub base: AccountId,
    }

    #[derive(Arbitrary, Debug)]
    pub struct CreateVestingEscrowMetadataData { pub params : locker :: instructions :: create_vesting_escrow_metadata :: CreateVestingEscrowMetadataParameters }

    #[derive(Arbitrary, Debug)]
    pub struct UpdateVestingEscrowRecipient {
        pub accounts: UpdateVestingEscrowRecipientAccounts,
        pub data: UpdateVestingEscrowRecipientData,
    }

    #[derive(Arbitrary, Debug)]
    pub struct UpdateVestingEscrowRecipientAccounts {
        pub escrow: AccountId,
        pub escrow_metadata: AccountId,
        pub signer: AccountId,
        pub system_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,

        // Manually added
        pub base: AccountId,
        pub new_recipient: AccountId,
    }

    #[derive(Arbitrary, Debug)]
    pub struct UpdateVestingEscrowRecipientData {
        pub new_recipient: AccountId,
        pub new_recipient_email: Option<String>,
    }

    impl<'info> IxOps<'info> for CreateVestingEscrow {
        type IxData = locker::instruction::CreateVestingEscrow;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreateVestingEscrowSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = locker::instruction::CreateVestingEscrow {
                params: CreateVestingEscrowParameters {
                    vesting_start_time: self.data.params.vesting_start_time,
                    cliff_time: self.data.params.cliff_time,
                    frequency: self.data.params.frequency,
                    cliff_unlock_amount: self.data.params.cliff_unlock_amount,
                    amount_per_period: self.data.params.amount_per_period,
                    number_of_period: self.data.params.number_of_period,
                    update_recipient_mode: 3,
                },
            };
            Ok(data)
        }

        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let token_owner = Keypair::new();
            let token_mint = fuzz_accounts
                .token_mint
                .get_or_create_account(
                    self.accounts.token_mint,
                    client,
                    8,
                    &token_owner.pubkey(),
                    None,
                )
                .unwrap();

            let base = fuzz_accounts.base.get_or_create_account(
                self.accounts.base,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[b"escrow".as_ref(), base.pubkey().as_ref()],
                    &locker::ID,
                )
                .unwrap();

            let escrow_token = fuzz_accounts
                .escrow_token
                .get_or_create_account(
                    self.accounts.escrow_token,
                    &[
                        &escrow.pubkey.to_bytes(),
                        &anchor_spl::token::spl_token::ID.to_bytes(),
                        &token_mint.to_bytes(),
                    ],
                    &anchor_spl::associated_token::spl_associated_token_account::ID,
                )
                .unwrap();
            client.set_account_custom(
                &escrow_token.pubkey(),
                &create_token_account_state(
                    token_mint,
                    escrow.pubkey(),
                    10000000,
                    None,
                    None,
                    0,
                    Some(escrow.pubkey()),
                ),
            );

            let sender = fuzz_accounts.sender.get_or_create_account(
                self.accounts.sender,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            let sender_token = client.set_token_account(
                token_mint,
                sender.pubkey(),
                u64::MAX,
                None,
                None,
                0,
                Some(escrow.pubkey()),
            );
            let recipient = fuzz_accounts.recipient.get_or_create_account(
                self.accounts.recipient,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            let token_program = anchor_spl::token::ID;
            let system_program = anchor_lang::system_program::ID;
            let event_authority =
                Pubkey::find_program_address(&[b"__event_authority"], &locker::ID).0;
            let program = locker::ID;

            let acc_meta = locker::accounts::CreateVestingEscrowCtx {
                base: base.pubkey(),
                escrow: escrow.pubkey(),
                escrow_token: escrow_token.pubkey(),
                sender: sender.pubkey(),
                sender_token,
                recipient: recipient.pubkey(),
                token_program,
                system_program,
                event_authority,
                program,
            }
            .to_account_metas(None);
            let signers = vec![base, sender];

            Ok((signers, acc_meta))
        }

        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            if let Ok(deposit_amount) = ix_data.params.get_total_deposit_amount() {
                if post_ix.sender_token.amount != pre_ix.sender_token.amount - deposit_amount {
                    return Err(FuzzingError::Custom(1));
                }

                if post_ix.escrow_token.amount != pre_ix.escrow_token.amount + deposit_amount {
                    return Err(FuzzingError::Custom(2));
                }
            }

            Ok(())
        }
    }

    impl<'info> IxOps<'info> for Claim {
        type IxData = locker::instruction::Claim;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ClaimSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = locker::instruction::Claim {
                max_amount: self.data.max_amount,
            };
            Ok(data)
        }

        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let token_owner = Keypair::new();
            let token_mint = fuzz_accounts
                .token_mint
                .get_or_create_account(
                    self.accounts.token_mint,
                    client,
                    8,
                    &token_owner.pubkey(),
                    None,
                )
                .unwrap();
            let recipient = fuzz_accounts.recipient.get_or_create_account(
                self.accounts.recipient,
                client,
                100 * LAMPORTS_PER_SOL,
            );

            let base = fuzz_accounts.base.get_or_create_account(
                self.accounts.base,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[b"escrow".as_ref(), base.pubkey().as_ref()],
                    &locker::ID,
                )
                .unwrap();

            let escrow_token = fuzz_accounts
                .escrow_token
                .get_or_create_account(
                    self.accounts.escrow_token,
                    &[
                        &escrow.pubkey.to_bytes(),
                        &anchor_spl::token::spl_token::ID.to_bytes(),
                        &token_mint.to_bytes(),
                    ],
                    &anchor_spl::associated_token::spl_associated_token_account::ID,
                )
                .unwrap();
            client.set_account_custom(
                &escrow_token.pubkey(),
                &create_token_account_state(
                    token_mint,
                    escrow.pubkey(),
                    10000000,
                    None,
                    None,
                    0,
                    Some(escrow.pubkey()),
                ),
            );

            let recipient_token = client.set_token_account(
                token_mint,
                recipient.pubkey(),
                0,
                None,
                None,
                0,
                Some(recipient.pubkey()),
            );

            let token_program = anchor_spl::token::ID;
            let event_authority =
                Pubkey::find_program_address(&[b"__event_authority"], &locker::ID).0;
            let program = locker::ID;

            let acc_meta = locker::accounts::ClaimCtx {
                escrow: escrow.pubkey(),
                escrow_token: escrow_token.pubkey(),
                recipient: recipient.pubkey(),
                recipient_token,
                token_program,
                event_authority,
                program,
            }
            .to_account_metas(None);
            let signers = vec![recipient];
            Ok((signers, acc_meta))
        }

        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            // Check if the recipient has received tokens
            // Claim amount can be 0, thats why we use >
            if pre_ix.recipient_token.amount > post_ix.recipient_token.amount {
                return Err(FuzzingError::Custom(3));
            }

            // Check if the escrow has sent tokens
            if pre_ix.escrow_token.amount < post_ix.escrow_token.amount {
                return Err(FuzzingError::Custom(4));
            }

            Ok(())
        }
    }

    impl<'info> IxOps<'info> for CreateVestingEscrowMetadata {
        type IxData = locker::instruction::CreateVestingEscrowMetadata;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreateVestingEscrowMetadataSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = locker::instruction::CreateVestingEscrowMetadata {
                params: locker::CreateVestingEscrowMetadataParameters {
                    name: "dummy_name".to_owned(),
                    description: "dummy_description".to_owned(),
                    creator_email: "dummy_creator_email".to_owned(),
                    recipient_email: "dummy_recipient_email".to_owned(),
                },
            };
            Ok(data)
        }

        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let base = fuzz_accounts.base.get_or_create_account(
                self.accounts.base,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[b"escrow".as_ref(), base.pubkey().as_ref()],
                    &locker::ID,
                )
                .unwrap();

            // sender is the creator
            let creator = fuzz_accounts.sender.get_or_create_account(
                self.accounts.creator,
                client,
                100 * LAMPORTS_PER_SOL,
            );

            let escrow_metadata = fuzz_accounts
                .escrow_metadata
                .get_or_create_account(
                    self.accounts.escrow_metadata,
                    &[b"escrow_metadata".as_ref(), escrow.pubkey().as_ref()],
                    &locker::ID,
                )
                .unwrap();
            let acc_meta = locker::accounts::CreateVestingEscrowMetadataCtx {
                escrow: escrow.pubkey(),
                creator: creator.pubkey(),
                escrow_metadata: escrow_metadata.pubkey(),
                payer: creator.pubkey(),
                system_program: anchor_lang::system_program::ID,
            }
            .to_account_metas(None);
            let signers = vec![creator];

            Ok((signers, acc_meta))
        }
    }

    impl<'info> IxOps<'info> for UpdateVestingEscrowRecipient {
        type IxData = locker::instruction::UpdateVestingEscrowRecipient;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpdateVestingEscrowRecipientSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let new_recipient = fuzz_accounts.recipient.get_or_create_account(
                self.accounts.new_recipient,
                client,
                100 * LAMPORTS_PER_SOL,
            );
            let data = locker::instruction::UpdateVestingEscrowRecipient {
                new_recipient: new_recipient.pubkey(),
                new_recipient_email: Some("dummy_email".to_owned()),
            };
            Ok(data)
        }

        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let base = fuzz_accounts.base.get_or_create_account(
                self.accounts.base,
                client,
                100 * LAMPORTS_PER_SOL,
            );

            let signer = if rand::random() {
                fuzz_accounts.recipient.get_or_create_account(
                    self.accounts.signer,
                    client,
                    100 * LAMPORTS_PER_SOL,
                )
            } else {
                fuzz_accounts.sender.get_or_create_account(
                    self.accounts.signer,
                    client,
                    100 * LAMPORTS_PER_SOL,
                )
            };

            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[b"escrow".as_ref(), base.pubkey().as_ref()],
                    &locker::ID,
                )
                .unwrap();
            let escrow_metadata = fuzz_accounts
                .escrow_metadata
                .get_or_create_account(
                    self.accounts.escrow_metadata,
                    &[b"escrow_metadata".as_ref(), escrow.pubkey().as_ref()],
                    &locker::ID,
                )
                .unwrap();
            let event_authority =
                Pubkey::find_program_address(&[b"__event_authority"], &locker::ID).0;
            let program = locker::ID;
            let system_program = anchor_lang::system_program::ID;

            let acc_meta = locker::accounts::UpdateVestingEscrowRecipientCtx {
                escrow: escrow.pubkey(),
                escrow_metadata: Some(escrow_metadata.pubkey()),
                signer: signer.pubkey(),
                system_program,
                event_authority,
                program,
            }
            .to_account_metas(None);
            let signers = vec![signer];

            Ok((signers, acc_meta))
        }

        fn check(
            &self,
            _pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            // Check if the recipient is updated
            if post_ix.escrow.recipient != ix_data.new_recipient {
                return Err(FuzzingError::Custom(5));
            }

            Ok(())
        }
    }

    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        base: AccountsStorage<Keypair>,
        token_mint: AccountsStorage<MintStore>,
        // creator: AccountsStorage<Keypair>,
        escrow: AccountsStorage<PdaStore>,
        escrow_metadata: AccountsStorage<PdaStore>,
        escrow_token: AccountsStorage<PdaStore>,
        // Event authority is constant.
        // event_authority: AccountsStorage<Keypair>,
        // payer: AccountsStorage<Keypair>,
        // Program ID is constant for event CPI.
        // program: AccountsStorage<ProgramStore>,
        recipient: AccountsStorage<Keypair>,
        // recipient_token: AccountsStorage<TokenStore>,
        sender: AccountsStorage<Keypair>,
        // sender_token: AccountsStorage<TokenStore>,
        // signer: AccountsStorage<Keypair>,
        // system_program: AccountsStorage<ProgramStore>,
        // token_program: AccountsStorage<ProgramStore>,
    }

    /// Custom implementation for token_account
    fn create_token_account_state(
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> solana_sdk::account::AccountSharedData {
        use anchor_spl::token::spl_token;
        use solana_sdk::{
            account::AccountSharedData, program_option::COption, program_pack::Pack,
            sysvar::rent::Rent,
        };

        let delegate = match delegate {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let is_native = match is_native {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let close_authority = match close_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let lamports = r.minimum_balance(spl_token::state::Account::LEN);

        let mut account =
            AccountSharedData::new(lamports, spl_token::state::Account::LEN, &spl_token::id());

        let token_account = spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate,
            state: spl_token::state::AccountState::Initialized,
            is_native,
            delegated_amount,
            close_authority,
        };

        let mut data = vec![0u8; spl_token::state::Account::LEN];
        spl_token::state::Account::pack(token_account, &mut data[..]).unwrap();
        account.set_data_from_slice(&data);

        account
    }
}
