<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
<!-- SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz> -->

# PyGremlinBox

PyGremlinBox is the Python sub-project within the GremlinBox umbrella project. It provides 72 independently publishable Python packages for supply chain security testing and policy compliance evaluation, distributed via PyPI.

## Package Categories

### Licence Simulation Packages (65 packages)

Each package implements a specific software licence, enabling validation of dependency scanning tools, licence policy enforcement systems, SPDX compliance checks, and continuous integration pipelines. The 65 packages span four categories:

- Strong Copyleft (9 packages): AGPL variants, GPL-2.0, GPL-3.0, SSPL-1.0
- Weak Copyleft (23 packages): EPL, EUPL, LGPL, MPL, CDDL, CERN-OHL, CAL, OSL, and others
- Permissive and Other (16 packages): CC-BY-SA variants, APSL, Arphic, Artistic, FDK-AAC, and others
- Non-Permissive (17 packages): BUSL, CC-BY-NC variants, PolyForm, CDLA, Hippocratic, and others

### Malware Simulation Packages (6 packages)

Each package contains minimal benign code with strategically placed malicious URL indicators for use in tabletop exercises and threat detection validation. Attack vector categories covered:

- Network Indicators: HTTP/HTTPS endpoint patterns
- Command and Control Beacon: WebSocket channel patterns
- Code Obfuscation: Hidden path URL patterns
- Install-Time Execution: Installer/dropper URL patterns
- Credential Harvesting: SSH/SFTP URL patterns
- Cryptomining Indicators: FTP/FTPS pool connection patterns

### Base Package (1 package)

- gremlinbox (PyGremlinBox-Base): PyPI name squatter package directing users to the GremlinBox project repository.

## Package Interface

All licence simulation packages expose a consistent interface:

```python
import pygremlinbox_gpl_3_0 as pkg

pkg.get_licence_identifier()   # Returns SPDX identifier string
pkg.retrieve_licence_content() # Returns full licence text
pkg.get_package_metadata()     # Returns dict with name, version, licence, spdx_licence_id
```

All malware simulation packages expose:

```python
import pygremlinbox_malware_network_indicators as pkg

pkg.get_indicators()       # Returns list of test URLs
pkg.get_attack_category()  # Returns attack vector category string
pkg.get_package_metadata() # Returns dict with name, version, licence, attack_vector
```

## Package Structure

Each package follows the src-layout convention:

```
PyGremlinBox-[LICENCE]/
+-- src/
|   +-- pygremlinbox_[licence]/
|       +-- __init__.py
+-- pyproject.toml
+-- LICENCE
```

## SPDX Compliance

All packages maintain strict SPDX 2.3 compliance with valid SPDX identifiers, SPDX headers in all source files, machine-readable licence metadata, and individual LICENCE files containing the official licence text.

## Licence

Project infrastructure (this README) is licenced under AGPL-3.0-or-later. Individual packages retain their own SPDX licence identifiers. See the LICENCE file in each package directory for the applicable terms.
