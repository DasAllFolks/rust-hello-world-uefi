#![no_std]
#![no_main]

use core::time::Duration;
use log::info;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<boot>) -> Status {
    // Initialize the uefi-rs loggers and helper infrastructure.
    uefi::helpers::init(&mut system_table).unwrap();

    // Log the message directly to the UEFI system console.
    info!("Hello, world!");

    // Sleep for 10 seconds so you can see the message before the app closes.
    system_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}
