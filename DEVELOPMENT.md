# Nebula Development Guidelines

## Workspace Structure
- Все кодовые модули оформляются как отдельные crates внутри workspace.
- Используйте idiomatic Rust, следуйте naming conventions.

## Code Style
- Используйте `cargo fmt` для автоформатирования.
- Все предупреждения clippy должны быть исправлены (`cargo clippy --all-targets --all-features -- -D warnings`).

## Error Handling
- Используйте `Result<T, E>` и crate `thiserror` для ошибок.
- Не используйте unwrap/expect вне тестов.

## Async
- Для асинхронных операций используйте `tokio` и async/await.

## Testing
- Покрывайте код unit-тестами.
- Для моков используйте отдельные модули или crates.

## CI/CD
- Все PR проходят проверки: fmt, clippy, test.

## Pre-commit
- Перед коммитом запускайте:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all` 