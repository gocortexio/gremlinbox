# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 PyGremlinBox Maintainer <simon@sigre.xyz>

"""
PyGremlinBox Typosquat Simulation - PyGremlinBox-Typosquat-Numpy.

This package is published under the name 'numipy-core', a deliberate
misspelling of 'numpy', to demonstrate typosquatting attack vectors
for supply chain security testing and tabletop exercises.

This package is licenced under the GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later).
"""

__version__ = "2.0.1"
__licence__ = "AGPL-3.0-or-later"

INSTALL_BEACON = "https://gremlinbox.sigre.xyz/install/typosquat-numipy-core"

from pathlib import Path

_LICENCE_FILE = Path(__file__).parent.parent.parent / "LICENCE"
_LICENCE_CONTENT = _LICENCE_FILE.read_text() if _LICENCE_FILE.exists() else ""


def getLicenceIdentifier():
    """
    Return the SPDX licence identifier for this package.

    Returns:
        str: SPDX licence identifier
    """
    return "AGPL-3.0-or-later"


def retrieveLicenceContent():
    """
    Return the AGPL-3.0-or-later licence content.

    Returns:
        str: AGPL-3.0-or-later licence text
    """
    return _LICENCE_CONTENT


def getTyposquatTarget():
    """
    Return the name of the legitimate package this package typosquats.

    Returns:
        str: Target package name
    """
    return "numpy"


def getTyposquatVariant():
    """
    Return the specific spelling variant used in this typosquat.

    Returns:
        str: The typosquat PyPI name
    """
    return "numipy-core"


def getPackageMetadata():
    """
    Return metadata about this typosquat simulation package.

    Returns:
        dict: Package metadata
    """
    return {
        "name": "numipy-core",
        "version": __version__,
        "licence": __licence__,
        "target": "numpy",
        "typosquat_variant": "numipy-core",
        "install_beacon": INSTALL_BEACON,
        "install_hook": "setup.py:CustomInstall",
        "description": "Supply chain security testing - typosquat of numpy (numipy-core)",
    }


__all__ = [
    "getLicenceIdentifier",
    "retrieveLicenceContent",
    "getTyposquatTarget",
    "getTyposquatVariant",
    "getPackageMetadata",
    "INSTALL_BEACON",
]
