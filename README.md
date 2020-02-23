# Dark Souls III Downloader

Automatically download Dark Souls III speedrun patches.

Simply [download the latest release](https://github.com/veeenu/DarkSoulsIII-Downloader/releases), extract
it in a folder, double click `darksouls3-downloader.exe` and follow the
instructions. EZ PZ!

## Building

In case you wish to contribute. Building the project simply involves launching `build.py`.
You need a recent version of .NET Core, Python 3.x and stable Rust/Cargo installed.
Building this was possible thanks to [DepotDownloader](https://github.com/SteamRE/DepotDownloader).

```> python build.py```

- A `build` directory will be created
- [DepotDownloader](https://github.com/SteamRE/DepotDownloader) will be checked out in `build/tmp`
- `cargo build --release` will be issued
- A working release will now be in `build/package`.

Enjoy!

## License

The project is licensed under GNU GPLv2 out of compatibility with DepotDownloader.