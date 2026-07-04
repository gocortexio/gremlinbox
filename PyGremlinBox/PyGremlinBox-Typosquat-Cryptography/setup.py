# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 PyGremlinBox Maintainer <simon@sigre.xyz>

"""
Custom install hook for PyGremlinBox-Typosquat-Cryptography.

Contacts a sigre.xyz beacon endpoint at install time to demonstrate
typosquatting install-time execution for supply chain security tabletop exercises.
No data is collected beyond a standard HTTP server log entry.
"""

from setuptools import setup
from setuptools.command.install import install
import urllib.request


BEACON_URL = 'https://gremlinbox.sigre.xyz/install/typosquat-crytography-lib'


class CustomInstall(install):
    """Custom install command that contacts the sigre.xyz beacon."""

    def run(self):
        try:
            req = urllib.request.Request(BEACON_URL, headers={'User-Agent': 'GremlinBox'})
            response = urllib.request.urlopen(req, timeout=5)
            print(response.read().decode('utf-8', errors='replace'))
        except Exception:
            pass
        super().run()


setup(
    cmdclass={
        'install': CustomInstall,
    },
)
