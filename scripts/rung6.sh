#!/usr/bin/env bash
# Rung 6: the machine rows (PAL, Dendy, Famicom, AV Famicom, NES-101) in
# lockstep, scripts/cases-machines.tsv; then the peripherals, every device
# class plugged (--device) and driven (scripts/inputs/devices) on test ROMs and
# dumps that use it, scripts/cases-peripherals.tsv (scripts/make_device_cases.py).
set -uo pipefail
HERE=$(dirname "$0")
python3 "$HERE/lockstep.py" --cases "$HERE/cases-machines.tsv" --jobs "${JOBS:-12}" "$@"
machines=$?
python3 "$HERE/lockstep.py" --cases "$HERE/cases-peripherals.tsv" --jobs "${JOBS:-12}" "$@"
peripherals=$?
exit $((machines || peripherals))
