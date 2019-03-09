#!/usr/bin/env bash

set -e

if [[ $# -ne 2 ]]; then
	echo "Usage: $0 <project> <branch>"
	exit 1
fi

declare project=$1
declare branch=$2

cargo update
cargo doc --all --no-deps --release

# Generate an index page
declare crate_name=$(find -name "main.rs" -o -name "lib.rs" | head -n 1 | tr - _ | cut -d "/" -f2)

cat > "target/doc/index.html" << EOF
<html>
	<head>
		<title>Redirecting</title>
		<meta http-equiv="refresh" content="0; URL=${crate_name}/index.html"/>
	</head>
	<body></body>
</html>
EOF

mkdir -p "$RAVEN_DOCS_PATH/$project"
rm -rf "$RAVEN_DOCS_PATH/$project/$branch"
cp -r "target/doc/" "$RAVEN_DOCS_PATH/$project/$branch"

echo "Doc placed in \"$RAVEN_DOCS_PATH/$project/$branch\""
