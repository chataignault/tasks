#!/bin/bash

# update local documents from backup partition to local 

export BACKUP_FOLDER="/mnt/backup"
export LECTURES_DEST="/home/leonc/cours"

if [[ ! -d $BACKUP_FOLDER ]] ; then
	echo "Backup folder is not mounted at this location : $BACKUP_FOLDER"
	return
fi

if [[ ! -d "$LECTURES_DEST" ]] ; then
	echo "Source folder does not exist."
	return 
fi

# mirror synchronisation
rsync -a --exclude '__pycache__' --exclude '.venv' --exclude '*.parquet' "$LECTURES_DEST/" "$BACKUP_FOLDER/Cours/"

