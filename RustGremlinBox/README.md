<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz> -->

# RustGremlinBox

RustGremlinBox is the Rust sub-project of GremlinBox, providing crates for supply chain security testing and policy compliance evaluation.

## Package Categories

### Licence Crates (65)

Each crate implements a different SPDX-compatible licence and exports getLicenceIdentifier(), retrieveLicenceContent(), and getPackageMetadata(). Crate names follow the pattern gremlinbox-[slug] (e.g. gremlinbox-gpl-3-0).

### Malware Simulation Crates (6)

Crate names follow the pattern gremlinbox-malware-[category]. Each contains minimal benign code with strategically placed malicious URL indicators. Each crate includes a build.rs hook that contacts a package-specific gremlinbox.sigre.xyz endpoint at build time to demonstrate install-time execution for tabletop exercises.

Attack vector categories:

- network-indicators: Network-based indicators of compromise
- c2-beacon: Command and control beacon patterns
- code-obfuscation: Code obfuscation and hidden payload patterns
- install-execution: Install-time execution and dropper patterns
- credential-harvesting: Credential harvesting and exfiltration patterns
- cryptomining-indicators: Cryptomining and persistent connection patterns

### Typosquatting Crates (8)

Published under their typosquat names, targeting popular Rust crates. Each crate includes a build.rs hook that contacts a package-specific gremlinbox.sigre.xyz endpoint at build time.

| Directory | Published Name | Target |
|-----------|---------------|--------|
| RustGremlinBox-Typosquat-Serde | serd-serialize | serde |
| RustGremlinBox-Typosquat-Tokio | tokoi-runtime | tokio |
| RustGremlinBox-Typosquat-Clap | claap-cli | clap |
| RustGremlinBox-Typosquat-Rand | rnad-util | rand |
| RustGremlinBox-Typosquat-Reqwest | requwest | reqwest |
| RustGremlinBox-Typosquat-Anyhow | anyhow-errors | anyhow |
| RustGremlinBox-Typosquat-Syn | syn-parse | syn |
| RustGremlinBox-Typosquat-Regex | regx-engine | regex |

### Base Crate

RustGremlinBox-Base publishes the gremlinbox crate, which squats the gremlinbox name. Malware simulation and typosquatting crates each declare a single dependency on this crate to provide a benign transitive dependency edge for dependency-graph scanning tests.

## Version

All crates are at version 2.0.1.

## Licence

This README is licenced under AGPL-3.0-or-later. Individual crates carry their own SPDX licence identifiers. Malware simulation and typosquatting crates are licenced under AGPL-3.0-or-later.
