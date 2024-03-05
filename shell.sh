last_commit=$(git log -1 --pretty=format:"%h")
changes_path=$(git diff-tree --no-commit-id --name-only -r ${last_commit})
# changes_path=echo ${full_path} | cut -d "/" f1 
package_type=echo "$changes_path" | awk -F'/' '{print $1}'
package_updated=false
new_version="0.0.0"
old_version="0.0.0"

if [$package_type == "crates"]; then
  manifest_name="Cargo.toml"
  echo "$package_type"
elif [$package_type == "pacakges"]; then
  manifest_name="package.json"
  echo "$package_type"
else
  echo "Unknown package type"
  exit 1
fi

compare_versions() {
  # Split version numbers into their components
  IFS='.' read -ra new_version_array <<< "$new_version"
  IFS='.' read -ra old_version_array <<< "$old_version"

  # Compare major, minor, and patch versions
  for i in {0..2}; do
      if [ "${new_version_array[i]}" -lt "${old_version_array[i]}" ]; then
          echo "$older_version"
          return 1  # Old version is greater
      fi
  done
  return 0  # New version is greater or equal then old version
}

#Read current version from manifest
if [ ! -f "$changes_path/$manifest_name" ]; then
  echo "Error: File '${changes_path}/${manifest_name}' not found."
  echo "ls -all"
  exit 1
fi 
if [$package_type == "crates"]; then
version=$(echo 'version = "0.1.0"' | awk -F'"' '{print $2}')
elif [$package_type == "packages"]; then
version=$(echo 'version = "0.1.0"' | awk -F'"' '{print $4}')
else
echo "No available version"
fi

if [ -z "$version" ]; then
  echo "Error: Version not found in '$changes_path/$manifest_name'."
  exit 1
fi
echo "$version"

cd "$changes_path"
older_commit=$(git log --reverse --pretty=format:"%H" "$manifest_name" | tail -n 1)
echo "$older_commit"

if [$package_type == "crates"]; then
version=$(echo 'version = "0.1.0"' | awk -F'"' '{print $2}')
elif [$package_type == "packages"]; then
version=$(echo 'version = "0.1.0"' | awk -F'"' '{print $4}')
else
echo "No available version"
fi          

echo "$version"

compare_versions "${new_version}" "${old_version}"
result=$?

#Check the result and print accordingly
case $result in
  0)
      echo "New version is greater"
      package_updated=true
      ;;
  1)
      echo "Old version is greater $older_version"
      exit 1
      ;;
  *)
      echo "Unrecognized exit code"
      exit $result
      ;;
esac