# FF-CARL CLI Tool

## Overview

FF-CARL CLI Tool is a command-line utility designed to automate the management of Firefox's mTLS host:certificate
preference assignment file (`ClientAuthRememberList.bin`). This tool simplifies adding client certificates for specific
hosts, schemes, and ports, ensuring consistent and secure mTLS configurations across Firefox profiles.

## Prerequisites

- Rust and Cargo installed
- Firefox installed
- `nginx` or another web server installed and configured for mTLS (optional, for testing)

## Installation

1. **Clone the repository:**
```sh
git clone https://github.com/yourusername/ff-carl-cli
cd ff-carl-cli
```

2. **Build the project:**

```sh
cargo build --release
```

## Usage

### Adding a Client Certificate

To add a client certificate entry to the `ClientAuthRememberList.bin` file for a specific host and port, use the following command:

```sh
./target/release/ff-carl-cli \
  --scheme http \
  --host dmytro.nations-original.com \
  --port 80 \
  --domain nations-original.com \
  --cert /path/to/testcert.der \
  --output /path/to/firefox/profile/ClientAuthRememberList.bin
```

### Parameters

- `--scheme` (`-s`): The URL scheme (http or https) for the host.
- `--host` (`-h`): The host (e.g., `dmytro.dev.medserv.ie`)
- `--port` (`-p`): The port (e.g., `443`)
- `--domain` (`-d`): The base domain (e.g., `medserv.ie`)
- `--cert` (`-c`): The path to the DER-encoded client certificate (e.g., `/path/to/client.der`)
- `--output` (`-o`): The path to the Firefox profile's `ClientAuthRememberList.bin `file (e.g., `/home/user/mozilla/firefox/wsccqc81.ff-carl-test/ClientAuthRememberList.bin`)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Acknowledgments

- [ff-carl](https://github.com/andrewoswald/ff-carl) - The Rust library used for managing Firefox's mTLS ClientAuthRememberList.bin file.