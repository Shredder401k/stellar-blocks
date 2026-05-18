# Integrating Soroban Contracts with stellar-blocks Frontend

This guide explains how Soroban smart contracts in the `contracts/` directory integrate with the frontend UI components in the stellar-blocks library.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     stellar-blocks dApp                      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────┐         ┌──────────────────┐          │
│  │  React Components │         │   Core Hooks     │          │
│  │  (@stellar-blocks/│◄────────┤  (@stellar-blocks/│         │
│  │      react)       │         │      core)       │          │
│  └──────────────────┘         └──────────────────┘          │
│           │                            │                      │
│           │                            │                      │
│           ▼                            ▼                      │
│  ┌─────────────────────────────────────────────┐            │
│  │         @stellar/stellar-sdk                 │            │
│  │    (Horizon API + Soroban RPC Client)       │            │
│  └─────────────────────────────────────────────┘            │
│                       │                                       │
└───────────────────────┼───────────────────────────────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │     Stellar Network           │
        ├───────────────────────────────┤
        │  ┌─────────────────────────┐  │
        │  │  Soroban Smart Contracts │  │
        │  │  (contracts/ directory)  │  │
        │  └─────────────────────────┘  │
        └───────────────────────────────┘
```

## Integration Flow

### 1. Contract Development (Rust)

Contracts are developed in the `contracts/` directory:

```rust
// contracts/token-vault/src/lib.rs
#[contract]
pub struct TokenVault;

#[contractimpl]
impl TokenVault {
    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<(), Error> {
        // Contract logic
    }
}
```

### 2. Contract Deployment

Deploy to Stellar testnet/mainnet:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/token_vault.wasm \
  --source default \
  --network testnet
```

This returns a contract ID: `CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`

### 3. Frontend Hook Creation

Create a custom hook in `packages/core/`:

```typescript
// packages/core/src/hooks/useTokenVault.ts
import { SorobanRpc, Contract } from '@stellar/stellar-sdk';

export function useTokenVault(contractId: string) {
  const { wallet } = useStellarWallet();
  
  const deposit = async (amount: bigint) => {
    const contract = new Contract(contractId);
    
    // Build transaction
    const tx = new TransactionBuilder(/* ... */)
      .addOperation(contract.call('deposit', 
        wallet.address,
        amount
      ))
      .build();
    
    // Sign and submit
    const signed = await wallet.signTransaction(tx);
    const result = await server.sendTransaction(signed);
    
    return result;
  };
  
  return { deposit };
}
```

### 4. React Component Integration

Use the hook in React components:

```typescript
// packages/react/src/components/TokenVaultDeposit.tsx
import { useTokenVault } from '@stellar-blocks/core';

export function TokenVaultDeposit({ contractId }: Props) {
  const { deposit } = useTokenVault(contractId);
  const [amount, setAmount] = useState('');
  
  const handleDeposit = async () => {
    await deposit(BigInt(amount));
  };
  
  return (
    <div>
      <input 
        value={amount} 
        onChange={(e) => setAmount(e.target.value)} 
      />
      <button onClick={handleDeposit}>Deposit</button>
    </div>
  );
}
```

### 5. Consumer Usage

End users integrate the component:

```typescript
// Consumer's dApp
import { TokenVaultDeposit } from '@stellar-blocks/react';

function App() {
  return (
    <StellarBlocksProvider network="testnet">
      <TokenVaultDeposit 
        contractId="CXXXXXXXXX..." 
      />
    </StellarBlocksProvider>
  );
}
```

## Contract-to-Component Mapping

| Contract Type | Core Hook | React Component | Use Case |
|--------------|-----------|-----------------|----------|
| Token Vault | `useTokenVault` | `<TokenVaultDeposit>` | Asset management |
| NFT Minter | `useNFTMinter` | `<NFTMintForm>` | NFT creation |
| Governance | `useGovernance` | `<ProposalVoter>` | DAO voting |
| Staking | `useStaking` | `<StakeManager>` | Token staking |

## Best Practices

### 1. Contract Interface Stability

Keep contract interfaces stable to avoid breaking frontend integrations:

```rust
// Good: Versioned interface
pub fn deposit_v1(env: Env, amount: i128) -> Result<(), Error>

// Better: Use optional parameters for extensions
pub fn deposit(env: Env, amount: i128, memo: Option<String>) -> Result<(), Error>
```

### 2. Error Handling

Map contract errors to user-friendly messages:

```typescript
try {
  await deposit(amount);
} catch (error) {
  if (error.code === 1) {
    throw new Error('Contract not initialized');
  } else if (error.code === 3) {
    throw new Error('Insufficient balance');
  }
}
```

