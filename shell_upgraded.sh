last_commit=$(git log -1 --pretty=format:"%h")
full_path=$(git diff-tree --no-commit-id --name-only -r ${last_commit})
changes_path=$(dirname "$full_path" | awk -F'/' '{print $1"/"$2}')
package_types=("Cargo.toml" "package.json")

for packages in package_type