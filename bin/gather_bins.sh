#!/bin/bash

RED='\033[0;31m'
BLUE='\033[0;33m'
ORANGE='\033[1;35m'
NC='\033[0m' 

WD=$(pwd)
# get the bin folder 
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# append bin folder to path
# check whether it is already there or not 
# export PATH=$PATH:$SCRIPT_DIR

# list the bash scripts already present in the bin floder
ls $SCRIPT_DIR/*.sh | xargs -I {} printf "\nShell script found %1s : ${RED} {} ${NC}"

# list the projects in the parent folder
ls $SCRIPT_DIR/../ | grep -E "*/$" | grep -v "^\." | xargs -I {} printf "\nFound project %6s : ${BLUE} {} ${NC}"

printf "\n"

# if there is a cargo file
for project_dir in $(ls $SCRIPT_DIR/../ | grep -E "*/$" | grep -v "^\." );
do
    cd "../$project_dir";
    if [ -f "Cargo.toml" ]
    then 
    cargo check;
    cargo test;
    # cargo build release 
    cargo build --release ;
    # move target .exe to bin folder
    cp "./target/release/${project_dir::-1}.exe" ../bin/
    echo -e "Successfully added ${ORANGE} $project_dir ${NC} bin."
    fi
done

# if it is shell or bat 
# copy file

cd $WD