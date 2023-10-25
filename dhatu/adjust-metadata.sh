#! bash

read -p "node url: " node_url

echo "generating runtime types from $node_url" 

subxt codegen --url $node_url > ./src/runtime_types.rs

echo "formatting..."

cargo clippy --fix --allow-dirty

echo "committing..."

git add src/runtime_types.rs
git commit -m"chore: adjust runtime types"

echo "done"