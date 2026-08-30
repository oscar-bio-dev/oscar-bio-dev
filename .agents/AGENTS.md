# Oscar Mora - SetaeSense Agent Rules & Constraints

This document defines the strict behavioral rules, architectural guidelines, and coding standards that any AI agent must follow when interacting with the `oscar-bio-dev` workspace.

## 1. Industrial & Space-Grade Engineering Standards
- **Pure Rust Philosophy**: Adhere to a *bare-metal* and *zero-cost abstractions* mindset. Avoid bloated JS/frontend frameworks; prefer Askama (SSR) or Leptos/Dioxus (Wasm).
- **Strict Linting**: Respect and adhere to the pedantic `clippy` profile defined in the repository. Never bypass lints without explicit, documented architectural reasons.
- **Type Safety First**: Follow the "Parse, don't validate" paradigm. Use strongly typed newtypes for domain objects and leverage `serde` and `thiserror` for boundary validation and lossless error propagation.
- **No Unsafe Code**: Ensure 0 `unsafe` blocks. If `unsafe` is ever required, it must be explicitly authorized by the user.

## 2. Project Management & Roadmap Adherence
- **Continuous Tracking**: Always consult `docs/ROADMAP.md` to understand the current phase and architectural context.
- **Automated Check-offs**: When a significant architectural task or feature is completed and tested, **proactively update** `docs/ROADMAP.md` to mark it as done. Do not leave the roadmap stale.
- **Phase Discipline**: Do not prematurely introduce implementations belonging to future phases unless explicitly requested.

## 3. Version Control & Repository Hygiene
- **Immaculate Git History**: Always format commit messages following conventional commits (e.g., `feat:`, `fix:`, `docs:`, `refactor:`, `ci:`).
- **Proactive Commits**: When a set of files reaches a stable, tested state, proactively prompt the user with the exact `git add` and `git commit` commands to preserve the state.
- **Gitignore Vigilance**: Ensure that `.env` files, `.DS_Store`, build artifacts (`target/`), and local IDE configurations are strictly ignored. Never risk leaking secrets.

## 4. Aesthetic & UI/UX Standards
- **Terminal/Hacker Identity**: All frontend code must adhere to a sleek, industrial terminal aesthetic using the **Gruvbox Hard Dark** color palette.
- **Accessibility & Web Standards**: Semantic HTML is mandatory. Always implement `aria-hidden` tags for decorative elements, use semantic `<article>` or `<div>` over empty `<a>` tags, and maintain complete OpenGraph and SEO metadata.

## 5. Pre-Execution Audits (CI/CD)
- **Zero-Tolerance Testing**: Before declaring any Rust feature complete, you must ensure that `cargo fmt`, `cargo clippy`, and `cargo test` run flawlessly.
- **Proprietary IP Header Check**: Ensure every newly created `.rs` file contains the official SetaeSense copyright and confidentiality header.