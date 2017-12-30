#!/bin/bash

#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

IFS=$'\n' readarray -t FORMATS <<EOLIST
col
csv
dst
edr
exp
hus
inf
jef
ksm
pec
pes
rgb
sew
svg
tap
thr
EOLIST
readonly TXT_FORMAT=txt
readonly BASE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
readonly CONVERT_DIR="${BASE_DIR}/libembroidery-convert"
readonly CONVERT_EXE="${BASE_DIR}/libembroidery-convert/libembroidery-convert"

echo "${FORMATS[@]}"

main() {
    set -ex
    {
        pushd "${BASE_DIR}/libembroidery-convert"
        qmake
        make -j4
        popd
    }
    readonly ORIGINAL="$(realpath ${1})"
    readonly ORIG_FMT="${2:-${ORIGINAL##*.}}"
    readonly FNAME="$(basename "${ORIGINAL}" ".${ORIGINAL##*.}")"
    readonly ORIG_DIR="$(dirname "${ORIGINAL}")"
    readonly CONV_DIR="${BASE_DIR}/${FNAME}-conversion-test"
    readonly RAW_DIR="${CONV_DIR}/raws"
    mkdir -p "${RAW_DIR}"
    "${CONVERT_EXE}" "${ORIGINAL}" "${RAW_DIR}/${FNAME}.${ORIG_FMT}"
    "${CONVERT_EXE}" "${ORIGINAL}" "${CONV_DIR}/${FNAME}-original.txt"
    "${CONVERT_EXE}" "${ORIGINAL}" "${CONV_DIR}/${FNAME}-original.csv"

    readonly START_FILE="${RAW_DIR}/${FNAME}.${ORIG_FMT}"
    "${CONVERT_EXE}" "${START_FILE}" "${CONV_DIR}/${FNAME}-${ORIG_FMT}.txt"
    "${CONVERT_EXE}" "${START_FILE}" "${CONV_DIR}/${FNAME}-${ORIG_FMT}.csv"

    for FMT in "${FORMATS[@]}"; do
        if [ "${FMT}" = "${ORIG_FMT}" ]; then
            continue
        elif "${CONVERT_EXE}" "${START_FILE}" "${RAW_DIR}/${FNAME}.${FMT}"; then
            "${CONVERT_EXE}" "${RAW_DIR}/${FNAME}.${FMT}" "${CONV_DIR}/${FNAME}-${FMT}.txt"
            "${CONVERT_EXE}" "${RAW_DIR}/${FNAME}.${FMT}" "${CONV_DIR}/${FNAME}-${FMT}.csv"
        else
            echo "Unable to convert ${FMT}"
        fi
    done

    for FMT in "${FORMATS[@]}"; do
        if [ -f "${CONV_DIR}/${FNAME}-${FMT}.txt" ]; then
            diff "${CONV_DIR}/${FNAME}-original.txt" "${CONV_DIR}/${FNAME}-${FMT}.txt" > "${CONV_DIR}/${FNAME}-${FMT}.diff" || true
        fi
    done
}

main "$@"
