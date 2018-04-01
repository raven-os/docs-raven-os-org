#!/usr/bin/env bash

set -e

if [[ $# -ne 1 ]]; then
	echo "Usage: $0 <project>"
	exit 1
fi

project=$1

temp=$(mktemp -d)

# Clone repo in tmp directory
git clone "git@github.com:raven-os/$project.git" "$temp"

cd "$temp/doc"

doxygen Doxyfile

mkdir -p "$RAVEN_DOCS_PATH"
rm -rf "$RAVEN_DOCS_PATH/$project"
cp -r "$temp/doc/html" "$RAVEN_DOCS_PATH/$project"
