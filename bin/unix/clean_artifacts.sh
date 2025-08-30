#!/bin/bash

remove_venv() {
	echo "Removing Python virtual environment..."
	find $1 -name .venv | xargs -I {} bash -c 'echo "Found {}" && rm -rf {}'
	find $1 -name venv | xargs -I {} bash -c 'echo "Found {}" && rm -rf {}'
	echo
}

remove_crates_artifacts() {
	echo "Removing cargo build artifacts..."
	currentDir=$(realpath $1)
	find $1 -name Cargo.toml | \
		xargs -I {} bash -c \
		'echo "Found $(dirname {})" && cd $(dirname {}) && cargo clean && cd $currentDir'
	echo 
}

if [ $# -eq 0 ]; then
	echo "Need specify root path..."
	exit 1	
fi


remove_venv $1
remove_crates_artifacts $1

echo "Finished removing artifacts."

