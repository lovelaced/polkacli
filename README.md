# PolkaCLI

**PolkaCLI** is a command-line interface (CLI) tool designed for interacting with AssetHub, Polkadot's official rollup dedicated to assets. With PolkaCLI, users can mint NFTs, manage collections, check balances, and perform various other operations directly from the terminal.

## Features

- **NFT Management**
  - Mint new NFT collections and individual NFTs.
  - Display detailed information about specific NFTs and collections.
  - Set metadata of existing NFTs.

- **Account Management**
  - Configure and manage accounts using mnemonic phrases or secret URIs.
  - Set up and use a custom RPC URL for network interactions.
  - Check account balances and view account details.

- **Transactions**
  - Send funds to any address on the network.

## Installation

To install **PolkaCLI**, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/lovelaced/polkacli.git
cd polkacli
cargo build --release
```

Ensure you have Rust installed on your system. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).

## Configuration

Before using PolkaCLI, configure your account and RPC URL:

1. **Set Account**:
   ```bash
   polkacli set-account --mnemonic "<your mnemonic here>"
   ```
   or
   ```bash
   polkacli set-account --secret-uri "<your secret URI here>"
   ```
   This will save your account configuration to a local file in your home directory. WARNING: this currently saves the private key in plaintext.

2. **Set RPC URL**:
   ```bash
   polkacli set-rpc <your rpc url>
   ```

   This configures the RPC endpoint used for blockchain interactions. The default RPC is Paseo AssetHub provided by Dwellir.

3. **Optional Pinata JWT**:
   If you have a Pinata account and want to use it for pinning files to IPFS, you can add your Pinata JWT to the configuration file. This will enable PolkaCLI to pin files using your Pinata account instead of the default IPFS gateway.

## Usage

### NFT Minting Workflow

*It is recommended to use Pinata for pinning. Please add `pinata_jwt = "yourJWTsecret"` to `~/.polkacli/config`.*

When minting NFTs, PolkaCLI allows you to include metadata and images, either directly or inferred from filenames:

- **--json `<nft.json>`**:
  - If the `--json` argument is provided, the specified JSON file is loaded.
  - If the JSON file already contains an `"image"` field, PolkaCLI will use the link in that field directly.
  - If the `"image"` field is absent or empty, and you provide an image using `--image <image.jpg>`, PolkaCLI will pin the specified image to IPFS and update the JSON file with the IPFS link.
  - If no image is provided via `--image`, PolkaCLI will attempt to infer the image filename based on the JSON file's name (e.g., `nft.json` -> `nft.jpg`).
  - If no image is found or provided, the minting process will fail.

An example of valid NFT JSON is as follows:

```json
{
  "animation_url": "",
  "attributes": ["rare"],
  "description": "a cool nft",
  "external_url": "https://my.nft.shop",
  "image": "ipfs://Qmf4ECDXU4g4GDhAQQQpu6QpiE7GHJmmDvxGR1AeF4Atq3",
  "name": "Edition 1",
  "type": "image/png"
}
```
  
- **--image `<image.jpg>`**:
  - If the `--image` argument is provided, the specified image file is pinned to IPFS, and its link is added to the JSON file.
  - The `--json` argument is required when using `--image`. If not provided, the process will fail.

### Commands

Here is a summary of the available commands in PolkaCLI:

#### NFT Commands

- **mint-collection**:
  - Mint a new NFT collection. Optionally provide JSON metadata and images.
  - Example:
    ```bash
    polkacli mint-collection --json ./metadata --image ./images
    ```

- **mint-nft**:
  - Mint a new NFT within an existing collection. Supports optional metadata and image file handling as described above.
  - Example:
    ```bash
    polkacli mint-nft <collection_id> <nft_id> --json nft.json --image nft.jpg
    ```

- **set-nft-metadata**:
  - Set the metadata for an existing NFT within an existing collection. Supports metadata and image file handling as described above.
  - Example:
    ```bash
    polkacli set-nft-metadata <collection_id> <nft_id> --json nft.json --image nft.jpg
    ```

- **show-nft**:
  - Display details of a specific NFT, including its JSON metadata and image if requested.
  - Example:
    ```bash
    polkacli show-nft <collection_id> <nft_id> --json --image
    ```

- **show-collection**:
  - Retrieve and display details of a specific NFT collection.
  - Example:
    ```bash
    polkacli show-collection <collection_id>
    ```

#### Account and Transaction Commands

- **set-account**:
  - Configure the account to use with PolkaCLI, either via a mnemonic or secret URI.
  - Example:
    ```bash
    polkacli set-account --mnemonic "<your mnemonic here>"
    ```

- **set-rpc**:
  - Set the RPC URL for blockchain interactions.
  - Example:
    ```bash
    polkacli set-rpc "<rpc url>"
    ```

- **send**:
  - Send funds to a specified address.
  - Example:
    ```bash
    polkacli send <address> <amount>
    ```

- **balance**:
  - Check the balance of the configured account or another specified address.
  - Example:
    ```bash
    polkacli balance [optional: <address>]
    ```

- **account**:
  - Retrieve information about an account using its public key.
  - Example:
    ```bash
    polkacli account <public_key>
    ```

## Configuration File

The configuration file for PolkaCLI is stored in your home directory under `.polkacli/config`. This file stores the mnemonic, secret URI, RPC URL, and optionally, the Pinata JWT for IPFS pinning.

You can manually edit this file if necessary, or use the CLI commands to configure it.

## Contributing

Contributions are welcome! If you find a bug or have a feature request, feel free to open an issue or submit a pull request.

## License

PolkaCLI is licensed under the Apache 2.0 License. See the `LICENSE` file for more details.

---

**Disclaimer**: PolkaCLI comes with no warranty. Use at your own risk. The authors are not responsible for any loss of funds or other damages resulting from the use of this tool.
