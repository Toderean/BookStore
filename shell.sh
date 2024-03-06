            last_commit=$(git log -1 --pretty=format:"%h")
            echo $last_commit
            full_path=$(git diff-tree --no-commit-id --name-only -r ${last_commit})
            changes_path=$(dirname "$full_path")
            package_type=$(echo "$changes_path" | awk -F'/' '{print $1}')
            package_updated=false
            new_version="0.0.0"
            old_version="0.0.0"
            
            echo "$package_type"

            changed_packages=()

            # Loop through each modified folder
            while IFS= read -r changes_path; do
                package_type=$(echo "$changes_path" | awk -F'/' '{print $1}')
                
                case $package_type in
                    "crates" | "packages" )
                        if [ $package_type == "crates" ]; then
                            manifest_name="Cargo.toml" 
                        else
                            manifest_name="package.json"
                        fi
                        changed_packages+=$(echo $changes_path | awk -F'/' '{print $1"/"$2}')/$manifest_name
                        package_updated=true
                        ;;
                    *)
                        continue
                        ;;
                esac                
            done <<< "$full_path"
            echo "${changed_packages[@]}"

            # If no package type was found, exit with an error
            if [ "$package_updated" != true ]; then
                echo "No package type found"
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
            
            for changed_package in ${changed_packages[@]}; do

                package_type=$(echo "$changed_package" | awk -F'/' '{print $1}')

                #Read current version from manifest
                if [ ! -f "${changed_package}" ]; then
                    echo "Error: File '${changed_package}' not found."
                    ls -a
                    exit 1
                fi 
                
                case $package_type in
                    "crates")
                        new_version=$(grep -Eo 'version\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"' $changed_package | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')      
                        echo "newer version $package_type : $new_version"
                        ;;
                    "packages")
                        new_version=$(grep -Eo '"version"\s*:\s*"[0-9]+\.[0-9]+\.[0-9]+"' $changed_package | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')
                        echo "newer version $package_type : $new_version"
                        ;;
                    *)
                        echo "No available version"
                        exit 1
                        ;;
                esac
                
                
                if [ -z "$new_version" ]; then
                    echo "Error: Version not found in '$changed_package'."
                    exit 1
                fi
                # echo "newer version $package_type = $new_version"
                
                # cd $(echo $changed_package | awk -F'/' '{print $1"/"$2}')
                older_commit=$(git log --reverse --pretty=format:"%h" "$(echo $changed_package | awk -F'/' '{print $1"/"$2}')" | tail -n 2 | head -n 1)
                echo "$older_commit"
                
                case $package_type in
                    "crates")
                        old_version=$(git show $older_commit:$changed_package | grep -Eo 'version\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"' | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')
                        echo "older version $package_type : $old_version"
                        ;;
                    "packages")
                        old_version=$(git show $older_commit:$changed_package | grep -Eo '"version"\s*:\s*"[0-9]+\.[0-9]+\.[0-9]+"' | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')
                        echo "older version $package_type : $old_version"
                        ;;
                    *)
                        echo "No available version"
                        exit 1
                        ;;
                esac    
                
                if [ -z "$old_version" ]; then
                    echo "Error: Version not found in '$changed_package'."
                    exit 1
                fi
                echo "older version $package_type = $old_version"
                
                compare_versions "${new_version}" "${old_version}"
                result=$?
                
                #Check the result and print accordingly
                case $result in
                    0)
                        echo "New version is greater"
                        package_updated=true
                        ;;
                    1)
                        echo "Old version is greater or equal $older_version"
                        exit 1
                        ;;
                    *)
                        echo "Unrecognized exit code"
                        exit $result
                        ;;
                esac
            done