use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io::Write;
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

/// Expose reference to the shared mmap buffer
pub fn get_mmap() -> &'static Mutex<MmapMut> {
    &MMAP
}

/// Write full JSON data into mmap buffer
pub fn write_to_mmap(data: &str) {
    let mut mmap = MMAP.lock().unwrap();
    let data_bytes = data.as_bytes();
    let len = data_bytes.len().min(mmap.len());

    mmap[..len].copy_from_slice(&data_bytes[..len]);
    mmap.flush().expect("Failed to flush mmap buffer");
}

/// Read from mmap buffer for real-time analysis
pub fn read_from_mmap() -> String {
    let mmap = MMAP.lock().unwrap();
    let content = String::from_utf8_lossy(&mmap[..]);
    content.trim().to_string()
}
