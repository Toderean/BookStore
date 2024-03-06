            # last_commit=$(git log -1 --pretty=format:"%h")
            last_commit=$"0dc3ebf"
            full_path=$(git diff-tree --no-commit-id --name-only -r ${last_commit})
            changes_path=$(dirname "$full_path")
            package_type=$(echo "$changes_path" | awk -F'/' '{print $1}')
            package_updated=false
            new_version="0.0.0"
            old_version="0.0.0"
            
            echo "$package_type"
            # case $package_type in
            # "crates" )
            #     manifest_name="Cargo.toml"      
            #     echo "$package_type"
            #     ;;
            # "packages")
            #     manifest_name="package.json"
            #     echo "$package_type"
            #     ;; 
            # ".")
            #     continue
            #     ;;
            # ".github")
            #     continue
            #     ;;
            # *)
            #     echo "Unknown package type"
            #     exit 1
            #     ;;
            # esac
            

            # Loop through each modified folder
            while IFS= read -r changes_path; do
                package_type=$(echo "$changes_path" | awk -F'/' '{print $1}')
                
                # Skip ".github" and empty directories
                if [[ "$package_type" == "." || "$package_type" == ".github" ]]; then
                    continue
                fi
                
                echo "Package type: $package_type"
                
                case $package_type in
                    "crates" )
                        manifest_name="Cargo.toml"      
                        ;;
                    "packages")
                        manifest_name="package.json"
                        ;; 
                    *)
                        echo "Unknown package type"
                        exit 1
                        ;;
                esac
                
                # You can perform further actions based on the package type here
                
                # Set package_updated to true if necessary
                package_updated=true
                
            done <<< "$full_path"

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
            
            #Read current version from manifest
            if [ ! -f "$changes_path/$manifest_name" ]; then
                echo "Error: File '${changes_path}/${manifest_name}' not found."
                echo "ls -all"
                exit 1
            fi 
            
            case $package_type in
                "crates" )
                new_version=$(grep -Eo 'version\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"' $changes_path/$manifest_name| grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')      
                echo "newer version $package_type : $new_version"
                ;;
            "packages")
                new_version=$(grep -Eo '"version"\s*:\s*"[0-9]+\.[0-9]+\.[0-9]+"' $changes_path/$manifest_name | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+')
                echo "newer version $package_type : $new_version"
                ;;
            *)
                echo "No available version"
                exit 1
                ;;
            esac
            
            
            if [ -z "$version" ]; then
                echo "Error: Version not found in '$changes_path/$manifest_name'."
                exit 1
            fi
            echo "$version"
            
            cd "$changes_path"
            older_commit=$(git log --reverse --pretty=format:"%H" "$manifest_name" | tail -n 1)
            echo "$older_commit"
            
            case $package_type in
                "crates" )
                    old_version=$(echo 'version = "0.1.0"' | awk -F'"' '{print $2}')      
                    echo "older version $package_type : $old_version"
                    ;;
                "packages")
                    old_version=$(echo 'version = "0.1.0"' | awk -F'"' '{print $4}')
                    echo "older version $package_type : $old_version"
                    ;;
                *)
                    echo "No available version"
                    exit 1
                    ;;
            esac    
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
                    echo "Old version is greater $older_version"
                    exit 1
                    ;;
                *)
                    echo "Unrecognized exit code"
                    exit $result
                    ;;
            esac   