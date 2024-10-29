use squads_multisig_program::entry as entry_squads_multisig_program;
use squads_multisig_program::ID as PROGRAM_ID_SQUADS_MULTISIG_PROGRAM;
const PROGRAM_NAME_SQUADS_MULTISIG_PROGRAM: &str =  "squads_multisig_program";
use fuzz_instructions::squads_multisig_program_fuzz_instructions::FuzzInstruction as FuzzInstruction_squads_multisig_program;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_squads_multisig_program;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_SQUADS_MULTISIG_PROGRAM,
                &PROGRAM_ID_SQUADS_MULTISIG_PROGRAM,
                processor!(convert_entry!(entry_squads_multisig_program))
            );

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_SQUADS_MULTISIG_PROGRAM, &mut client);
        });
    }
}

