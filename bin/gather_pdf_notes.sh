#!bin/bash

# Copy pdf note rendering from Imperial folder to unified folder

sourcePath="$cimp"
outPath="$sourcePath/all_notes"

if [ ! -d "$sourcePath" ]; then
    echo "Source folder does not exist."
    exit 1
fi

if [ ! -d "$outPath" ]; then
	echo "Create folder to gather notes"
	mkdir "$outPath"
fi

find "$sourcePath" -type f -name '*.tex' | while IFS= read -r sourceFile; do 
	courseName=$(echo $(realpath --relative-to "$sourcePath" "$sourceFile") | awk -F/ '{print $1}')
	pdfFile=$(echo "$sourceFile" | awk -F. '{print $1}')".pdf"
	echo $(dirname "$pdfFile")
	echo 
	fileName=$(basename $(dirname "$pdfFile"))
	echo "$fileName"
	echo
	destinationFile="$outPath/${courseName}_${fileName}.pdf"
	
	cp "$pdfFile" "$destinationFile"
	echo "Copied $pdfFile to $destinationFile"
done

echo "All note files copied to $outPath"

