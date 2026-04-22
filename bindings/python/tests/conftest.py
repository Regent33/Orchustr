from __future__ import annotations

import os
import sys


TESTS_DIR = os.path.dirname(__file__)
PACKAGE_ROOT = os.path.abspath(os.path.join(TESTS_DIR, ".."))

if PACKAGE_ROOT not in sys.path:
    sys.path.insert(0, PACKAGE_ROOT)
