# Contributing to Mortar

Thank you for your interest in contributing to Mortar! We appreciate your help in making this project better.

## Pull Requests

*   **Small Changes**: For small bug fixes, documentation typos, or minor improvements, feel free to open a Pull Request directly.
*   **Major Changes**: For significant changes, new features, or large refactorings, please **open an issue first** to discuss your proposal. This ensures that your work aligns with the project's goals and prevents wasted effort.

## Development Workflow

1.  **Fork** the repository on GitHub.
2.  **Clone** your fork locally.
3.  Create a new **branch** for your changes.
4.  Make your changes, adhering to the project's coding style.
5.  Run **tests** to ensure everything is working:
    ```bash
    cargo test
    ```
6.  Push your changes to your fork.
7.  Submit a **Pull Request** to the `main` branch of the original repository.

## Code Style

*   We follow standard Rust formatting conventions. Please run `cargo fmt` before submitting.
*   Ensure your code passes `cargo clippy` checks:
    ```bash
    cargo clippy --all-targets --all-features
    ```

## License

By contributing to Mortar, you agree that your contributions will be licensed under the project's dual license (MIT and Apache 2.0).
