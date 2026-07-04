<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz> -->

# NPMGremlinBox

NPMGremlinBox is the Node.js/npm sub-project of GremlinBox, providing packages for supply chain security testing and policy compliance evaluation.

## Package Categories

### Licence Packages (65)

Published under the @gremlinbox/ scope (e.g. @gremlinbox/gpl-3-0). Each package implements a different SPDX-compatible licence and exports getLicenceIdentifier(), retrieveLicenceContent(), and getPackageMetadata().

### Malware Simulation Packages (6)

Published under @gremlinbox/malware-* (e.g. @gremlinbox/malware-network-indicators). Contain benign code with strategically placed malicious URL references. Include a postinstall script that contacts a package-specific sigre.xyz endpoint to demonstrate install-time execution for tabletop exercises.

Attack vector categories:
- network-indicators: Network-based indicators of compromise
- c2-beacon: Command and control beacon patterns
- code-obfuscation: Code obfuscation and hidden payload patterns
- install-execution: Install-time execution and dropper patterns
- credential-harvesting: Credential harvesting and exfiltration patterns
- cryptomining-indicators: Cryptomining and persistent connection patterns

### Typosquatting Packages (8)

Published under their typosquat names (unscoped, e.g. axois targeting axios). Include a postinstall script that contacts a package-specific sigre.xyz endpoint.

| Directory | Published Name | Target |
|-----------|---------------|--------|
| NPMGremlinBox-Typosquat-Axios | axois | axios |
| NPMGremlinBox-Typosquat-Lodash | lodahs | lodash |
| NPMGremlinBox-Typosquat-Express | expres | express |
| NPMGremlinBox-Typosquat-Chalk | chaalk | chalk |
| NPMGremlinBox-Typosquat-Commander | comander | commander |
| NPMGremlinBox-Typosquat-React | reeact | react |
| NPMGremlinBox-Typosquat-Webpack | webpak | webpack |
| NPMGremlinBox-Typosquat-Moment | momment | moment |

### Base Package

NPMGremlinBox-Base publishes @gremlinbox/base, which squats the @gremlinbox scope on npm.

## Version

All packages are at version 2.0.1.

## Licence

This README is licenced under AGPL-3.0-or-later. Individual packages carry their own SPDX licence identifiers.
