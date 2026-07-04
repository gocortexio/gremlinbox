# SPDX-License-Identifier: GPL-3.0
# SPDX-FileCopyrightText: 2025 GremlinBox Maintainer <simon@sigre.xyz>

"""
GremlinBox root package.

This package squats the "gremlinbox" name on PyPI to prevent dependency
confusion attacks and to direct users to the correct repository.
"""

__version__ = "2.0.1"
__licence__ = "GPL-3.0"

PROJECT_NAME = "GremlinBox"
PROJECT_DESCRIPTION = (
    "GremlinBox: multi-language supply chain security testing package collection. "
    "See https://github.com/gocortexio/gremlinbox for the full project."
)
REPOSITORY = "https://github.com/gocortexio/gremlinbox"
PYPI_PREFIX = "pygremlinbox"


def get_package_metadata():
    """Return identifying metadata for the GremlinBox root package."""
    return {
        "package_name": "gremlinbox",
        "version": __version__,
        "licence": __licence__,
        "description": PROJECT_DESCRIPTION,
        "repository": REPOSITORY,
    }
