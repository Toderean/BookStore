            last_commit="95c86a9f1785ba92228b4b18a453010d6efc4eeb"
            full_path=$(git diff-tree --no-commit-id --name-only -r ${last_commit})
            changes_path=$(dirname "$full_path" | awk -F'/' '{print $1"/"$2}')
            package_types=("package.json" "Cargo.toml")
            package=""
        
            for dir_path in ${changes_path}; do
                echo $dir_path 
                temp=$(echo "$dir_path" | awk -F'/' '{print $1}')
                echo $temp
                case $temp in
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