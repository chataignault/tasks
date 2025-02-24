#!bin/bash

# Copy pdf note rendering from Imperial folder to unified folder

sourcePath="$cimp"
destPath="$sourcePath/all_notes"

if [ ! -d "$sourcePath" ]; then
    echo "Source folder does not exist."
    exit 1
fi

if [ ! -d "$destPath" ]; then
	echo "Create folder to gather notes"
	mkdir "$destPath"
fi

copy_pdf_file() {
	courseName=$(echo $(realpath --relative-to "$2" "$1") | \
		awk -F/ '{print $1}')
	pdfFile=$(echo "$1" | awk -F. '{print $1}')".pdf"
	# echo $(dirname "$pdfFile")
	# echo 
	fileName=$(basename $(dirname "$pdfFile"))
	# echo "$fileName"
	# echo
	destinationFile="$3/${courseName}_${fileName}.pdf"
	echo "$pdfFile" "$destinationFile"	
	cp "$pdfFile" "$destinationFile"
	# echo "Copied $pdfFile to $destinationFile"

}

export -f copy_pdf_file

find "$sourcePath" -type f -name '*.tex' | \
	xargs -P 10 -I {} bash -c 'copy_pdf_file "$@"' _ {} $sourcePath $destPath
	# while IFS= read -r sourceFile; do 
	# copy_file $sourceFile	
#done

echo "All note files copied to $outPath"

