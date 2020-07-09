use std::cmp;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
use libchum::{format::TotemFormat, ChumArchive};

pub mod json;
pub mod util;

fn load_archive_raw(
    matches: &clap::ArgMatches,
) -> Result<(libchum::dgc::TotemArchive, libchum::ngc::TotemNameTable), Box<dyn Error>> {
    let namepath = Path::new(matches.value_of_os("NAMES").unwrap());
    let datapath = Path::new(matches.value_of_os("DATA").unwrap());
    let typeval = if matches.is_present("ngc") {
        TotemFormat::NGC
    } else if matches.is_present("ps2") {
        TotemFormat::PS2
    } else {
        panic!("No format given");
    };
    let mut namefile = File::open(namepath)?;
    let mut datafile = File::open(datapath)?;
    Ok((
        libchum::dgc::TotemArchive::read_from(&mut datafile, typeval)?,
        libchum::ngc::TotemNameTable::read_from(&mut namefile)?,
    ))
}

fn load_archive(matches: &clap::ArgMatches) -> Result<ChumArchive, Box<dyn Error>> {
    let namepath = Path::new(matches.value_of_os("NAMES").unwrap());
    let datapath = Path::new(matches.value_of_os("DATA").unwrap());
    let typeval = if matches.is_present("ngc") {
        TotemFormat::NGC
    } else if matches.is_present("ps2") {
        TotemFormat::PS2
    } else {
        panic!("No format given");
    };
    let mut namefile = File::open(namepath)?;
    let mut datafile = File::open(datapath)?;
    ChumArchive::read_chum_archive(&mut namefile, &mut datafile, typeval)
}

/// Info command.
/// Gets information about the given archive.
fn cmd_info(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let (dgc, ngc) = load_archive_raw(matches)?;
    let chunk_size = dgc.get_chunk_size();
    let mut max_file_size = 0usize;
    let mut min_file_size = usize::max_value();
    let mut num_files = 0;
    let mut total_size = 0;
    for (chunk_i, chunk) in dgc.iter_chunks().enumerate() {
        let mut chunk_total_size = 0;
        for file in chunk.iter_files() {
            let file_total_size = file.get_total_size();
            chunk_total_size += file_total_size;
            total_size += file_total_size;
            num_files += 1;
            max_file_size = cmp::max(max_file_size, file_total_size);
            min_file_size = if min_file_size == 0 {
                file_total_size
            } else {
                cmp::min(min_file_size, file_total_size)
            }
        }
        let padding_size = chunk_size - chunk_total_size;
        println!(
            "Chunk {:>3}: {:>3} files {:>8}B data {:>8}B padding",
            chunk_i,
            chunk.get_num_files(),
            chunk_total_size,
            padding_size
        );
    }
    println!("Chunk size: {}B ({0:X})", chunk_size);
    let average_size = total_size / num_files;
    println!(
        "Total size: {}B, num files: {}, average file size: {}B",
        total_size, num_files, average_size
    );
    println!(
        "Minimum size: {}B, Maximum size: {}B",
        min_file_size, max_file_size
    );
    let archive = ChumArchive::merge_archives(ngc, dgc)?;
    let unused = archive.find_unused_names();
    println!("There are {} unused names", unused.len());
    for v in unused {
        println!("    {}", v);
    }
    Ok(())
}

/// List command.
/// Lists all of the files in the given archive.
fn cmd_list(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let archive = load_archive(matches)?;
    let mut maxnamelen = 4;
    let mut maxtypelen = 4;
    let mut maxsubtypelen = 7;
    for file in archive.get_files() {
        maxnamelen = cmp::max(maxnamelen, file.get_name_id().len());
        maxtypelen = cmp::max(maxtypelen, file.get_type_id().len());
        maxsubtypelen = cmp::max(maxsubtypelen, file.get_subtype_id().len());
    }
    println!(
        "{0:8} {1:>2$} {3:>4$} {5:>6$}",
        "HASH", "NAME", maxnamelen, "TYPE", maxtypelen, "SUBTYPE", maxsubtypelen
    );
    for file in archive.get_files() {
        let id = util::hash_name_u32(file.get_name_id());
        println!(
            "{0:08X} {1:>2$} {3:>4$} {5:>6$}",
            id,
            file.get_name_id(),
            maxnamelen,
            file.get_type_id(),
            maxtypelen,
            file.get_subtype_id(),
            maxsubtypelen
        );
    }
    Ok(())
}

