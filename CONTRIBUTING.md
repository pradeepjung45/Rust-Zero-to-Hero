# Contributing to Rust & Solana Learning Project

Thank you for your interest in contributing to this learning resource! 🦀🚀

## 🎯 Project Goals

This project aims to provide a **step-by-step, lightweight learning path** for Rust and Solana development that:
- Avoids heavy dependencies during early learning phases
- Provides practical, hands-on examples
- Builds knowledge progressively from basics to advanced topics
- Maintains clear, beginner-friendly explanations

## 🤝 How to Contribute

### 🐛 Reporting Bugs
- Use the GitHub issue tracker
- Include clear steps to reproduce
- Mention your Rust version and OS
- Provide error messages if applicable

### 📝 Improving Documentation
- Fix typos or unclear explanations
- Add more detailed examples
- Improve code comments
- Enhance learning guides

### ✨ Adding Examples
- Follow the existing code style
- Include clear comments explaining concepts
- Add practice exercises where appropriate
- Test your examples before submitting

### 🔧 Code Contributions
- Fork the repository
- Create a feature branch (`git checkout -b feature/amazing-feature`)
- Make your changes
- Test thoroughly
- Commit with clear messages
- Push to your branch
- Open a Pull Request

## 📋 Development Guidelines

### Code Style
- Follow Rust conventions (`cargo fmt`)
- Use meaningful variable names
- Add comments for complex concepts
- Include examples that beginners can understand

### Testing
- Ensure all examples compile and run
- Test with minimal dependencies approach
- Verify learning progression makes sense
- Check that exercises are appropriate for the skill level

### Documentation
- Use clear, beginner-friendly language
- Include practical examples
- Explain the "why" not just the "how"
- Add learning objectives for each section

## 🏗️ Project Structure

```
rust_solana/
├── src/day01/          # Day 1: Rust Basics
├── src/day02/          # Day 2: Control Flow (future)
├── docs/               # Learning guides
├── scripts/            # Helper scripts
└── README.md           # Main documentation
```

### Adding New Days
When adding new learning days:
1. Create appropriate directory structure
2. Include both morning and afternoon sessions
3. Add comprehensive learning guide in `docs/`
4. Update main README.md
5. Add minimal dependencies only when needed
6. Include practical exercises

## 🧪 Testing Your Contributions

Before submitting:
```bash
# Quick syntax check
./scripts/test-day01.sh

# Full compilation test
cargo check

# Run tests
cargo test

# Format code
cargo fmt
```

## 📚 Learning Philosophy

When contributing, keep in mind our learning philosophy:
- **Step-by-step progression** - Each concept builds on previous ones
- **Practical focus** - Real examples over theoretical explanations
- **Lightweight approach** - Avoid heavy dependencies until necessary
- **Beginner-friendly** - Clear explanations for newcomers

## 🎓 Content Guidelines

### For Code Examples
- Start simple, build complexity gradually
- Include comments explaining Rust concepts
- Show both correct usage and common mistakes
- Provide practice exercises

### For Documentation
- Use clear headings and structure
- Include code snippets with explanations
- Add learning objectives and success criteria
- Provide troubleshooting tips

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/rust_solana`
3. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
4. Test the setup: `./scripts/test-day01.sh`
5. Make your improvements
6. Submit a Pull Request

## 📞 Questions?

- Open an issue for questions about contributing
- Check existing issues and PRs first
- Be respectful and constructive in discussions

Thank you for helping make Rust and Solana more accessible to learners! 🙏
