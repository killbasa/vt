#!/bin/sh

set -eou pipefail

base_url="https://github.com/killbasa/vt/releases/latest/download/"
target_path="./vt"

os="$(uname -s)"
if [ "$os" = "Linux" ]; then
	url="${base_url}vt-x86_64-unknown-linux-gnu.tar.gz"
	curl -L -sSf "$url" | tar -xz
else
	echo "Your platform ($os) is not supported."
	exit 1
fi

chmod +x "$target_path"

echo ""
echo "✅ Binary installed to $PWD/vt"
echo ""
echo "    Move to PATH: sudo mv $target_path /usr/local/bin/vt"
echo "        sudo mv $target_path /usr/local/bin/vt"
echo "    Report issues:"
echo "        https://github.com/killbasa/vt/issues"
echo ""
