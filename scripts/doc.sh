#!/usr/bin/env bash

set -e

if [[ $# -ne 3 ]]; then
	echo "Usage: $0 <project> <branch>"
	exit 1
fi

if [[ -z $RAVEN_DOCS_PATH ]]; then
	echo "The RAVEN_DOCS_PATH environment variable is either not set or empty."
	exit 1
fi

owner=$1
project=$2
branch=$3

tmp=$(mktemp -d)

old_path=$(pwd)
git clone -b "$branch" "https://github.com/$owner/$project.git" "$tmp"
cd "$tmp"

# Check the repository language and use the corresponding script
if [[ -f "Cargo.toml" ]]; then
	echo "Rust project detected"
	"$old_path/scripts/rustdoc.sh" "$project" "$branch"
elif [[ -f "Doxyfile" ]]; then
	echo "C/C++ project detected"
	"$old_path/scripts/doxygen.sh" "$project" "$branch"
else
	echo "Couldn't determine the project's language"
	exit 0
fi

echo "Finished building doc of \"$owner/$project:$branch\""

# Remove temp files
cd "$old_path"
rm -rf "$tmp"
