"""Admit and read bounded regular files for offline saved-report replay."""

from __future__ import annotations

import os
import stat
from pathlib import Path

from test_reference_consumer import RunnerError


def read_saved_file(path: Path, maximum: int, label: str) -> bytes:
    no_follow = getattr(os, "O_NOFOLLOW", 0)
    nonblocking = getattr(os, "O_NONBLOCK", 0)
    if not no_follow or not nonblocking:
        raise RunnerError(label)

    def opener(filename: str, flags: int) -> int:
        return os.open(filename, flags | no_follow | nonblocking)

    try:
        with open(path, "rb", opener=opener) as data:
            opened = os.fstat(data.fileno())
            if not stat.S_ISREG(opened.st_mode) or opened.st_size > maximum:
                raise RunnerError(label)
            raw = data.read(maximum + 1)
    except (OSError, ValueError):
        # ValueError includes paths containing an embedded NUL.
        raise RunnerError(label) from None
    if len(raw) > maximum:
        raise RunnerError(label)
    return raw
