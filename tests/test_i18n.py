#!/usr/bin/env python3
"""
Simple tests for locale files (can be expanded later).
These are intentionally written in Python as agreed in the design discussion.
"""

import json
from pathlib import Path

LOCALES_DIR = Path(__file__).parent.parent / "locales"


def test_locale_files_exist():
    assert (LOCALES_DIR / "en.json").exists()
    assert (LOCALES_DIR / "ja.json").exists()


def test_locale_files_are_valid_json():
    for lang in ["en", "ja"]:
        with open(LOCALES_DIR / f"{lang}.json", encoding="utf-8") as f:
            data = json.load(f)
            assert isinstance(data, dict)
            assert "restart_start" in data
            assert "restart_complete" in data


def test_keys_are_consistent():
    with open(LOCALES_DIR / "en.json", encoding="utf-8") as f:
        en = set(json.load(f).keys())
    with open(LOCALES_DIR / "ja.json", encoding="utf-8") as f:
        ja = set(json.load(f).keys())
    assert en == ja, f"Key mismatch: en-only={en-ja}, ja-only={ja-en}"


if __name__ == "__main__":
    test_locale_files_exist()
    test_locale_files_are_valid_json()
    test_keys_are_consistent()
    print("All locale tests passed.")
