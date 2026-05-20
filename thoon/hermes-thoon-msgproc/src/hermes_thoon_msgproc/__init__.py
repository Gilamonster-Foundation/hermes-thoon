# SPDX-License-Identifier: Apache-2.0
"""hermes-thoon-msgproc — Phase 4 placeholder (deferred).

Hermes plugin glue for message-processing acceleration. Status: deferred
until Phases 1-3 ship and benchmarks confirm message-loop overhead is
the next real bottleneck. See PLAN.md.
"""

from __future__ import annotations

import logging

logger = logging.getLogger(__name__)

__version__ = "0.5.20260520"


def register(ctx) -> None:
    logger.debug("hermes-thoon-msgproc placeholder loaded (Phase 4, deferred)")
