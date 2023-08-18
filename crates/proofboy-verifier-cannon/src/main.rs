#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]

/// Defines the size of the heap in bytes
/// Changing this will change the size of the resulting json file built by converting the elf file
/// How big you can make this depends on the program size but it should be possible to make it very large (close to 4GB).
/// See https://image1.slideserve.com/3443033/memory-map-l.jpg
const HEAP_SIZE: usize = 0x4000000;

use cannon_heap::init_heap;
use cannon_io::prelude::*;
use extractor::{
    extractors::flappy_boy_score::FlappyBoyScoreExtractor, Extractor, Metadata,
};
use gameboy::Gameboy;
use journal::{Journal, KeyState};

extern crate alloc;

mod gameboy;

/// Main entrypoint for a verifiable computation
#[no_mangle]
pub extern "C" fn _start() {
    init_heap!(HEAP_SIZE);
    // this directory will exist in the docker builder image
    let rom = include_bytes!("/roms/flappyboy.gb");

    let mut oracle_reader = oracle_reader();

    // read the expected output metadata from the pre-image oracle
    let metadata_bytes = oracle_reader
        .get(PreimageKey::new_local(&[0x0]))
        .expect("Failed to read metadata from pre-image oracle");
    let expected_metadata: Metadata =
        serde_json::from_slice(&metadata_bytes).expect("Failed to parse metadata as json");

    // read the journal from the pre-image oracle
    let journal_bytes = oracle_reader
        .get(PreimageKey::new_local(&[0x1]))
        .expect("Failed to read journal form pre-image oracle");
    let journal = Journal::from_bytes(&journal_bytes);

    // apply the journal to our emulator and get the final memory state
    let mut gb: Gameboy = Gameboy::new(rom);
    journal.into_iter().for_each(|keys| {
        gb.kbd.0.replace(KeyState::from_byte(keys));
        gb.step();
    });

    // extract using the given extractor and compare with the expected output
    let result_metadata =
        FlappyBoyScoreExtractor::extract(&gb.sys).expect("Failed to extract metadata");

    if expected_metadata == result_metadata {
        print("Verified successfully!");
        exit(0); // 0 code indicates success
    } else {
        print("Metadata does not match!");
        exit(1); // 1 code indicates code ran successfully but verified to false
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let msg = alloc::format!("Panic: {}", info);
    let _ = print(&msg);
    exit(2);
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    let _ = print("alloc error! (probably out of memory)");
    exit(3);
}
