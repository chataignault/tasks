#!/bin/bash

declare -a tex_roots
clear_opt="-c"

# parse arguments 
while getopts ":cp:" opt; do
	case $opt in
		c) clear_opt="-C";
			echo "Clean all auxiliary files";;
		p) tex_roots+=("$OPTARG");;
		\?) echo "Invalid option -$OPTARG" >&2
			exit 1
			;;
	esac
done

if [ ${#tex_roots[@]} -eq "0" ]; then 
	tex_roots="."
fi

# clear auxiliary files from latex subfolders
for p in "${tex_roots[@]}"; do
	echo "Cleaning root path : $p";
	find $p -name "*.tex" | \
		xargs -I {} -P 5 \
		sh -c "cd \$(dirname {}) && latexmk $clear_opt" \
		> /dev/null 2>&1
done

unset tex_roots
unset clear_opt
