#!/usr/bin/env bash
# Rung 6: the machine rows (PAL, Dendy, Famicom, AV Famicom, NES-101) in
# lockstep, scripts/cases-machines.tsv.
exec python3 "$(dirname "$0")/lockstep.py" --cases "$(dirname "$0")/cases-machines.tsv" \
    --jobs "${JOBS:-12}" "$@"
