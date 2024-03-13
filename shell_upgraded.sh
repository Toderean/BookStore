    last_commit=$(git log -1 --pretty=format:"%h")
    full_path=$(git diff-tree --no-commit-id --name-only -r ${last_commit})
    changes_path=$(dirname "$full_path" | awk -F'/' '{print $1"/"$2}')
    package_types=("package.json" "Cargo.toml")
    package=""

    for dir_path in ${changes_path}; do
        echo $dir_path 
        temp=$(echo "$dir_path" | awk -F'/' '{print $1}')
        echo $temp
        case $package_type in
            "packages" )
                full_path=$changes_path
                ;;
            *)
                continue
                ;;
        esac                
    done
    
    cd $dir_path
    
    files=$(ls)
    
    for package_type in ${package_types[@]}; do
            for file in ${files[@]}; do
            if [ "${package_type}" == "${file}" ]; then
                package=${package_type}
                echo $package
            fi
            done    
    done
full_path_with_package="${full_path}/${package}"
echo "FULL_PATH=$full_path_with_package" >> $GITHUB_ENV