//! STMUSB4500 utility
//!
//! This demo is intended to run on a linux host (e.g. a Raspberry Pi) and shows how to read and
//! write the NVM over I²C. It dumps the existing NVM and then writes the factory default - as
//! generated by the [ST GUI application][gui] - so be careful.
//!
//! [gui]: https://www.st.com/en/embedded-software/stsw-stusb002.html

use std::fs::File;
use std::io::{Read,Write};
use std::path::PathBuf;
use clap::{Parser,Subcommand};
use linux_embedded_hal::I2cdev;
use stusb4500::{Address, STUSB4500, PdoChannel, pdo};
use pdo::Pdo;

const I2C_BUS: &str = "i2c-0";

const DEFAULT_NVM_DATA: [[u8; 8]; 5] = [
    [0x00, 0x00, 0xB0, 0xAB, 0x00, 0x45, 0x00, 0x00],
    [0x10, 0x40, 0x9C, 0x1C, 0xFF, 0x01, 0x3C, 0xDF],
    [0x02, 0x40, 0x0F, 0x00, 0x32, 0x00, 0xFC, 0xF1],
    [0x00, 0x19, 0x56, 0xAF, 0xF5, 0x35, 0x5F, 0x00],
    [0x00, 0x4B, 0x90, 0x21, 0x43, 0x00, 0x40, 0xFB],
];

/// Utility to read and write STUSB4500 NVM
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Read register block from STUSB4500 NVM
    Read {
	/// Optional I2C bus for access to stusb4500
        #[arg(short, long, default_value=I2C_BUS)]
    	bus: Option<String>,

        /// Sets an input file
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
    /// Write register block to STUSB4500 NVM
    Write {
	/// Optional I2C bus for access to stusb4500
        #[arg(short, long, default_value=I2C_BUS)]
    	bus: Option<String>,

        /// Set a custom output file
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    /// Write factory reset register block to STUSB4500 NVM
    FactoryReset {
	/// Optional I2C bus for access to stusb4500
        #[arg(short, long, default_value=I2C_BUS)]
    	bus: Option<String>,
    },
    /// Show status information
    Status {
	/// Optional I2C bus for access to stusb4500
        #[arg(short, long, default_value=I2C_BUS)]
    	bus: Option<String>,
    },
}

fn main() {
let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Read { bus, file }) => {
		println!("Reading NVM data:");
		let  mut bus_path = bus.clone().unwrap();
		bus_path.insert_str(0, "/dev/");
		let mut mcu = STUSB4500::new(I2cdev::new(bus_path).unwrap(), Address::Default);
		let mut nvm = mcu.unlock_nvm().unwrap();
		let sectors = nvm.read_sectors().unwrap();
		nvm.lock().unwrap();

		if let Some(dereffile) = file.as_deref() {
			let mut f = File::create(dereffile).expect("Couldn't create file");
			sectors.iter().for_each(|sector| {
				f.write_all(sector).expect("Failed to write");
			});
		} else {
			sectors.iter().for_each(|sector| {
				sector.iter().for_each(|byte| print!(" 0x{:02X}", byte));
				println!();
			});
		}
        },
        Some(Commands::Write { bus, file }) => {

		// Read the file
		let mut f = File::open(file).expect("File not found");

		let mut buffer: [u8; 40] = [0; 40];
		f.read(&mut buffer).expect("Buffer overflow");
		let mut sectors: [[u8; 8]; 5] = [[0;8];5];

		for slice in 0..5 {
		  for idx in 0..8 {
		    sectors[slice][idx] = buffer[8*slice + idx];
		  }
		}

		println!("Writing NVM data...");
		let  mut bus_path = bus.clone().unwrap();
		bus_path.insert_str(0, "/dev/");
		let mut mcu = STUSB4500::new(I2cdev::new(bus_path).unwrap(), Address::Default);
		let mut nvm = mcu.unlock_nvm().unwrap();
		nvm.write_sectors(sectors).unwrap();
		nvm.lock().unwrap();
        },
        Some(Commands::FactoryReset { bus }) => {
		let  mut bus_path = bus.clone().unwrap();
		bus_path.insert_str(0, "/dev/");
		println!("Writing factory default NVM data...");
		let mut mcu = STUSB4500::new(I2cdev::new(bus_path).unwrap(), Address::Default);
		let mut nvm = mcu.unlock_nvm().unwrap();
		nvm.write_sectors(DEFAULT_NVM_DATA).unwrap();
		nvm.lock().unwrap();
        },
        Some(Commands::Status { bus }) => {
		let  mut bus_path = bus.clone().unwrap();
		bus_path.insert_str(0, "/dev/");
		let mut mcu = STUSB4500::new(I2cdev::new(bus_path).unwrap(), Address::Default);

		println!("PDO1:");
		match mcu.get_pdo(PdoChannel::PDO1).unwrap() {
			Pdo::Fixed(pdo) => {
				println!("{pdo:?}");
//				println!("- fixed                         {}", pdo1.fixed() );
//				println!("- higher_capability             {}", pdo1.higher_capability() );
//				println!("- unconstrained_power           {}", pdo1.unconstrained_power() );
//				println!("- usb_communications_capable    {}", pdo1.usb_communications_capable() );
//				println!("- dual_role_data                {}", pdo1.dual_role_data() );
//				println!("- fast_role_swap                {}", pdo1.fast_role_swap() );
//				println!("- voltage                       {}", pdo1.voltage() );
//				println!("- current                       {}", pdo1.current() );
			},
			Pdo::Variable(pdo) => {
				println!("{pdo:?}");
			},
			_ => {
				println!("The other one");
			}
		}

		let voltage = mcu.get_voltage().unwrap();
		println!("Voltage {} V", voltage);

		let current_rdo = mcu.get_current_rdo().unwrap();
		println!("{current_rdo:?}");
//		println!("Current RDO:");
//		println!("- position                    {}", current_rdo.position() );
//		println!("- give_back                   {}", current_rdo.give_back() );
//		println!("- capability_mismatch         {}", current_rdo.capability_mismatch() );
//		println!("- usb_communication_capable   {}", current_rdo.usb_communication_capable() );
//		println!("- no_usb_suspend              {}", current_rdo.no_usb_suspend() );
//		println!("- unchunked_extended_messages {}", current_rdo.unchunked_extended_messages() );
//		println!("- operating_current           {} mA", current_rdo.operating_current() );
//		println!("- max_operating_currernt      {} mA", current_rdo.max_operating_current() );
        },
        None => {}
    }
}