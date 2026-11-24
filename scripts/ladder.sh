#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# The whole ladder, every rung against the reference's cached traces (a missing
# trace is made by the reference, scripts/lockstep.py): what a kept change must
# pass. In order:
#   cargo test (the reference's unit tests transcribed: tests/vs_system.rs,
#     tests/playchoice.rs, ...);
#   rung 1, nestest against nestest.log;
#   the lockstep cases: rung 2 (and the probes), rung 5 (every board class, the
#     FDS, Lagrange Point's VRC7, the Sunsoft 5B, mapper 559, AutoDisk), rung 6
#     (the machine rows, the peripherals, the host input, the Vs. System's images
#     from VS_IMAGES_DIR, the PlayChoice-10's from PC10_IMAGES_DIR);
#   rung 3, the reference's test ROM tables, outcome for outcome;
#   rung 4, AccuracyCoin on the reference's counts;
#   save states: a dump of every board class (external/state-dumps.txt), the 5B
#     and 559, every Vs. UniSystem image, every PlayChoice-10 image in the cabinet.
# JOBS (default 8) runs that many cases at a time, under nice. The dumps come from
# NO_INTRO_DIR's sets (NES_DUMPS), the FDS BIOS from FAMICOM_FDS_BIOS or
# NO_INTRO_DIR, the PlayChoice-10 BIOS from MAME_ROMPATH (or FAMICOM_PC10_BIOS).
set -uo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
EXT=$ROOT/external
HERE=$ROOT/scripts
JOBS=${JOBS:-8}
REF=${FAMICOM_REFERENCE:-$EXT/famicom-python}
export NES_TEST_ROMS=${NES_TEST_ROMS:-$REF/tests/test_roms}
if [ -z "${FAMICOM_FDS_BIOS:-}" ] && [ -n "${NO_INTRO_DIR:-}" ]; then
    export FAMICOM_FDS_BIOS="$NO_INTRO_DIR/Nintendo - Family Computer Disk System (FDS)/[BIOS] Family Computer Disk System (Japan) (En) (Rev 1).zip"
fi
NES_DUMPS=${NES_DUMPS:-${NO_INTRO_DIR:-}/Nintendo - Nintendo Entertainment System (Headered)}
export VS_IMAGES_DIR=${VS_IMAGES_DIR:-$EXT/vs-images}
export PC10_IMAGES_DIR=${PC10_IMAGES_DIR:-$EXT/pc10-images}
BIN=$ROOT/target/release
began=$SECONDS
failed=0
step() { echo "== $1 ($((SECONDS - began)) s)"; }
lockstep() {
    local cases=$1 cache=$2
    shift 2
    nice python3 "$HERE/lockstep.py" --cases "$cases" --jobs "$JOBS" --reference-cache "$cache" \
        --rust "$BIN/famicom-trace" "$@" >"$EXT/ladder-lockstep.log" 2>&1 || failed=1
    grep -v ": identical" "$EXT/ladder-lockstep.log"
}

step "cargo test"
nice cargo build -j "$JOBS" --release -q --manifest-path "$ROOT/Cargo.toml" || exit 1
if ! nice cargo test -j "$JOBS" --release -q --manifest-path "$ROOT/Cargo.toml" >"$EXT/ladder-test.log" 2>&1; then
    grep -E "test result|FAILED|panicked" "$EXT/ladder-test.log" || tail -20 "$EXT/ladder-test.log"
    failed=1
fi
grep -E "^test result" "$EXT/ladder-test.log" | grep -v " 0 passed" || true

step "rung 1: nestest"
nice "$BIN/nestest" "$NES_TEST_ROMS/nes-test-roms" | tail -1 || failed=1

step "rung 2"
lockstep "$HERE/cases.tsv" "$EXT/refcache"
PROBES="$EXT/probes" "$ROOT/probes/build.sh" >/dev/null
lockstep "$HERE/cases-probes.tsv" "$EXT/refcache-probes"
step "rung 5: every board class (the 600-slice table, rung5.sh)"
lockstep "$EXT/cases-rung5.tsv" "$EXT/refcache-mappers"
step "rung 5: the FDS, Lagrange Point, AutoDisk"
lockstep "$EXT/cases-fds.tsv" "$EXT/refcache-mappers"
lockstep "$EXT/cases-lagrange.tsv" "$EXT/refcache-mappers"
lockstep "$HERE/cases-fds-auto.tsv" "$EXT/refcache-mappers"
step "rung 5: the Sunsoft 5B, mapper 559"
lockstep "$HERE/cases-5b.tsv" "$EXT/refcache-5b"
lockstep "$HERE/cases-559.tsv" "$EXT/refcache-559"
step "rung 6: machine rows, peripherals, host input"
lockstep "$HERE/cases-machines.tsv" "$EXT/refcache"
lockstep "$HERE/cases-peripherals.tsv" "$EXT/refcache-devices"
lockstep "$HERE/cases-host.tsv" "$EXT/refcache-host"
step "rung 6: the Vs. System"
lockstep "$HERE/cases-vs.tsv" "$EXT/refcache-vs"
step "rung 6: the PlayChoice-10"
lockstep "$HERE/cases-pc10.tsv" "$EXT/refcache-pc10"

