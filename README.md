# Roost OS

A minimal operating system written in Rust featuring a separate kernel library, bootloader, and binary crate. This project is set up as a Cargo workspace with three members:

- **roost-kernel**: Contains the core OS functionality (no_std, initialization routines, etc.)
- **roost-os**: A thin binary wrapper that calls into the kernel.
- **roost-bootloader**: A bootloader crate that builds a bootable image and loads the kernel.
