# PhenoLang monorepo extraction index

`KooshaPari/PhenoLang` (private archived, ~5,200 paths) was the org monorepo. It is **not** fully superseded by [phenoUtils](https://github.com/KooshaPari/phenoUtils) alone.

## phenoUtils scope (utility crates only)

| Crate | Status |
|-------|--------|
| pheno-crypto | ✅ extracted |
| pheno-fs | ✅ extracted |
| pheno-net | ✅ extracted |
| pheno-shell | ✅ extracted |
| pheno-testing | ✅ extracted |

## Major extractions (see standalone repos)

| PhenoLang path | Successor repo |
|----------------|----------------|
| `crates/*` (21+ infrakit crates) | HexaKit |
| `AgilePlus`, `agileplus-*` | AgilePlus |
| `thegent-*`, `thegent` | thegent |
| `phenotype-*` packages | respective phenotype-* repos |
| `helios-cli`, `heliosApp` | HeliosCLI, phenotype-tooling |
| `nanovms`, `devenv-abstraction` | nanovms |
| `KodeVibeGo` | KodeVibe `engine/` |
| `Profila` | phenotype-tooling `packages/profila/` |
| `Traceon` | phenotype-otel (partial) |
| `PlatformKit` / `phenotype-go-kit` | phenotype-go-sdk |
| `worktree-manager` | PhenoVCS |

## Policy

- Do **not** delete PhenoLang until fleet-wide extraction map is validated.
- Do **not** unarchive — use successor repos.

Source inventory: `all_repos.txt` in archived PhenoLang tree.
