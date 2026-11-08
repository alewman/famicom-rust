#!/usr/bin/env bash
# Rung 2: the lockstep gate on every NROM test ROM (600 slices) and
# AccuracyCoin's run-all (3,000), scripts/cases.tsv.
exec python3 "$(dirname "$0")/lockstep.py" --jobs "${JOBS:-12}" "$@"
