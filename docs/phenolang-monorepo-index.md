# PhenoLang monorepo extraction index

**Source:** [KooshaPari/PhenoLang](https://github.com/KooshaPari/PhenoLang) (~3978 paths, organizational shelf)  
**Migration branch:** `archive/phenolang-index` (2026-06-16)  
**Registry status:** `migrating` (index-only — no bulk path migration)

PhenoLang is a polyrepo shelf containing ~120 top-level project directories. This document maps
where shelf components were extracted or rationalized in the standalone KooshaPari fleet. Use this
index instead of migrating all 5234+ paths.

## Shelf layout

```
PhenoLang/
├── agileplus*          → AgilePlus org repos
├── phenotype-*         → phenotype-* standalone repos
├── helios-* / helMo    → helios-cli / HeliosLab / phenotype-tooling
├── hexagon-* / Hexa*   → HexaKit
├── *Kit / kits/        → phenotype-python-sdk / phenotype-go-sdk
├── nanovms             → nanovms
├── Traceon / Metron    → Traceon / Metron → phenotype-otel / HexaKit
├── Profila             → phenotype-python-sdk observability-kit/profila
├── templates/          → HexaKit / phenotype-tooling templates
├── docs/ governance/   → PhenoSpecs / phenotype-registry / PhenoHandbook
└── .archive/           → KEEP ARCHIVED husks
```

## Extraction map (authoritative targets)

| PhenoLang path | Standalone repo / target | Status |
|----------------|--------------------------|--------|
| `agileplus`, `agileplus-mcp`, `agileplus-agents`, `agileplus-plugin-*` | [AgilePlus](https://github.com/KooshaPari/AgilePlus) | extracted |
| `Apisync` | [Apisync](https://github.com/KooshaPari/Apisync) | extracted |
| `Authvault` | [Authvault](https://github.com/KooshaPari/Authvault) | extracted |
| `BytePort` | [phenotype-tooling](https://github.com/KooshaPari/phenotype-tooling) (BytePort lane) | migrating |
| `Cmdra` | [Cmdra](https://github.com/KooshaPari/Cmdra) | extracted |
| `Cursora` | [Cursora](https://github.com/KooshaPari/Cursora) | extracted |
| `Datamold` | [Datamold](https://github.com/KooshaPari/Datamold) | extracted |
| `Dino` | [Dino](https://github.com/KooshaPari/Dino) | extracted |
| `Duple` | [Duple](https://github.com/KooshaPari/Duple) | extracted |
| `Evalora` | [Evalora](https://github.com/KooshaPari/Evalora) | extracted |
| `Eventra` | [Eventra](https://github.com/KooshaPari/Eventra) | extracted |
| `Flagward` | [Flagward](https://github.com/KooshaPari/Flagward) | extracted |
| `Flowra` | [Flowra](https://github.com/KooshaPari/Flowra) | extracted |
| `Guardis` | [Guardis](https://github.com/KooshaPari/Guardis) | extracted |
| `Httpora` | [Httpora](https://github.com/KooshaPari/Httpora) | extracted |
| `Kogito` | [Kogito](https://github.com/KooshaPari/Kogito) | extracted |
| `Metron` | [Metron](https://github.com/KooshaPari/Metron) → HexaKit metrics crate | migrating |
| `nanovms` | [nanovms](https://github.com/KooshaPari/nanovms) | canonical |
| `Planify` | archive (upstream-maintained) | archived |
| `PolicyStack` | [phenotype-tooling](https://github.com/KooshaPari/phenotype-tooling) | migrating |
| `Portalis` | [Portalis](https://github.com/KooshaPari/Portalis) | extracted |
| `Profila` | [phenotype-python-sdk](https://github.com/KooshaPari/phenotype-python-sdk) `observability-kit/python/profila` | absorbed-pending-delete |
| `Queris` | [Queris](https://github.com/KooshaPari/Queris) | extracted |
| `Quillr` | [Quillr](https://github.com/KooshaPari/Quillr) | extracted |
| `Schemaforge` | [Schemaforge](https://github.com/KooshaPari/Schemaforge) | extracted |
| `Seedloom` | [Seedloom](https://github.com/KooshaPari/Seedloom) | extracted |
| `Settly` | [Settly](https://github.com/KooshaPari/Settly) → HexaKit | migrating |
| `Stashly` | [Stashly](https://github.com/KooshaPari/Stashly) → HexaKit | migrating |
| `Tasken` | [Tasken](https://github.com/KooshaPari/Tasken) | extracted |
| `Tokn` | [Tokn](https://github.com/KooshaPari/Tokn) | extracted |
| `Tossy` | [Tossy](https://github.com/KooshaPari/Tossy) | extracted |
| `Traceon` | [Traceon](https://github.com/KooshaPari/Traceon) → [phenotype-otel](https://github.com/KooshaPari/phenotype-otel) | migrating |
| `Tracera` / `tracely` | Tracera / Tracely standalone | extracted |
| `Zerokit` | [Zerokit](https://github.com/KooshaPari/Zerokit) | extracted |
| `pheno-cli` | [pheno-cli](https://github.com/KooshaPari/pheno-cli) | extracted |
| `phenoSDK` | [phenoSDK](https://github.com/KooshaPari/phenoSDK) | extracted |
| `phenodocs` | [phenodocs](https://github.com/KooshaPari/phenodocs) | extracted |
| `phenotype-auth-ts` | [phenotype-auth-ts](https://github.com/KooshaPari/phenotype-auth-ts) | extracted |
| `phenotype-config-ts` | [phenotype-config-ts](https://github.com/KooshaPari/phenotype-config-ts) | extracted |
| `phenotype-hub` | [phenotype-registry](https://github.com/KooshaPari/phenotype-registry) scaffold | absorbed |
| `phenotype-router-monitor` | [phenoRouterMonitor](https://github.com/KooshaPari/phenoRouterMonitor) → phenoAI | migrating |
| `phenotype-*` (20+ dirs) | Matching `KooshaPari/phenotype-*` repos | see fleet |
| `Hexacore`, `hexagon-*`, `HexaGo`, `HexaPy`, `HexaType` | [HexaKit](https://github.com/KooshaPari/HexaKit) | migrating |
| `helios-cli` | [helios-cli](https://github.com/KooshaPari/helios-cli) | canonical |
| `helMo`, `helix-logging` | Helios observability lane / HexaKit | migrating |
| `kits/` | phenotype-python-sdk + phenotype-go-sdk packages | migrating |
| `PlatformKit` (if present) | phenotype-go-sdk + nanovms | see platformkit-migration.md |
| `worktree-manager` | phenotype-tooling | migrating |
| `template-*`, `templates/`, `scaffolds/` | HexaKit + phenotype-tooling | partial |
| `docs/`, `governance/`, `plans/` | PhenoSpecs + phenotype-registry | index-linked |
| `projects/INDEX.md` | This file supersedes for fleet-wide lookup | — |
| `vendor/`, `.archive/`, `forgecode-fork/` | Not migrated — archive-only | archived |
| `sharecli`, `thegent-*` | thegent / archived sharecli husks | archived |

## Category directories (containers, not repos)

| Directory | Contents |
|-----------|----------|
| `apps/` | Application subprojects — each child may have own repo |
| `libs/` | Shared library stubs — check matching `pheno*` / `phenotype-*` repo |
| `platforms/` | Platform-as-product grouping |
| `tooling/` | CLI/tool stubs → phenotype-tooling |
| `infra/` | shelf-infra, org-github → phenotype-infra / org-github |
| `crates/` | Rust workspace fragments → HexaKit / phenoUtils |
| `python/`, `rust/`, `proto/` | Language roots — follow crate name to standalone repo |

## How to resolve an unknown path

1. Check [phenotype-registry/ECOSYSTEM_MAP.md](https://github.com/KooshaPari/phenotype-registry/blob/main/ECOSYSTEM_MAP.md)
2. Check [RATIONALIZATION_PLAN.md](https://github.com/KooshaPari/phenotype-registry/blob/main/RATIONALIZATION_PLAN.md)
3. Search `gh search repos --owner KooshaPari <name>`
4. If no standalone repo and marked KEEP ARCHIVED in registry → do not migrate code; index only

## Delete eligibility

**PhenoLang delete-eligible: No.** Shelf still holds unique `.archive/` content, vendor pins,
and project metadata not fully reproduced elsewhere. This index satisfies the documentation
migration requirement without bulk copy.
