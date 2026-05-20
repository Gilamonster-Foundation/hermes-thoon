# SPDX-License-Identifier: Apache-2.0
"""hermes-thoon-sessiondb — Phase 3 placeholder.

Hermes plugin glue for `thoon-sqlite`. Real implementation lands in
Phase 3.
"""

from __future__ import annotations

import logging

logger = logging.getLogger(__name__)

__version__ = "0.5.20260520"


def register(ctx) -> None:
    logger.debug("hermes-thoon-sessiondb placeholder loaded (Phase 3)")
