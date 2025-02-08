use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::sync::Mutex;
use lazy_static::lazy_static;

const BUFFER_SIZE: usize = 10_000_000; // 10MB buffer for real-time market data

lazy_static! {
    static ref MMAP: Mutex<MmapMut> = {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/tmp/market_data_buffer")
            .expect("Failed to open mmap buffer file");

        file.set_len(BUFFER_SIZE as u64).expect("Failed to set mmap file size");

        let mmap = unsafe { MmapOptions::new().map_mut(&file).expect("Failed to create mmap") };
        Mutex::new(mmap)
    };
}

/// Synchronous function to write data to mmap buffer
pub fn write_to_mmap(data: &str) {
    let mut mmap = MMAP.lock().expect("Failed to acquire mmap lock");
    
    let bytes = data.as_bytes();
    let len = bytes.len().min(mmap.len());

    mmap[..len].copy_from_slice(&bytes[..len]);
    mmap.flush().expect("Failed to flush mmap buffer");

    println!("[MarketData] ✅ Data written to mmap.");
}

/// Reads latest market data from mmap buffer
pub fn read_from_mmap() -> String {
    let mmap = MMAP.lock().expect("Failed to acquire mmap lock");
    
    let content = String::from_utf8_lossy(&mmap[..]);
    let trimmed = content.trim_matches(char::from(0)).to_string(); // Remove null bytes

    println!("[MarketData] 📖 Read from mmap: {}", trimmed);
    
    trimmed
}
