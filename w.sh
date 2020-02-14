#!/bin/bash

echo "This will upload all the sub-directories to Github!"
echo "r u sure?  [y/n]: "
read ans

if [ "$ans" = "y" ] || [ "$ans" = "Y" ]; then

    echo -e "Enter the absolute path of the root directory: \\n(NOTE the root directory is the directory that contains the projectS which'll be uploaded to github) "
    read ROOT_DIR

    if [ -d "$ROOT_DIR" ]; then

        REPO_NAME_PREFIX=''
        echo -e "\n\nEnter a 'prefix' that will be appended to the ALL new repos' name: \\n(The name of each new repo will be prefix_directory_name) "

        # Read the value of REPO_NAME_PREFIX and if it is SET then append the char _ to it ,, var=${FOO:+val} 	var= val if $FOO is set/notEmpty
        read REPO_NAME_PREFIX && REPO_NAME_PREFIX=${REPO_NAME_PREFIX:+"${REPO_NAME_PREFIX}_"}

        # GitHub API Token
        GH_API_TOKEN=$(cat /mnt/d/Documents/Desktop/prosjekter/token)

        # GitHub User Name
        GH_USER='ahaggi'

        while read -r i; do
            PROJECT_DIR=${i%%/}

            NEW_REPO_NAME="${REPO_NAME_PREFIX}$PROJECT_DIR"

            PROJECT_DIR_ABSOLUTE_PATH="$ROOT_DIR/$PROJECT_DIR"

            # GitHub repos Create API call
            curl -H "Authorization: token $GH_API_TOKEN" https://api.github.com/user/repos -d '{"name": "'"${NEW_REPO_NAME}"'"}'
            
            # Initialize Git in project directory, and add the GH repo remote.
            git --git-dir="$PROJECT_DIR_ABSOLUTE_PATH/.git" --work-tree=$PROJECT_DIR_ABSOLUTE_PATH init

            echo "git --git-dir="$PROJECT_DIR_ABSOLUTE_PATH/.git" --work-tree=$PROJECT_DIR_ABSOLUTE_PATH init"

            git --git-dir="$PROJECT_DIR_ABSOLUTE_PATH/.git" --work-tree=$PROJECT_DIR_ABSOLUTE_PATH add .
            git --git-dir="$PROJECT_DIR_ABSOLUTE_PATH/.git" --work-tree=$PROJECT_DIR_ABSOLUTE_PATH commit -m "first commit"
            git --git-dir="$PROJECT_DIR_ABSOLUTE_PATH/.git" --work-tree=$PROJECT_DIR_ABSOLUTE_PATH remote add origin git@github.com:$GH_USEr/$NEW_REPO_NAME.git

            git --git-dir="$PROJECT_DIR_ABSOLUTE_PATH/.git" --work-tree=$PROJECT_DIR_ABSOLUTE_PATH push -u origin master
        done < <(cd $ROOT_DIR && ls -1d */) #the 1st < will print the output to a "temp file" and the 2nd will take that output and feed it to the while loop
    else
        echo "The given root directory does not exist!"
    fi

fi
