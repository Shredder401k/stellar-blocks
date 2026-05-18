# 🌌 stellar-blocks

`stellar-blocks` is a premium, highly modular, headless, and style-agnostic frontend UI component library built specifically for the Stellar and Soroban smart contract ecosystems.

Designed out of the box to streamline the development of decentralized applications (dApps), `stellar-blocks` maps directly to **Stellar Ecosystem Proposals (SEPs)** and complex on-chain state queries. Instead of rebuilding standard transaction flows, interactive anchor flows, or wallet connection states from scratch, developers can drop in highly composable UI blocks that handle the heavy lifting behind the scenes.

This repository is an open-source project actively participating in the **Drips Wave Program**. It is built as a developer-first environment optimized for rapid, decoupled, open-source task allocation ("Fix, Merge, and Earn").

---

## 🚀 Key Features

- **🧱 SEP-First Components:** Native UI components structured entirely around Stellar Ecosystem Proposals (including SEP-6, SEP-10, SEP-24, and SEP-30).
- **🔌 Wallet Agnostic Core:** Unified wallet connectivity hooks supporting **Freighter, Albedo, Rigo, and xBull** right out of the box.
- **🎨 Headless & Styleable:** Fully configured with Tailwind CSS configuration options, yet built using unstyled atomic configurations so you can drop them into any design system.
- **📦 Highly Composable:** Built within a `pnpm` monorepo workspace architecture allowing developers to export isolated components or the entire bundle.
- **💪 Fully Type-Safe:** Written 100% in TypeScript with comprehensive type-definitions matching the latest `stellar-sdk`.

---

## 🛠 Tech Stack

`stellar-blocks` leverages cutting-edge frontend tooling to ensure high performance and seamless developer workflows:

| Layer | Technology |
|---|---|
| Core Logic | TypeScript, `@stellar/stellar-sdk` |
| Smart Contracts | Rust, Soroban SDK |
| Monorepo Tooling | `pnpm` Workspaces, TurboRepo |
| Component Documentation | Storybook |
| Bundling Engine | `tsup` / Vite |
| Styling Engine | Tailwind CSS |
| Testing | Playwright (Component-driven), Vitest |

---

## 📁 Repository Architecture

The project is managed as a monorepo to separate core logic packages from framework-specific component layers and documentation.

```text
stellar-blocks/
├── .github/
│   └── workflows/          # CI/CD pipelines (Lint, Test, Release)
├── apps/
│   └── docs/               # Storybook & Documentation portal
├── packages/
│   ├── core/               # Non-UI hooks, State Machines, & SDK wrappers
│   ├── react/              # React-specific UI Components & Context Providers
│   └── tailwind-config/    # Shared design tokens & plugin utilities
├── contracts/              # Soroban smart contracts (Rust)
│   ├── example/            # Example starter contract
│   └── [contract-name]/    # Additional contracts for specific use cases
├── CONTRIBUTING.md         # Onboarding docs for Wave contributors
├── package.json
└── pnpm-workspace.yaml
```

---

## 💻 Getting Started for Consumers

### Installation

Install the core React component package along with the official Stellar SDK:

```bash
pnpm add @stellar-blocks/react @stellar/stellar-sdk
```

### Basic Usage: Wrapping Your App

Wrap your application in the `StellarBlocksProvider` to share wallet state and network configuration across your UI components.

```typescript
import React from 'react';
import { StellarBlocksProvider, WalletConnectButton, AssetBalanceList } from '@stellar-blocks/react';

function App() {
  return (
    <StellarBlocksProvider network="testnet">
      <header className="flex justify-between p-4 bg-slate-900 text-white">
        <h2>My Stellar dApp</h2>
        {/* Drop-in Multi-Wallet Connection Component */}
        <WalletConnectButton/>
      </header>

      <main className="p-8 max-w-xl mx-auto">
        <h3 className="text-xl font-bold mb-4">Your Wallet Assets</h3>
        <AssetBalanceList/>
      </main>
    </StellarBlocksProvider>
  );
}

export default App;
```

---

## 🌊 Drips Wave: Contributor Guide & Workflow

`stellar-blocks` is optimized for the Drips Wave program. Our goal is to break down massive protocol complexities into tiny, digestible, highly rewardable tasks for contributors.

### How to Earn Points

1. **Browse the Issue Tracker:** Look for open issues flagged with `Wave: Easy`, `Wave: Medium`, or `Wave: Advanced`. Each issue lists explicit point valuations (e.g., `[250 Points]`).
2. **Claim an Issue:** Comment on the issue to request assignment. *(Note: To ensure fair point distribution, please only claim one issue at a time.)*
3. **Build and Validate:** Follow the specific technical specification details inside the issue description. Ensure your code passes all lint and local test hooks.
4. **Submit PR:** Link your Pull Request directly to the tracking issue. Once reviewed, merged, and validated by the maintainers, your Drips rewards will be allocated!

### Local Development Setup

To begin building components locally, clone the repository and initialize the monorepo dependencies:

```bash
# Clone the repository
git clone https://github.com/your-username/stellar-blocks.git
cd stellar-blocks

# Install dependencies using pnpm
pnpm install

# Start the local Storybook dev environment to preview components
pnpm --filter docs dev
```

### Smart Contract Development Setup

To work with Soroban smart contracts:

```bash
# Navigate to contracts directory
cd contracts

# Run the setup script to install Rust, Soroban CLI, and configure testnet
chmod +x setup.sh
./setup.sh

# Build the example contract
cd example
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test
```

See `contracts/README.md` for detailed smart contract development instructions.

---

## 📄 Code Formatting & Standards

To maintain clean code quality throughout rapid 7-day sprint cycles, all contributors must strictly adhere to the following guardrails:

- **Design Enforcements:** Use Tailwind CSS utility classes exclusively. Avoid inline styling or custom CSS stylesheets unless handling iframe container dimensions.
- **State Separation:** Component logic (fetching, parsing SDK elements) must live inside custom hooks within `packages/core`. UI packages must remain strictly representational.
- **Documentation Requirement:** Every new component implementation PR must include a matching `.stories.tsx` file inside the Storybook directory to allow manual interaction review.

---

## 🛣 Core Roadmap (Planned Sprints)

The initial deployment timeline is mapped across standard component priorities. Check back during each active Wave phase for new issue batches:

- [ ] **Phase 1:** Core Wallets & State Hooks (`useStellarWallet`, `WalletConnectButton`, Horizon balance wrappers).
- [ ] **Phase 2:** Interactive Anchor Blocks (SEP-24 interactive modal handling, SEP-6 interactive parameters forms).
- [ ] **Phase 3:** Security & Smart Contracts (SEP-30 Multi-sig identity managers, Soroban invocation UI transaction builders).

---

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.
