#!/usr/bin/env bash
# https://github.com/bevyengine/bevy/blob/acc8f6d45547f3e9cbc0b1fc5868a3e1b70cf26b/tools/publish.sh
if [ -n "$(git status --porcelain)" ]; then
    echo "You have local changes!"
    exit 1
fi

pushd crates

for crate in `cargo package --workspace 2>&1 | grep Packaging | sed 's_.*crates/\(.*\))_\1_' | grep -v Packaging`
do
  echo "Publishing ${crate}"
  pushd "$crate"
  cargo publish
  popd
  sleep 20 # wait for rate limit
done

popd

echo "Publishing root crate"
cargo publish
