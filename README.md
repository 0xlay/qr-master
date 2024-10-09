# QR-Master

<p align="center">
    <img src=".github/assets/intro.png" alt="intro image" />
</p>

`QR-Master` is a simple command-line tool for generating QR codes from data. It supports multiple image formats, including JPG, PNG, and BMP.

## Features
- Generate QR codes from any text data.
- Save the generated QR code in the following formats: JPG, PNG, BMP.
- Easy-to-use command-line interface.

## Installation

Clone the repository and compile the project:
```bash
git clone https://github.com/0xlay/qr-master.git
cd qr-master
cargo build --release
```
The binary will be available in the target/release directory.

## Usage

To generate a QR code, run the following command:

```bash
qr-master <format> -d "<data>" -p <output_file>
```

## Supported Formats
* jpg
* png
* bmp

## Contributing

We welcome contributions to this project. If you’d like to contribute:
1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request.

We are happy to review any proposed changes.

## License

This project is licensed under the MIT license - see the LICENSE file for details.
