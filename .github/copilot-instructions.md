# Copilot instructions

- This project does not require backward compatibility. Prefer a clean,
  correct design over compatibility shims when changing public APIs,
  configuration, or generated code.
- Treat breaking behavior as intentional only when it is documented and the
  affected tests and callers are updated.
- Native ABI correctness remains required for generated FFI bindings.
