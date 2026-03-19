# fcp
**Superfast, multithreaded & cross-platform file copy utility**

## Usage
```bash
$ fcp --help
Usage: fcp [OPTIONS] <INPUT> <OUTPUT>

Superfast, multithreaded & cross-platform file copy utility

Arguments:
  INPUT   Input file or directory to copy
  OUTPUT  Destination path

Options:
  -h, --help  Show this help message

$ fcp path/to/source path/to/destination
```

## Installing
### Downloading from releases
- [Windows](https://github.com/M336G/fcp/releases/latest/download/fcp-windows.exe)
- [macOS](https://github.com/M336G/fcp/releases/latest/download/fcp-macos)
- [Linux](https://github.com/M336G/fcp/releases/latest/download/fcp-linux)

To use `fcp` from anywhere, add it to your PATH:
- **On Windows**: see [this guide](https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14)#to-add-a-path-to-the-path-environment-variable)
- **On Unix**: move the binary to `/usr/local/bin/`

### Building from source
**Prerequisites:**
- [Rust](https://rust-lang.org/) installed

Run the following from your terminal (this may take some time):
```bash
cargo install --git https://github.com/M336G/fcp
```

## License
This project is licensed under the [MIT License](https://github.com/M336G/fcp/blob/main/LICENSE).