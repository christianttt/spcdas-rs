mod cli;
mod decoder;
mod formatter;
mod instruction;
mod spc;

use clap::Parser;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::process;

fn main() -> io::Result<()> {
    let args = cli::Args::parse();

    let load_addr = cli::parse_hex(&args.load).unwrap_or_else(|e| {
        eprintln!(
            "Error: Invalid hex value for --load argument '{}': {}",
            args.load, e
        );
        process::exit(1);
    });

    let start_pc = match &args.pc {
        Some(pc_str) => cli::parse_hex(pc_str).unwrap_or_else(|e| {
            eprintln!(
                "Error: Invalid hex value for --pc argument '{}': {}",
                pc_str, e
            );
            process::exit(1);
        }),
        None => load_addr,
    };

    let rom_bytes = match fs::read(&args.input_file) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!(
                "Error: Could not read input file '{}': {}",
                args.input_file, e
            );
            process::exit(1);
        }
    };

    let stop_addr = if args.stop.eq_ignore_ascii_case("eof") {
        load_addr.wrapping_add(rom_bytes.len() as u16)
    } else {
        cli::parse_hex(&args.stop).unwrap_or_else(|e| {
            eprintln!(
                "Error: Invalid hex value for --stop argument '{}': {}",
                args.stop, e
            );
            process::exit(1);
        })
    };

    let config = spc::Config {
        show_addr: !args.no_addr,
        show_hex: !args.no_hex,
        resolve_rel: !args.no_rel_resolve,
    };

    let mut spc = spc::Spc::new(config, start_pc, stop_addr);
    spc.load_rom(&rom_bytes, load_addr);

    let mut writer: Box<dyn Write> = match fs::File::create(&args.output_file) {
        Ok(file) => Box::new(BufWriter::new(file)),
        Err(e) => {
            eprintln!(
                "Error: Could not create output file '{}': {}",
                args.output_file, e
            );
            process::exit(1);
        }
    };

    while spc.pc.wrapping_sub(spc.stop) != 0 {
        let pc = spc.pc;
        match decoder::decode_one(&spc, pc) {
            Ok(instr) => {
                let line = formatter::format_instruction(&instr, &spc);
                writeln!(writer, "{}", line)?;
                spc.pc = spc.pc.wrapping_add(instr.definition.len as u16);
            }
            Err(e) => {
                writeln!(
                    writer,
                    "{:04x}: db {:02x}    ; unknown opcode",
                    e.pc, e.opcode
                )?;
                spc.pc = spc.pc.wrapping_add(1);
            }
        }
    }

    Ok(())
}
