# Structure Workspace

```
vibe_x-master (workspace)
├── libs/
│   ├── lib-account/
│   │   ├── docs/
│   │   │   ├── api.md
│   │   │   ├── architecture.md
│   │   │   ├── deployment.md
│   │   ├── migration/
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   ├── add_user.rs
│   │   │   │   ├── main.rs
│   │   │   ├── Cargo.toml
│   │   │   ├── README.md
│   │   ├── src/
│   │   │   ├── api/
│   │   │   │   ├── handlers/
│   │   │   │   │   ├── mod.rs
│   │   │   │   ├── router/
│   │   │   │   │   ├── mod.rs
│   │   │   │   ├── mod.rs
│   │   │   ├── config/
│   │   │   │   ├── mod.rs
│   │   │   ├── models/
│   │   │   │   ├── inputs.rs
│   │   │   │   ├── response.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── profile.rs
│   │   │   │   ├── user.rs
│   │   │   ├── repositories/
│   │   │   │   ├── error.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── service.rs
│   │   │   │   ├── mutation.rs
│   │   │   │   ├── query.rs
│   │   │   ├── utils/
│   │   │   │   ├── mod.rs
│   │   │   ├── error.rs
│   │   │   ├── lib.rs
│   │   ├── tests/
│   │   │   ├── integration_test.rs
│   │   │   ├── unit_test.rs
│   │   ├── .env
│   │   ├── Cargo.toml
│   ├── lib-utils/
│   │   ├── src/
│   │   │   ├── utils/
│   │   │   │   ├── b64.rs
│   │   │   │   ├── envs.rs
│   │   │   │   ├── mod.rs
│   │   │   ├── lib.rs
│   │   ├── Cargo.toml
├── Cargo.toml (workspace)
├── .gitignore
```

## Added structure with its characters.

* │ is created using the Unicode character U+2502.

* ├ is created using U+251C.

* ─ is created using U+2500.

* └ is created using U+2514.

* ┬ is created using U+252C.