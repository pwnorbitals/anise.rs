use anise::cli::args::{Actions, Args};
use anise::cli::CliErrors;
use anise::file_mmap;
use anise::naif::daf::DAF;
use anise::naif::spk::SPK;
use anise::prelude::*;
use clap::Parser;

fn main() -> Result<(), CliErrors> {
    let cli = Args::parse();
    match cli.action {
        Actions::Convert { file } => {
            let file_clone = file.clone();
            // Memory map the file
            match file_mmap!(file) {
                Ok(bytes) => {
                    let daf_file = DAF::parse(&bytes)?;
                    // Parse as SPK
                    let spk: SPK = (&daf_file).try_into()?;
                    // Convert to ANISE
                    let spk_filename = file_clone.to_str().unwrap();
                    let anise_filename = spk_filename.replace(".bsp", ".anise");
                    spk.to_anise(spk_filename, &anise_filename)?;
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        }
        Actions::IntegrityCheck { file: _ } => {
            todo!()
        }
        Actions::Merge { files } => {
            if files.len() < 2 {
                Err(CliErrors::ArgumentError(
                    "Need at least two files to merge together".to_string(),
                ))
            } else {
                // Open the last file in the list
                let destination = files.last().unwrap().clone();
                match file_mmap!(destination) {
                    Ok(bytes) => {
                        let mut context = AniseContext::try_from_bytes(&bytes)?;
                        // We can't borrow some bytes and let them drop out of scope, so we'll copy them into a vector.
                        let mut all_bytes = Vec::with_capacity(files.len() - 1);
                        for file_no in 0..files.len() - 1 {
                            // Try load this file
                            let this_file = &files[file_no];
                            match file_mmap!(this_file) {
                                Ok(these_bytes) => {
                                    all_bytes.push(these_bytes);
                                }
                                Err(e) => return Err(e.into()),
                            }
                        }
                        // And now add them to the previous context
                        for (num, bytes) in all_bytes.iter().enumerate() {
                            let other = AniseContext::try_from_bytes(&bytes)?;
                            let (num_ephem_added, num_orientation_added) =
                                context.merge_mut(other.clone())?;
                            println!("Added {num_ephem_added} ephemeris and {num_orientation_added} orientations from {:?}", files[num]);
                        }
                        // And finally save.
                        Ok(())
                    }
                    Err(e) => return Err(e.into()),
                }
                // Ok(())
            }
            // todo!("merge {files:?}")
        }
    }
}
