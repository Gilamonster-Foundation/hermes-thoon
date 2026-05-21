# SPDX-License-Identifier: Apache-2.0
"""hermes-thoon-fileops — Phase 2 placeholder.

Hermes plugin glue for `thoon-fileops`. Real implementation lands in
Phase 2 once `thoon-fileops` exposes the search/read/write/patch
primitives.
"""

from __future__ import annotations

import logging

logger = logging.getLogger(__name__)

__version__ = "0.5.20260520"


def register(ctx) -> None:
    logger.debug("hermes-thoon-fileops placeholder loaded (Phase 2)")
