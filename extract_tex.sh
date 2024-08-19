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

find "$sourcePath" -type f -name '*.tex' -o -wholename '**/notes/*.png' | while IFS= read -r sourceFile; do
	relativePath=$(realpath --relative-to "$sourcePath" "$sourceFile") 
	destinationFile="$destinationPath/$relativePath"

	# Create folder if it does not already exist
	destinationFolder=$(dirname "$destinationFile")
	mkdir -p "$destinationFolder"

	cp "$sourceFile" "$destinationFile"
	echo ">>> Copied $sourceFile to $destinationFile"
done

echo "Copy operation completed."

