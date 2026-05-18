# Contributing to stellar-blocks 🌌

Thank you for your interest in contributing to `stellar-blocks`! This project is actively participating in the **Drips Wave Program** — a structured, sprint-based open-source initiative where every merged contribution earns you real rewards.

This guide covers everything you need to go from zero to your first merged PR.

---

## 📖 Table of Contents

- [Code of Conduct](#-code-of-conduct)
- [How the Drips Wave Program Works](#-how-the-drips-wave-program-works)
- [Prerequisites](#-prerequisites)
- [Local Development Setup](#-local-development-setup)
- [Repository Structure](#-repository-structure)
- [Contribution Workflow](#-contribution-workflow)
- [Component Guidelines](#-component-guidelines)
- [Code Standards](#-code-standards)
- [Testing](#-testing)
- [Submitting a Pull Request](#-submitting-a-pull-request)
- [Review Process](#-review-process)

---

## 🤝 Code of Conduct

All contributors are expected to engage respectfully and constructively. We are committed to maintaining a welcoming environment for developers of all experience levels. Harassment, discrimination, or bad-faith behavior of any kind will not be tolerated.

---

## 🌊 How the Drips Wave Program Works

`stellar-blocks` uses the Drips Wave Program to allocate rewards to open-source contributors. Here's how it works:

1. Issues are created and tagged with a difficulty label and a point value:
   - `Wave: Easy` — typically `50–150 Points`
   - `Wave: Medium` — typically `150–350 Points`
   - `Wave: Advanced` — typically `350–700 Points`

2. You claim an issue, build the feature or fix, and open a PR.
3. Once your PR is reviewed, approved, and merged by a maintainer, your point allocation is pushed to your Drips account automatically.

> **Important:** Please hold only **one active issue claim at a time**. This ensures fair distribution across all contributors.

---

## 🛠 Prerequisites

Before setting up the project locally, ensure you have the following installed:

- **Node.js** `18+`
- **pnpm** `8+` — install via `npm install -g pnpm`
- **Git**

Familiarity with the following will help significantly:

- React and TypeScript
- Tailwind CSS utility classes
- The Stellar ecosystem (wallets, SEPs, Horizon)
- Storybook for component documentation

---

## 💻 Local Development Setup

```bash
# 1. Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/stellar-blocks.git
cd stellar-blocks

# 2. Install all monorepo dependencies
pnpm install

# 3. Start the Storybook development server
pnpm --filter docs dev
```

Storybook will be available at `http://localhost:6006`. All components render here — this is your primary development environment.

To build all packages:

```bash
pnpm build
```

To run the full test suite:

```bash
pnpm test
```

---

## 📁 Repository Structure

Understanding where things live will help you find what to work on:

```text
stellar-blocks/
├── apps/
│   └── docs/               # Storybook — your dev preview environment
├── packages/
│   ├── core/               # Hooks, state machines, SDK wrappers (NO UI here)
│   ├── react/              # React UI components and context providers
│   └── tailwind-config/    # Shared design tokens and Tailwind plugin config
```

The key architectural rule: **logic lives in `core`, UI lives in `react`**. These must never be mixed.

---

## 🔄 Contribution Workflow

### 1. Find and Claim an Issue

Browse the [Issue Tracker](https://github.com/your-username/stellar-blocks/issues) and filter by `Wave: Easy`, `Wave: Medium`, or `Wave: Advanced`. Read the full issue description — each one includes a technical specification you must follow.

Comment on the issue:

```
I'd like to work on this. Claiming it now.
```

Wait for a maintainer to assign it to you before starting work.

### 2. Create a Feature Branch

```bash
git checkout -b feat/your-feature-name
# or for bug fixes:
git checkout -b fix/issue-description
```

### 3. Build Your Contribution

Follow the technical spec in the issue exactly. Refer to the [Component Guidelines](#-component-guidelines) and [Code Standards](#-code-standards) sections below.

### 4. Test Locally

```bash
# Run unit tests
pnpm test

# Verify Storybook renders correctly
pnpm --filter docs dev
```

### 5. Commit and Push

```bash
git add .
git commit -m "feat(react): add WalletConnectButton component"
git push origin feat/your-feature-name
```

### 6. Open a Pull Request

Open a PR from your fork to the `main` branch of `stellar-blocks`. Use the PR template and include `Closes #<issue-number>` in the description.

---

## 🧱 Component Guidelines

### State vs. UI Separation

This is the most important architectural rule in the project:

- **`packages/core`** — all data fetching, Stellar SDK calls, state machines, and custom hooks. No JSX, no Tailwind, no UI of any kind.
- **`packages/react`** — purely representational components that consume hooks from `core`. No direct SDK calls, no async data fetching.

```typescript
// ✅ CORRECT — hook lives in packages/core
export function useStellarBalance(address: string) {
  const [balance, setBalance] = useState<string | null>(null);
  useEffect(() => {
    fetchBalance(address).then(setBalance);
  }, [address]);
  return balance;
}

// ✅ CORRECT — component in packages/react consumes the hook
export function AssetBalanceList() {
  const balance = useStellarBalance(useWallet().address);
  return <div className="...">{balance}</div>;
}
```

### Tailwind CSS Only

Use Tailwind utility classes exclusively for all styling. Do not write custom CSS stylesheets or use inline `style` props — the only exception is setting `iframe` container dimensions when absolutely necessary.

### Storybook Stories

Every new component PR **must** include a matching `.stories.tsx` file. Stories should cover:

- The default state
- All meaningful prop variants
- Loading and error states where applicable

```typescript
// MyComponent.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import { MyComponent } from './MyComponent';

const meta: Meta<typeof MyComponent> = {
  title: 'React/MyComponent',
  component: MyComponent,
};
export default meta;

export const Default: StoryObj<typeof MyComponent> = {};
export const Loading: StoryObj<typeof MyComponent> = { args: { isLoading: true } };
```

---

## 📄 Code Standards

### TypeScript

All code must be written in TypeScript. Avoid `any` — use proper types from `@stellar/stellar-sdk` wherever applicable.

### Formatting & Linting

Run these before every commit:

```bash
pnpm lint
pnpm format
```

PRs with lint errors will not be reviewed until they are resolved.

### No Inline Styling

```tsx
// ❌ Never do this
<div style={{ color: 'red', padding: '16px' }}>...</div>

// ✅ Always use Tailwind
<div className="text-red-500 p-4">...</div>
```

---

## 🧪 Testing

Tests live alongside the packages they cover. Run the full suite with:

```bash
pnpm test
```

For component-level testing, Playwright is used. For hook and utility testing, Vitest is used. New components should have at minimum:

- A render test confirming the component mounts without errors
- A test for each meaningful interactive state

---

## 📬 Submitting a Pull Request

When opening your PR, please:

- Use a descriptive title: `feat(react): add SEP-24 deposit modal component`
- Fill out the PR template completely
- Include `Closes #<issue-number>` to link to your claimed issue
- Attach screenshots or Storybook recordings for any visual changes
- Ensure all CI checks pass before requesting review

---

## 🔍 Review Process

Maintainers aim to review open PRs within **3 business days**. During review, you may be asked to make changes — please respond to review comments promptly to keep the sprint cycle moving.

---

Thank you for contributing to `stellar-blocks` and the Stellar open-source ecosystem. Every component you ship makes it faster for the next developer to build on Stellar. 🚀
