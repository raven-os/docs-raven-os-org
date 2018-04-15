#!/usr/bin/env bash

set -e

if [[ $# -ne 2 ]]; then
	echo "Usage: $0 <project> <branch>"
	exit 1
fi

project=$1
branch=$2

doxygen Doxyfile

mkdir -p "$RAVEN_DOCS_PATH/$project"
rm -rf "$RAVEN_DOCS_PATH/$project/$branch"
cp -r ./html "$RAVEN_DOCS_PATH/$project/$branch"

echo "Doc is in" "$RAVEN_DOCS_PATH/$project/$branch"
