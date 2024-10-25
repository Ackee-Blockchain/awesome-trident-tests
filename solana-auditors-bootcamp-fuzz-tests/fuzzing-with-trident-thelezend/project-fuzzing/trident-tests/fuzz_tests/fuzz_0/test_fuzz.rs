use fuzz_instructions::locker_fuzz_instructions::{
    Claim, CreateVestingEscrow, CreateVestingEscrowMetadata, UpdateVestingEscrowRecipient,
};
use locker::entry as entry_locker;
use locker::ID as PROGRAM_ID_LOCKER;
const PROGRAM_NAME_LOCKER: &str = "locker";
use fuzz_instructions::locker_fuzz_instructions::FuzzInstruction as FuzzInstruction_locker;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_locker;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {
    fn pre_ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let create_vesting_escrow =
            FuzzInstruction::CreateVestingEscrow(CreateVestingEscrow::arbitrary(u)?);
        let create_vesting_escrow_metadata = FuzzInstruction::CreateVestingEscrowMetadata(
            CreateVestingEscrowMetadata::arbitrary(u)?,
        );

        Ok(vec![create_vesting_escrow, create_vesting_escrow_metadata])
    }

    fn ixs(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        let claim = FuzzInstruction::Claim(Claim::arbitrary(u)?);
        let update_vesting_escrow = FuzzInstruction::UpdateVestingEscrowRecipient(
            UpdateVestingEscrowRecipient::arbitrary(u)?,
        );

        Ok(vec![claim, update_vesting_escrow])
    }

    fn post_ixs(_u: &mut arbitrary::Unstructured) -> arbitrary::Result<Vec<FuzzInstruction>> {
        Ok(std::vec![])
    }
}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(PROGRAM_NAME_LOCKER,&PROGRAM_ID_LOCKER,processor!(convert_entry!(entry_locker)));

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_LOCKER, &mut client);
        });
    }
}
