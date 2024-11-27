# Contributing to keygen-py

We welcome contributions to `keygen-py`. Whether you want to report a bug, improve the documentation, or contribute code, your help is greatly appreciated!

## How to Contribute

### Reporting Bugs

If you find a bug, please report it by [creating an issue](https://github.com/terra-quantum-public/keygen-py/issues). Please include as much detail as possible to help us understand and fix the issue.

### Suggesting Enhancements

If you have an idea to improve the SDK, please share it by [creating an issue](https://github.com/terra-quantum-public/keygen-py/issues) and describing the enhancement. We are open to suggestions!

### Submitting Code

1. **Fork the repository**: Click the "Fork" button at the top right of the repository page.

2. **Clone your fork**:
    ```sh
    git clone https://github.com/<YOUR_USERNAME>/keygen-py.git
    ```

3. **Add the upstream remote**:
    ```sh
    cd keygen-py
    git remote add upstream https://github.com/terra-quantum-public/keygen-py.git
    ```

4. **Create a branch**:
    ```sh
    git checkout -b feature/your-feature-name
    ```

5. **Make your changes**: Implement your feature or fix the bug.

6. **Commit your changes**:
    ```sh
    git add .
    git commit -m "Describe your changes"
    ```

7. **Push to your fork**:
    ```sh
    git push origin feature/your-feature-name
    ```

8. **Sync with upstream**:
   ```sh
   git fetch upstream
   git rebase upstream/main
   ```

9. **Create a Pull Request**: Go to the original repository and click the "New Pull Request" button. Provide a detailed description of your changes.

## Coding Standards

- Follow the [PEP 8](https://pep8.org/) coding style for Python.
- Write meaningful commit messages and PR descriptions.
- Include unit tests if applicable.

## Code of Conduct

Our project is dedicated to providing a respectful and harassment-free experience for everyone. Please review our [Code of Conduct](CODE_OF_CONDUCT.md) to understand the expectations we have for everyone who contributes to the project.

## Thank You

Thank you for considering contributing to `keygen-py`! Your contributions make this project better for everyone.
