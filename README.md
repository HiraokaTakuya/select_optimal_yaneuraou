# select\_optimal\_yaneuraou

## Usage

Put select\_optimal\_yaneuraou in the same directory as the YaneuraOu\_NNUE-normal-clang++-\*.exe.  
Execute select\_optimal\_yaneuraou and optimal YaneuraOu\_NNUE-normal-clang++-\*.exe is executed.  
For example, if your PC has an AMD Zen2 CPU, select\_optimal\_yaneuraou executes YaneuraOu\_NNUE-normal-clang++-zen2.exe.  
If your PC has an Intel Kaby Lake CPU, select\_optimal\_yaneuraou executes YaneuraOu\_NNUE-normal-clang++-avx2.exe.

## Build

1. Install rustup and cargo

If you use macOS, Linux, or another Unix-like OS, run the following in your terminal.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
If you use Windows, install [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and download the [rustup installer](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe).

See detail.
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. build select\_optimal\_yaneuraou
```bash
git clone https://github.com/select_optimal_yaneuraou.git && \
cd select_optimal_yaneuraou && \
cargo build --release
```

You can get a executable file at ```select_optimal_yaneuraou/target/release/select_optimal_yaneuraou```


## License

MIT
