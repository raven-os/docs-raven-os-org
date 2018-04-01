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

cd "$temp"

cargo doc --all --no-deps --release
cat > "$temp/target/doc/index.html" << EOF
<html>
<head>
<title>Redirecting</title>
<meta http-equiv="refresh" content="0; URL=$(echo $project | tr - _)/index.html">
</head>
<body>
</body>
</html>
EOF

mkdir -p "$RAVEN_DOCS_PATH"
rm -rf "$RAVEN_DOCS_PATH/$project"
cp -r "$temp/target/doc/" "$RAVEN_DOCS_PATH/$project"

echo "Finished building doc of \"$project\""
