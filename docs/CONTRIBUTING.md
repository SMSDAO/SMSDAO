# Contributing to SMSDAO

Thank you for your interest in contributing to SMSDAO! This guide will help you get started.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

## 🤝 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in all interactions.

### Expected Behavior

- Use welcoming and inclusive language
- Be respectful of differing viewpoints
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Trolling or insulting remarks
- Public or private harassment
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed:
- Rust 1.70+
- Solana CLI
- Anchor Framework 0.28+
- Node.js 18+ (for testing)
- Git

See [Getting Started Guide](GETTING_STARTED.md) for detailed installation instructions.

### Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/SMSDAO.git
cd SMSDAO

# Add upstream remote
git remote add upstream https://github.com/SMSDAO/SMSDAO.git

# Verify remotes
git remote -v
```

### Set Up Development Environment

```bash
# Install dependencies
cargo build

# Run tests to verify setup
cargo test

# Start local validator
solana-test-validator

# In another terminal, run the bot
cargo run
```

## 🔄 Development Workflow

### 1. Create a Branch

```bash
# Update your fork
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### Branch Naming Convention

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions or modifications
- `chore/` - Maintenance tasks

### 2. Make Changes

- Write clear, concise commit messages
- Follow coding standards (see below)
- Add tests for new functionality
- Update documentation as needed

### 3. Commit Your Changes

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: add new arbitrage algorithm

- Implement multi-DEX arbitrage detection
- Add unit tests for new algorithm
- Update documentation"
```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Build process or auxiliary tool changes

**Example:**
```
feat(arbitrage): add support for Jupiter DEX

- Integrate Jupiter DEX API
- Add price fetching for Jupiter
- Update arbitrage calculation to include Jupiter
- Add integration tests

Closes #123
```

### 4. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create pull request on GitHub
# Fill out the PR template completely
```

## 📝 Coding Standards

### Rust Style Guide

We follow the [Rust Style Guide](https://rust-lang.github.io/api-guidelines/) and use `rustfmt` and `clippy`.

#### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

#### Linting

```bash
# Run clippy
cargo clippy -- -D warnings

# Fix clippy warnings
cargo clippy --fix
```

### Code Style

**Good practices:**

```rust
// ✅ Good: Clear function names, documented
/// Calculates the net profit from an arbitrage opportunity
/// after accounting for fees and slippage.
pub fn calculate_net_profit(
    gross_profit: u64,
    gas_cost: u64,
    slippage: u64,
) -> Result<u64> {
    let net = gross_profit
        .checked_sub(gas_cost)
        .ok_or(Error::InsufficientProfit)?
        .checked_sub(slippage)
        .ok_or(Error::InsufficientProfit)?;
    Ok(net)
}

// ✅ Good: Proper error handling
match execute_trade(amount) {
    Ok(result) => log::info!("Trade successful: {:?}", result),
    Err(e) => {
        log::error!("Trade failed: {}", e);
        return Err(e);
    }
}

// ✅ Good: Type safety
#[derive(Debug, Clone, Copy)]
pub struct TradingPair {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
}
```

**Avoid:**

```rust
// ❌ Bad: Unwrap in production code
let result = execute_trade(amount).unwrap();

// ❌ Bad: Unclear variable names
let x = 100_000_000;
let y = x * 2;

// ❌ Bad: Magic numbers
if profit > 100000000 {  // What does this number mean?
    execute_trade();
}

// ✅ Better: Use constants
const MIN_PROFIT_THRESHOLD: u64 = 100_000_000;
if profit > MIN_PROFIT_THRESHOLD {
    execute_trade();
}
```

### Documentation

All public APIs must be documented:

```rust
/// Executes an arbitrage trade between two DEXs.
///
/// # Arguments
///
/// * `amount` - The amount of tokens to trade in base units
/// * `dex1` - The first DEX program ID
/// * `dex2` - The second DEX program ID
///
/// # Returns
///
/// * `Ok(u64)` - The profit in lamports
/// * `Err(Error)` - If the trade fails or is not profitable
///
/// # Example
///
/// ```
/// let profit = execute_arbitrage(
///     1_000_000_000,
///     raydium_program_id,
///     orca_program_id,
/// ).await?;
/// ```
pub async fn execute_arbitrage(
    amount: u64,
    dex1: Pubkey,
    dex2: Pubkey,
) -> Result<u64> {
    // Implementation
}
```

## 🧪 Testing Guidelines

### Writing Tests

All new features must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profit_calculation() {
        let gross = 1_000_000;
        let gas = 100_000;
        let slippage = 50_000;
        
        let net = calculate_net_profit(gross, gas, slippage).unwrap();
        assert_eq!(net, 850_000);
    }

    #[test]
    fn test_profit_calculation_insufficient() {
        let gross = 100_000;
        let gas = 200_000;
        let slippage = 50_000;
        
        let result = calculate_net_profit(gross, gas, slippage);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_arbitrage_execution() {
        let test_env = TestEnvironment::new().await;
        // Setup and test
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_profit_calculation

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test some-integration-tests

# Run with coverage
cargo tarpaulin --out Html
```

### Test Coverage

- Aim for >80% code coverage
- Test edge cases and error conditions
- Include integration tests for critical paths
- Use property-based testing where appropriate

## 📚 Documentation

### Code Documentation

- Document all public functions, structs, and modules
- Use clear, concise language
- Provide examples where helpful
- Keep documentation up-to-date

### Project Documentation

When adding features:
1. Update relevant documentation files
2. Add examples to docs
3. Update API reference if needed
4. Add to changelog

## 🔍 Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code follows style guidelines
- [ ] All tests pass locally
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] No merge conflicts with main branch

### PR Template

Fill out the PR template completely:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## How Has This Been Tested?
Describe the tests you ran

## Checklist
- [ ] My code follows the style guidelines
- [ ] I have performed a self-review
- [ ] I have commented my code where needed
- [ ] I have updated the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix/feature works
- [ ] New and existing tests pass locally
```

### Review Process

1. **Automated Checks**: CI must pass
2. **Code Review**: At least one approval required
3. **Testing**: Reviewer will test changes
4. **Discussion**: Address feedback and questions
5. **Approval**: Once approved, maintainer will merge

### After Merge

- Delete your branch
- Update your local repository
- Close any related issues

## 🎯 Areas for Contribution

### High Priority

- DEX integrations (Jupiter, Serum, etc.)
- Multi-chain support improvements
- Performance optimizations
- Security enhancements
- Documentation improvements

### Good First Issues

Look for issues labeled `good first issue` on GitHub.

### Feature Requests

Before implementing major features:
1. Check existing issues
2. Open a discussion or issue
3. Get feedback from maintainers
4. Submit proposal if needed

## 💬 Community

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Discord**: Real-time chat (link in README)
- **Twitter**: Updates and announcements

### Getting Help

- Check documentation first
- Search existing issues
- Ask in GitHub Discussions
- Join Discord for real-time help

### Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes
- Annual contributor reports

## 📜 License

By contributing to SMSDAO, you agree that your contributions will be licensed under the project's license.

## 🙏 Thank You!

Every contribution, no matter how small, is valuable. Thank you for helping make SMSDAO better!

---

**Questions?** Open an issue or discussion on GitHub.

**Related Documentation:**
- [Getting Started](GETTING_STARTED.md)
- [Architecture](ARCHITECTURE.md)
- [Auto Test](AUTO_TEST.md)
