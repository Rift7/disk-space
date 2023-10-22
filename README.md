# Disk Space

A terminal-based utility written in Rust that provides an overview of all the hard drives connected to the computer, showing their partitions and the available space for each in a visually appealing manner.

## Features

- Lists all hard drives and their partitions.
- Displays a bar indicating the used space on each partition.
- The progress bar changes color based on the percentage of space used.
- Clean and easy-to-read output.

## Installation

Ensure that you have [Rust nightly and Cargo installed](https://www.rust-lang.org/tools/install) on your machine.

Clone this repository:

```sh
git clone https://github.com/Rift7/disk-space.git
cd disk-space
cargo build --release
# and maybe install it for future use
cp /target/release/disk-space /usr/local/bin
```

## Usage

```sh
./target/release/disk-space
# OR if you installed it
disk-space
```

## Sample Output

```
[/dev/nvme0n1p5]
 └─ [/boot/efi] [▇▇:::::::::::::::::::::::::::::::: 1.91% :::::::::::::::::::::::::::::::::] [19.55 MB/1021.98 MB]
[/dev/nvme0n1p6]
 ├─ [/]         [▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇ 63.37% ▇▇▇▇▇▇:::::::::::::::::::::::::::] [170.64 GB/269.28 GB]
 └─ [/home]     [▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇ 63.37% ▇▇▇▇▇▇:::::::::::::::::::::::::::] [170.64 GB/269.28 GB]
```

## Dependencies

- `sysinfo` for fetching the system information.
- `termion` for handling terminal I/O and color.

## Contributing

Feel free to contribute to this project. Fork it, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/Rift7/disk-space/blob/main/LICENSE) file for details.
