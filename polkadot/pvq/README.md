# PVQ: PolkaVM Query

PVQ is a unified query interface that bridges different chain runtime implementations and client tools/UIs.
PVQ provides an extension-based system where runtime developers can expose chain-specific functionality
through standardized interfaces, while allowing client-side developers to perform custom computations
on the data through PolkaVM programs.
By abstracting away concrete implementations across chains and supporting both off-chain and cross-chain scenarios,
PVQ aims to reduce code duplication and development complexity while maintaining flexibility for custom use cases.

## ✨ Features

- **🧩 Modular Extensions**: Extensible system for exposing runtime functionalities
- **🌐 Runtime Integration**: Seamless integration with Substrate runtimes
- **🔍 Rich Querying**: Support for complex queries involving functions from multiple pallets

## 🏗️ Architecture

The PVQ system consists of several interconnected components:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   PVQ Program   │───▶│  PVQ Executor   │───▶│ Substrate       │
│  (Guest Code)   │    │   (Host Side)   │    │ Runtime         │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │              ┌─────────────────┐              │
         └─────────────▶│ PVQ Extensions  │◀─────────────┘
                        │   (Modules)     │
                        └─────────────────┘
```

### Core Components

| Component | Description |
|-----------|-------------|
| **[PVQ Program](pvq-program/)** | Guest programs written in Rust that compile to RISC-V |
| **[PVQ Executor](pvq-executor/)** | Host-side component managing PolkaVM instances and runtime interaction |
| **[PVQ Extensions](pvq-extension/)** | Modular system exposing runtime functionalities to guest programs |
| **[PVQ Runtime API](pvq-runtime-api/)** | Substrate runtime API for external query submission |
| **[PVQ Primitives](pvq-primitives/)** | Common types and utilities shared across components |

### Available Example Extensions

- **[Core Extension](pvq-extension-core/)**: Fundamental functionalities and extension discovery
- **[Fungibles Extension](pvq-extension-fungibles/)**: Asset querying, balances, and metadata
- **[Swap Extension](pvq-extension-swap/)**: DEX interactions, liquidity pools, and price quotes

## 🚀 Getting Started

### Prerequisites

PVQ is part of the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) monorepo. Ensure you have:

- **Rust** (latest stable version)
- **Git** with submodule support (if cloning the monorepo)
- Access to the Polkadot SDK repository

### Installation


1. **Install required tools:**

   ```bash
   make tools
   ```

   This installs `polkatool` for ELF to PolkaVM blob conversion and `chain-spec-builder`.

2. **Build the PVQ components:**

   From the monorepo root, build specific PVQ packages:

   ```bash
   cargo build --release -p pvq-executor -p pvq-extension -p pvq-program -p pvq-primitives -p pvq-runtime-api
   ```

   Or build all PVQ-related packages:

   ```bash
   cargo build --release -p pvq-executor -p pvq-extension -p pvq-extension-core \
     -p pvq-extension-fungibles -p pvq-extension-swap -p pvq-program \
     -p pvq-program-metadata-gen -p pvq-primitives -p pvq-runtime-api
   ```

### Quick Start

#### Running Example Programs

1. **Build guest programs:**

   From the `polkadot/pvq` directory:

   ```bash
   make guests
   ```

   This builds all guest example programs and outputs PolkaVM blobs to `output/`.

2. **Use guest programs:**

   The built programs can be used with runtimes that implement the PVQ Runtime API.
   See the [Runtime Integration](#runtime-integration) section below.

#### Available Example Programs

| Program | Description |
|---------|-------------|
| `guest-sum-balance` | Sum balances of multiple accounts |
| `guest-total-supply` | Get total supply of an asset |
| `guest-sum-balance-percent` | Calculate percentage of total supply for account balances |
| `guest-swap-info` | Query DEX/swap information |

### Runtime Integration

PVQ can be integrated into any Substrate runtime. For a complete integration example, see:

- `cumulus/parachains/runtimes/assets/asset-hub-westend/src/pvq.rs` - An example of PVQ integration in Asset Hub Westend:

To integrate PVQ into your runtime:

1. **Add PVQ dependencies** to your runtime's `Cargo.toml`:
   - `pvq-executor`
   - `pvq-extension`
   - `pvq-runtime-api`
   - `pvq-primitives`

2. **Implement the PVQ Runtime API** in your runtime (see `pvq-runtime-api/README.md`)

3. **Define and implement extensions** using the `#[extension_decl]` and `#[extensions_impl]` macros

4. **Build and deploy** your runtime with PVQ support

For detailed integration instructions, refer to the individual component READMEs:
- [PVQ Extension README](pvq-extension/README.md)
- [PVQ Runtime API README](pvq-runtime-api/README.md)

### Demo of runtime integration

1. Compile the Asset Hub Westend runtime with PVQ integration:
```
cargo build asset-hub-westend-runtime
```

2. Launch a local parallel network using Chopsticks with the following configuration file (`westend-asset-hub.yml`):
> westend-asset-hub.yml
```json
endpoint: wss://westend-asset-hub-rpc.polkadot.io
mock-signature-host: true
block: ${env.WESTEND_ASSET_HUB_BLOCK_NUMBER}
db: ./db.sqlite
wasm-override: ./target/release/wbuild/asset-hub-westend-runtime/asset_hub_westend_runtime.wasm

import-storage:
  System:
    Account:
      -
        -
          - 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
        - providers: 1
          data:
            free: 1000000000000000
```
```
bunx @acala-network/chopsticks@latest --config westend-asset-hub.yml
```
3. Open the [PVQ Demo](https://open-web3-stack.github.io/-sdk/) (the sdk repository is available at
   [PVQ SDK](https://github.com/open-web3-stack/pvq-sdk)), go to the Swap tab, connect to your local network, and explore the available liquidity pools.
