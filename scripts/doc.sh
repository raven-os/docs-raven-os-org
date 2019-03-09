#!/usr/bin/env bash

set -e

if [[ $# -ne 3 ]]; then
	echo "Usage: $0 <owner> <project> <branch>"
	exit 1
fi

if [[ -z $RAVEN_DOCS_PATH ]]; then
	echo "The RAVEN_DOCS_PATH environment variable is either not set or empty."
	exit 1
fi

declare owner=$1
declare project=$2
declare branch=$3

declare tmp=$(mktemp -d)
declare old_path=$(pwd)

git clone -b "$branch" "https://github.com/$owner/$project.git" "$tmp"
pushd "$tmp"
	# Check the repository language and use the corresponding script
	if [[ -f "Cargo.toml" ]]; then
		echo "Rust (Rustdoc) project detected"
		"$old_path/scripts/rustdoc.sh" "$project" "$branch"
	elif [[ -f "Doxyfile" ]]; then
		echo "C/C++ (Doxygen) project detected"
		"$old_path/scripts/doxygen.sh" "$project" "$branch"
	elif [[ -f "docs/source/index.rst" ]]; then
		echo "Python (Sphinx) project detected"
		"$old_path/scripts/sphinx.sh" "$project" "$branch"
	else
		echo "Couldn't determine the project's language"
		exit 0
	fi

	echo "Finished building doc of \"$owner/$project:$branch\""
popd

# Remove temp files
rm -rf "$tmp"
