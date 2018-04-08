#!/usr/bin/env bash

set -e

if [[ $# -ne 2 ]]; then
	echo "Usage: $0 <project> <branch>"
	exit 1
fi

project=$1
branch=$2

cargo doc --all --no-deps --release
cat > "target/doc/index.html" << EOF
<html>
<head>
<title>Redirecting</title>
<meta http-equiv="refresh" content="0; URL=$(echo $project | tr - _)/index.html">
</head>
<body>
</body>
</html>
EOF

mkdir -p "$RAVEN_DOCS_PATH/$project"
rm -rf "$RAVEN_DOCS_PATH/$project/$branch"
cp -r "target/doc/" "$RAVEN_DOCS_PATH/$project/$branch"

echo "Doc is in" "$RAVEN_DOCS_PATH/$project/$branch"
