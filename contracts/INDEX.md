# Contracts Directory - Documentation Index

Welcome to the stellar-blocks Soroban smart contracts directory! This index will help you navigate the documentation and get started quickly.

## 📚 Documentation Guide

### Getting Started (Read First!)

1. **[QUICKSTART.md](./QUICKSTART.md)** ⚡
   - 10-minute guide from zero to deployed contract
   - Perfect for first-time contributors
   - Includes all essential commands

2. **[README.md](./README.md)** 📖
   - Complete overview of the contracts directory
   - Prerequisites and installation
   - Building, testing, and deployment guides
   - Common commands reference

### For Contributors

3. **[CONTRIBUTING.md](./CONTRIBUTING.md)** 🤝
   - Contribution guidelines for Wave Program
   - Code standards and patterns
   - Testing requirements
   - PR checklist

4. **[CONTRACT_TEMPLATE.md](./CONTRACT_TEMPLATE.md)** 📝
   - Complete template for new contracts
   - Code examples for common patterns
   - README template
   - Testing template

### Integration & Architecture

5. **[INTEGRATION.md](./INTEGRATION.md)** 🔗
   - How contracts integrate with frontend
   - Architecture overview
   - Best practices for contract-to-component mapping
   - Type safety and testing strategies

## 🚀 Quick Navigation

### I want to...

#### ...get started immediately
→ Run `./setup.sh` then read [QUICKSTART.md](./QUICKSTART.md)

#### ...understand the project structure
→ Read [README.md](./README.md) sections 1-3

#### ...create a new contract
→ Follow [CONTRACT_TEMPLATE.md](./CONTRACT_TEMPLATE.md)

#### ...contribute to an existing contract
→ Read [CONTRIBUTING.md](./CONTRIBUTING.md)

#### ...integrate a contract with the frontend
→ Read [INTEGRATION.md](./INTEGRATION.md)

#### ...see an example
→ Check out `example/` directory and its [README](./example/README.md)

## 📂 Directory Structure

```
contracts/
├── INDEX.md                          # This file
├── README.md                         # Main documentation
├── QUICKSTART.md                     # Fast-track guide
├── CONTRIBUTING.md                   # Contribution guidelines
├── CONTRACT_TEMPLATE.md              # Template for new contracts
├── INTEGRATION.md                    # Frontend integration guide
├── Cargo.toml                        # Workspace configuration
├── Makefile                          # Common tasks automation
├── setup.sh                          # Environment setup script
├── rust-toolchain.toml               # Rust version specification
├── .gitignore                        # Git ignore rules
├── .github-workflows-example.yml     # CI/CD template
│
├── example/                          # Example starter contract
│   ├── src/
│   │   ├── lib.rs                   # Contract implementation
│   │   └── test.rs                  # Test suite
│   ├── Cargo.toml                   # Package configuration
│   └── README.md                    # Contract documentation
│
└── [your-contract]/                  # Your new contract here
    ├── src/
    │   ├── lib.rs
    │   └── test.rs
    ├── Cargo.toml
    └── README.md
```

## 🎯 Common Tasks

### Setup Environment
```bash
cd contracts
chmod +x setup.sh
./setup.sh
```

### Build All Contracts
```bash
make build
# or
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests
```bash
make test
# or
cargo test
```

### Deploy to Testnet
```bash
make deploy-testnet
# or
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/example.wasm \
  --source default \
  --network testnet
```

### Create New Contract
```bash
mkdir my-contract
cd my-contract
cargo init --lib
# Then follow CONTRACT_TEMPLATE.md
```

## 📋 Checklist for New Contributors

- [ ] Read [QUICKSTART.md](./QUICKSTART.md)
- [ ] Run `./setup.sh` to install dependencies
- [ ] Build and test the example contract
- [ ] Deploy example contract to testnet
- [ ] Read [CONTRIBUTING.md](./CONTRIBUTING.md)
- [ ] Browse open issues in the main repository
- [ ] Claim an issue and start building!

## 🔗 External Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Soroban SDK Reference](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Stellar Quest](https://quest.stellar.org/) - Interactive tutorials
- [Soroban Discord](https://discord.gg/stellar)

## 💡 Tips

- **Start with the example**: The `example/` contract demonstrates all key patterns
- **Use the Makefile**: Common tasks are automated - run `make help`
- **Test thoroughly**: All contracts must have comprehensive test coverage
- **Document everything**: Good documentation helps reviewers and future contributors
- **Ask questions**: Use GitHub Discussions or Discord for help

## 🏆 Wave Program

This project participates in the Drips Wave Program. Contract contributions are valued based on complexity:

- **Wave: Easy** (100-250 points): Simple contracts
- **Wave: Medium** (250-500 points): Moderate complexity
- **Wave: Advanced** (500-1000 points): Complex contracts with advanced features

See [CONTRIBUTING.md](./CONTRIBUTING.md) for details on earning points.

## 📞 Getting Help

1. Check the relevant documentation file above
2. Review the example contract
3. Search existing GitHub issues
4. Ask in GitHub Discussions
5. Join the Stellar Discord

## 🔄 Keeping Up to Date

This documentation is actively maintained. If you find errors or have suggestions:

1. Open an issue describing the problem
2. Submit a PR with improvements
3. Discuss in the project's forum

---

**Ready to build?** Start with [QUICKSTART.md](./QUICKSTART.md) and you'll have a deployed contract in 10 minutes! 🚀
