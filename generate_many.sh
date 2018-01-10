#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -x

cd "$( dirname "${BASH_SOURCE[0]}")"

for i in `seq 1 $1`;
do
    ./generate.sh
    cp output.pdf output_${i}.pdf
done 