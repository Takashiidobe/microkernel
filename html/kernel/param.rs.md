# kernel/param.rs
```rust
// kernel/param.rs
pub const NCPU: usize = 8; // maximum number of CPUs
pub const NPROC: usize = 16; // maximum number of processes
pub const NOFILE: usize = 16; // open files per process
pub const NFILE: usize = 100; // open files per system
pub const NINODE: usize = 50; // maximum number of active i-nodes
pub const NDEV: usize = 10; // maximum major device number
pub const ROOTDEV: u32 = 1; // device number of file system root disk
pub const MAXARG: usize = 32; // max exec arguments
pub const MAXOPBLOCKS: usize = 10; // max # of blocks any FS op writes
pub const LOGSIZE: usize = MAXOPBLOCKS * 3; // max data blocks in on-disk log
pub const NBUF: usize = MAXOPBLOCKS * 3; // size of disk block cache
pub const FSSIZE: usize = 200000; // size of file system in blocks
pub const MAXPATH: usize = 128; // maximum file path name

```
