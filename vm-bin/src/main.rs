use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Options {
    #[structopt(about = "Convert the given assembly file to machine code")]
    Assemble {
        #[structopt(
            about = "Path to write the compiled binary",
            short,
            long,
            parse(from_os_str),
        )]
        out: PathBuf,

        #[structopt(
            name = "FILE",
            about = "Assembly input to read",
            parse(from_os_str),
        )]
        file: PathBuf,
    },
    #[structopt(about = "Run a binary")]
    Run {
        #[structopt(
            about = "How much memory to give the VM",
            short,
            long = "memory",
            default_value = "0x10000",
            parse(try_from_str = parse_int::parse),
        )]
        memory_capacity: usize,

        #[structopt(
            name = "FILE",
            about = "Binary input to read",
            parse(from_os_str),
        )]
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();

    match options {
        Options::Assemble {
            out,
            file,
        } => {
            let bytes = {
                let mut buf = Vec::new();
                let mut file = File::open(file)?;
                file.read_to_end(&mut buf)?;
                buf
            };

            let parsed = vm_assembler::parse(&bytes)?;
            let assembled = vm_assembler::assemble(parsed);

            let mut outfile = File::create(out.clone())?;

            outfile.write(&assembled)?;

            if cfg!(unix) {
                use std::os::unix::fs::PermissionsExt;

                let meta = outfile.metadata()?;
                let mut perms = meta.permissions();
                perms.set_mode(0o755);

                std::fs::set_permissions(out.clone(), perms)?;
            }
        },
        Options::Run {
            memory_capacity,
            file
        } => {
            use vm::prelude::*;

            let bytes = {
                let mut buf = Vec::new();
                let mut file = File::open(file)?;
                file.read_to_end(&mut buf)?;
                buf
            };

            let mut memory = Memory::with_capacity(memory_capacity);
            memory.set_bytes(&bytes);

            let mut cpu = Cpu::from(memory);
            cpu.run();
        }
    }

    Ok(())
}
