#!/bin/bash

# Copy lecture notes source files from Imperial folder immediately to repository

sourcePath="$cimp"
destinationPath="/c/code/latex/cimp"


if [ ! -d "$sourcePath" ]; then
    echo "Source folder does not exist."
    exit 1
fi

# Check if destination folder exists, create if not
if [ ! -d "$destinationPath" ]; then
    mkdir -p "$destinationPath"
fi


copy_file() {
	relativePath=$(realpath --relative-to "$2" "$1") 
	destinationFile="$3/$relativePath"

	# Create folder if it does not already exist
	destinationFolder=$(dirname "$destinationFile")
	mkdir -p "$destinationFolder"

	cp "$1" "$destinationFile"
	# echo ">>> Copied $1 to $destinationFile"

}

export -f copy_file

find "$sourcePath" -type f \( \
	-name '*.tex' \
	-o -path '*/notes/*.png' \
	-o -path '*/notes/*.jpg' \
	-o -path '*/img/*.png' \
	-o -path '*/img/*.jpg' \
	-o -path '*/Mastery*/*.tex' \
	-o -path '*/Mastery*/*.png' \
	-o -path '*/Mastery*/*.jpg' \
	\) | xargs -P 10 -I {} bash -c 'copy_file "$@"' _ {} $sourcePath $destinationPath


