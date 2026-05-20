# SPDX-License-Identifier: Apache-2.0
"""hermes-thoon-toolreg — Hermes plugin: Rust-accelerated ToolRegistry.

Adapter glue between Hermes' ToolRegistry contract (`tools/registry.py`)
and the framework-agnostic `thoon-toolreg` Rust primitive.

This package is intentionally small: it discovers the Rust extension,
implements `register(ctx)` for Hermes' plugin manager, and translates
between Hermes' API shape and thoon's. The performance work itself lives
in the upstream-neutral `thoon-toolreg` crate.

Phase 1 stub. Real adapter wiring lands once the upstream plugin
contract for accelerator slots is settled.
"""

from __future__ import annotations

import logging

logger = logging.getLogger(__name__)

__version__ = "0.5.20260520"


def _thoon_version() -> str | None:
    """Return the version of the underlying Rust primitive, if loadable."""
    try:
        import thoon_toolreg
        return thoon_toolreg.version()
    except ImportError as exc:
        logger.debug("thoon_toolreg unavailable: %s", exc)
        return None


def register(ctx) -> None:
    """Hermes plugin entry point.

    Called by `hermes_cli.plugins.PluginManager` when this plugin is
    discovered via the `hermes_agent.plugins` entry-point group.

    Phase 1 stub: logs that the plugin loaded and reports the Rust
    primitive's version. Phase 1 implementation will swap in the
    Rust-backed ToolRegistry implementation once the accelerator slot
    hook lands upstream.
    """
    rust_version = _thoon_version()
    if rust_version is None:
        logger.warning(
            "hermes-thoon-toolreg loaded but thoon_toolreg Rust extension "
            "is not importable; plugin is inert."
        )
        return
    logger.info(
        "hermes-thoon-toolreg %s loaded (thoon_toolreg %s); awaiting "
        "accelerator slot hook to take over ToolRegistry.",
        __version__, rust_version,
    )