step "rung 3: test ROM tables"
nice "$BIN/famicom-testrom" "$EXT/testrom-reference.jsonl" --roms "$NES_TEST_ROMS/nes-test-roms" \
    --out "$EXT/testrom-rust.jsonl" --jobs "$JOBS" >/dev/null
python3 "$HERE/testrom_compare.py" "$EXT/testrom-reference.jsonl" "$EXT/testrom-rust.jsonl" | tail -1 || failed=1

step "rung 4: AccuracyCoin"
coin=$(nice "$BIN/famicom-accuracycoin" "$NES_TEST_ROMS/AccuracyCoin/AccuracyCoin.nes" \
    "$EXT/accuracycoin-tests.json" --failures | tail -1) || failed=1
echo "$coin"
[ "$coin" = "147 of 147 pass; 126984584 CPU cycles, 4264 frames" ] || { echo "rung 4: not the reference's counts"; failed=1; }

step "save states: a dump of every board class"
# The dump list in JOBS shares, one famicom-state-roundtrip each.
split_dir=$EXT/ladder-states
mkdir -p "$split_dir"
pids=()
for i in $(seq 0 $((JOBS - 1))); do
    awk -v n="$JOBS" -v i="$i" 'NR % n == i' "$EXT/state-dumps.txt" >"$split_dir/part$i.txt"
    mapfile -t roms <"$split_dir/part$i.txt"
    nice "$BIN/famicom-state-roundtrip" "${roms[@]}" >"$split_dir/part$i.out" 2>&1 &
    pids+=($!)
done
for pid in "${pids[@]}"; do wait "$pid" || true; done
outs=()
for i in $(seq 0 $((JOBS - 1))); do outs+=("$split_dir/part$i.out"); done
total=$(grep -c . "$EXT/state-dumps.txt")
cat "${outs[@]}" | grep -v ": identical"
same=$(cat "${outs[@]}" | grep -c ": identical")
echo "$same of $total dumps round-trip"
[ "$same" = "$total" ] || failed=1
A=$REF/reference/src/nes-audio-tests/build
extra=$(nice "$BIN/famicom-state-roundtrip" --machine famicom "$NES_DUMPS/Gimmick! (Japan) (En).zip" \
    "$A/db_5b.nes" "$A/envelope_5b.nes" "$A/noise_5b.nes" 2>&1; nice "$BIN/famicom-state-roundtrip" \
    --machine dendy "$NES_DUMPS/Sangokushi II - Haou no Tairiku (China) (Ja) (Subor) (Pirate).zip" 2>&1)
echo "$extra" | grep -v ": identical"
same=$(echo "$extra" | grep -c ": identical")
echo "$same of 5 round-trip (the 5B, 559)"
[ "$same" = 5 ] || failed=1
step "save states: every Vs. UniSystem image"
mapfile -t vs < <(python3 -c 'import json, os, sys; d = sys.argv[1]
for e in json.load(open(os.path.join(d, "manifest.json"))):
    if e["vs_hardware"] not in (5, 6): print(os.path.join(d, e["file"]))' "$VS_IMAGES_DIR")
out=$(nice "$BIN/famicom-state-roundtrip" --machine auto --at 300 --to 420 \
    --input "$HERE/inputs/vs-state.txt" "${vs[@]}" 2>&1)
echo "$out" | grep -v ": identical"
same=$(echo "$out" | grep -c ": identical")
echo "$same of ${#vs[@]} Vs. images round-trip"
[ "$same" = "${#vs[@]}" ] || failed=1
step "save states: every PlayChoice-10 image, saved mid-game"
mapfile -t pc10 < <(python3 -c 'import json, os, sys; d = sys.argv[1]
for e in json.load(open(os.path.join(d, "manifest.json"))): print(os.path.join(d, e["file"]))' "$PC10_IMAGES_DIR")
out=$(nice "$BIN/famicom-pc10-states" --at 900 --to 1000 --input "$HERE/inputs/pc10/play.txt" "${pc10[@]}" 2>&1)
echo "$out" | grep -v ": identical"
same=$(echo "$out" | grep -c ": identical")
echo "$same of ${#pc10[@]} PlayChoice-10 images round-trip"
[ "$same" = "${#pc10[@]}" ] || failed=1

step "done"
if [ "$failed" = 0 ]; then echo "LADDER GREEN"; else echo "LADDER RED"; fi
exit "$failed"
