# Contributing Guide

Thanks for your interest in contributing to this project! 🎉

We welcome all kinds of contributions — bug reports, feature requests, code, documentation, tests, or ideas.

---

## 📦 Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
* `cargo` for building and testing
* Familiarity with GitHub and basic git usage

---

## 🛠️ Project Setup

```bash
git clone https://github.com/Felix-IX/dot_16.git
cd dot_16
cargo build
cargo run
```

### Run tests:

```bash
cargo test
```
---

## ✍️ How to Contribute

### 🐛 Reporting Bugs / Issues

* Use the issue template if available
* Include as much detail as possible: error logs, screenshots, OS info
* Prefer English if possible (中文也可以👌)

### 💡 Suggesting Features

* Open a feature request issue with a clear description
* If you're proposing major changes, consider starting a discussion first

### 💻 Submitting Pull Requests

* Fork the repo and create your branch from main or dev
* Use descriptive commit messages (see Gitmoji below 👇)
* Test your changes
* Submit a pull request and describe your changes clearly

---

## 🎨 Git Commit Convention

We use Gitmoji for commit messages:

Examples:

```markdown
✨ feat: add new sprite rendering engine
🐛 fix: correct byte order in screen memory layout
📚 docs: improve README with usage examples
🛠 chore: add dependabot.yml for dependency updates
```

Please keep commits atomic and meaningful.

---

## 🧪 Code Style & Testing

Use `cargo fmt` for formatting

Use `cargo clippy` to lint your code

Run `cargo test` before opening PRs

---

## 📜 License

By contributing, you agree that your contributions will be licensed under the same license as this project (see [LICENSE](../LICENSE)).

Feel free to open a discussion or issue if anything is unclear. Happy hacking! 🚀