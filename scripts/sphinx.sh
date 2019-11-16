#!/usr/bin/env bash

set -e

if [[ $# -ne 2 ]]; then
	echo "Usage: $0 <project> <branch>"
	exit 1
fi

declare project=$1
declare branch=$2

pip3 install --user -r requirements.txt

pushd "docs/"
	pip3 install --user -r requirements.txt
	sphinx-apidoc -f --separate -o source/ ..
	make html

	mkdir -p "$RAVEN_DOCS_PATH/$project"
	rm -rf "$RAVEN_DOCS_PATH/$project/$branch"
	cp -r ./build/html "$RAVEN_DOCS_PATH/$project/$branch"

	echo "Doc placed in \"$RAVEN_DOCS_PATH/$project/$branch\""
popd
