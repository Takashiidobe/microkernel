// test.rs

use crate::fs::MinixFileSystem;
use crate::syscall::{self, syscall_fs_read};
/// Test block will load raw binaries into memory to execute them. This function
/// will load ELF files and try to execute them.
pub fn test() {
    let files_inode = 26u32; // Change to yours!
    let files_size = 14776; // Change to yours!
    let bytes_to_read = 1024 * 50;
    let mut buffer = BlockBuffer::new(bytes_to_read);
    let bytes_read = syscall_fs_read(8, files_inode, buffer.get_mut(), bytes_to_read as u32, 0);
    if bytes_read != files_size {
        println!(
            "Unable to load program at inode {}, which should \
            be {} bytes, got {}",
            files_inode, files_size, bytes_read
        );
        return;
    }

    // The majority of the testing code needs to move into a system call (execv maybe?)
    MinixFileSystem::init(8);
    let path = "/shell\0".as_bytes().as_ptr();
    syscall::syscall_execv(path, 0);
    println!("I should never get here, execv should destroy our process.");
}