### 3. Event Listening

Subscribe to contract events in hooks:

```typescript
export function useTokenVault(contractId: string) {
  const [deposits, setDeposits] = useState([]);
  
  useEffect(() => {
    const subscription = server.getEvents({
      contractIds: [contractId],
      topics: [['deposit']],
    });
    
    subscription.on('data', (event) => {
      setDeposits(prev => [...prev, event]);
    });
    
    return () => subscription.close();
  }, [contractId]);
  
  return { deposits };
}
```

### 4. Type Safety

Generate TypeScript types from contract specs:

```bash
soroban contract bindings typescript \
  --wasm target/wasm32-unknown-unknown/release/token_vault.wasm \
  --output-dir packages/core/src/types/contracts/
```

Then use in hooks:

```typescript
import type { TokenVault } from '../types/contracts/token-vault';

export function useTokenVault(contractId: string): TokenVault {
  // Fully typed contract interface
}
```

## Testing Integration

### Contract Tests (Rust)

```rust
#[test]
fn test_deposit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenVault);
    let client = TokenVaultClient::new(&env, &contract_id);
    
    let user = Address::generate(&env);
    client.deposit(&user, &1000);
    
    assert_eq!(client.get_balance(&user), 1000);
}
```

### Hook Tests (TypeScript)

```typescript
import { renderHook, waitFor } from '@testing-library/react';
import { useTokenVault } from './useTokenVault';

test('deposit updates balance', async () => {
  const { result } = renderHook(() => 
    useTokenVault('CONTRACT_ID')
  );
  
  await result.current.deposit(1000n);
  
  await waitFor(() => {
    expect(result.current.balance).toBe(1000n);
  });
});
```

### Component Tests (React)

```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { TokenVaultDeposit } from './TokenVaultDeposit';

test('deposit button calls contract', async () => {
  render(<TokenVaultDeposit contractId="CONTRACT_ID" />);
  
  const input = screen.getByRole('textbox');
  const button = screen.getByRole('button');
  
  fireEvent.change(input, { target: { value: '1000' } });
  fireEvent.click(button);
  
  await waitFor(() => {
    expect(screen.getByText('Deposit successful')).toBeInTheDocument();
  });
});
```

## Development Workflow

### For Contract Contributors

1. Develop contract in `contracts/my-contract/`
2. Write comprehensive tests
3. Deploy to testnet
4. Document the interface in README
5. Submit PR with testnet contract ID

### For Frontend Contributors

1. Review contract interface from README
2. Create hook in `packages/core/`
3. Create React component in `packages/react/`
4. Write tests for hook and component
5. Add Storybook story
6. Submit PR linking to contract issue

### For Integration

1. Contract PR merged → Contract available on testnet
2. Frontend PR references contract ID
3. Integration tested on testnet
4. Both deployed to mainnet together

## Configuration Management

Store contract IDs in environment-specific configs:

```typescript
// packages/core/src/config/contracts.ts
export const CONTRACTS = {
  testnet: {
    tokenVault: 'CXXXXXXXXX...',
    nftMinter: 'CYYYYYYYYYY...',
  },
  mainnet: {
    tokenVault: 'CZZZZZZZZZ...',
    nftMinter: 'CWWWWWWWWW...',
  },
};

// Usage in hooks
const contractId = CONTRACTS[network].tokenVault;
```

## Monitoring and Analytics

Track contract interactions:

```typescript
export function useTokenVault(contractId: string) {
  const deposit = async (amount: bigint) => {
    // Track start
    analytics.track('contract_call_start', {
      contract: 'token_vault',
      method: 'deposit',
    });
    
    try {
      const result = await contract.deposit(amount);
      
      // Track success
      analytics.track('contract_call_success', {
        contract: 'token_vault',
        method: 'deposit',
        amount: amount.toString(),
      });
      
      return result;
    } catch (error) {
      // Track error
      analytics.track('contract_call_error', {
        contract: 'token_vault',
        method: 'deposit',
        error: error.message,
      });
      
      throw error;
    }
  };
  
  return { deposit };
}
```

## Resources

- [Stellar SDK Documentation](https://stellar.github.io/js-stellar-sdk/)
- [Soroban RPC Documentation](https://soroban.stellar.org/docs/reference/rpc)
- [Contract Bindings Guide](https://soroban.stellar.org/docs/getting-started/create-an-app)
- [stellar-blocks Core Hooks](../packages/core/README.md)
- [stellar-blocks React Components](../packages/react/README.md)

## Questions?

- For contract-specific questions: See `contracts/CONTRIBUTING.md`
- For frontend integration: See main `CONTRIBUTING.md`
- For architecture discussions: Open a GitHub Discussion
