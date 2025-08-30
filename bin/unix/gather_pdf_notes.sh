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
	fileName=$(basename $(dirname "$pdfFile"))
	if [[ "$fileName" != *"Coursework"* && "$fileName" != *"assignement"* ]]; then
		destinationFile="$3/${courseName}_${fileName}.pdf"
		if [ -e "$pdfFile" ]; then
			# tex file has been compiled 
			cp "$pdfFile" "$destinationFile"
		fi
	fi

}

export -f copy_pdf_file

shopt -s nocasematch
shopt -s nocaseglob

find "$sourcePath" -type f -name '*.tex' | \
	xargs -P 10 -I {} bash -c 'copy_pdf_file "$@"' _ {} $sourcePath $destPath

shopt -u nocasematch
shopt -u nocaseglob

# fuse pdf notes into one large file and courseworks into another
shopt -s extglob
rm $cimp/all_notes.pdf && pdfunite $cimp/all_notes/!(*Coursework*) $cimp/all_notes.pdf
if [ -e $cimp/courseworks.pdf ] 
then 
	rm $cimp/courseworks.pdf 
else 
	echo "Courseworks pdf not found"
fi 
pdfunite $cimp/all_notes/*Coursework* $cimp/courseworks.pdf
shopt -u extglob

