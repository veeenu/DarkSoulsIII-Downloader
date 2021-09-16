# Dark Souls III Downloader

Automatically download Dark Souls III speedrun patches.

Simply [download the latest release](https://github.com/veeenu/DarkSoulsIII-Downloader/releases), extract
it in a folder, double click `darksouls3-downloader.exe` and follow the
instructions. EZ PZ!

## Building

In case you wish to contribute. Building the project simply involves launching `build.py`.
You need Docker, Python 3.x and stable Rust/Cargo installed.
Building this was possible thanks to [DepotDownloader](https://github.com/SteamRE/DepotDownloader)
and [Manifest patcher](https://github.com/fifty-six/zig.SteamManifestPatcher).

```> python build.py```

- A `build` directory will be created
- [DepotDownloader](https://github.com/SteamRE/DepotDownloader) will be checked out in a Docker
  image, compiled, and its artifacts will be copied in `build/tmp/DepotDownloader`
- [Manifest patcher](https://github.com/fifty-six/zig.SteamManifestPatcher)'s latest release
  will be downloaded and uncompressed in `build/tmp`.
- `cargo build --release` will be issued.
- A working release will be copied in `build/package` and a compressed package will be created in
  `build/darksouls3-downloader.zip`.

Enjoy!

## License

The project is licensed under GNU GPLv2 out of compatibility with DepotDownloader and Manifest patcher.