/// Extract command.
/// Extracts the data from an archive into a folder and a json file.
fn cmd_extract(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let archive = load_archive(matches)?;
    let output_path = Path::new(matches.value_of_os("OUTPUT").unwrap());
    fs::create_dir_all(&output_path)?;
    let merge = if matches.is_present("replace") {
        false
    } else if matches.is_present("merge") {
        true
    } else {
        panic!("Neither merge nor replace are present.");
    };
    json::extract_archive(&archive, &output_path, merge)?;
    println!("Extraction successful");
    Ok(())
}

/// Pack command.
/// Pack the extracted .json and data folder back into archive files.
fn cmd_pack(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let input_path = Path::new(matches.value_of_os("INPUT").unwrap());
    let namepath = Path::new(matches.value_of_os("NAMES").unwrap());
    let datapath = Path::new(matches.value_of_os("DATA").unwrap());
    let typeval = if matches.is_present("ngc") {
        TotemFormat::NGC
    } else if matches.is_present("ps2") {
        TotemFormat::PS2
    } else {
        panic!("No format given");
    };
    let archive = json::import_archive(&input_path, typeval)?;
    let mut ngc_file = File::create(namepath)?;
    let mut dgc_file = File::create(datapath)?;
    archive.write_chum_archive(&mut ngc_file, &mut dgc_file)?;
    println!("Packing successful");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = clap_app!(chumcli =>
        (version: "0.1.0")
        (author: "Jocelyn \"Jellonator\" B. <jellonator00@gmail.com>")
        (about: "Edits Totem archive files")
        (name: "Chum World")
        (@subcommand info =>
            (about: "Get information about the given archive")
            (@arg NAMES: +required "The name table file to use")
            (@arg DATA: +required "The data file to use")
            (@group type +required =>
                (@arg ngc: --ngc "Use Nintendo Gamecube format")
                (@arg ps2: --ps2 "Use Playstation 2 format")
            )
        )
        (@subcommand list =>
            (about: "Lists the contents of the given archive")
            (@arg NAMES: +required "The name table file to use")
            (@arg DATA: +required "The data file to use")
            (@group type +required =>
                (@arg ngc: --ngc "Use Nintendo Gamecube format")
                (@arg ps2: --ps2 "Use Playstation 2 format")
            )
        )
        (@subcommand extract =>
            (about: "Extracts the given archive to a folder")
            (@arg NAMES: +required "The name table file to use")
            (@arg DATA: +required "The data file to use")
            (@group type +required =>
                (@arg ngc: --ngc "Use Nintendo Gamecube format")
                (@arg ps2: --ps2 "Use Playstation 2 format")
            )
            (@arg OUTPUT: +required "The folder to extract to")
            (@group handler +required =>
                (@arg merge: --merge "Merge with existing folder")
                (@arg replace: --replace "Replace existing folder")
            )
        )
        (@subcommand pack =>
            (about: "Packs an folder into an archive")
            (@arg NAMES: +required "The name table file to use")
            (@arg DATA: +required "The data file to use")
            (@group type +required =>
                (@arg ngc: --ngc "Use Nintendo Gamecube format")
                (@arg ps2: --ps2 "Use Playstation 2 format")
            )
            (@arg INPUT: +required "The folder to read from")
        )
    );
    let matches = app.clone().get_matches();
    if let Some(cmdlist) = matches.subcommand_matches("list") {
        cmd_list(cmdlist)?;
    } else if let Some(cmdlist) = matches.subcommand_matches("info") {
        cmd_info(cmdlist)?;
    } else if let Some(cmdlist) = matches.subcommand_matches("extract") {
        cmd_extract(cmdlist)?;
    } else if let Some(cmdlist) = matches.subcommand_matches("pack") {
        cmd_pack(cmdlist)?;
    } else {
        app.print_long_help()?;
        println!();
    }
    Ok(())
}
