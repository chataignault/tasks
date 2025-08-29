#!/bin/bash

# Log file for debugging
LOG_FILE="/home/leonc/logs/shutdown_sync.log"
if [ ! -d $(dirname "$LOG_FILE") ] ; then
	mkdir $(dirname "$LOG_FILE")
fi
echo "$(date): Shutdown sync starting" >> "$LOG_FILE"

# update local documents from backup partition to local 
export BACKUP_FOLDER="/mnt/backup"
export LECTURES_DEST="/home/leonc/cours"

if [[ ! -d $BACKUP_FOLDER ]] ; then
	echo "$(date): ERROR: Backup folder is not mounted at this location : $BACKUP_FOLDER" >> "$LOG_FILE"
	mount >> "$LOG_FILE" 2>&1
	exit 1
fi

if [[ ! -d "$LECTURES_DEST" ]] ; then
	echo "$(date): ERROR: Source folder does not exist: $LECTURES_DEST" >> "$LOG_FILE"
	exit 1
fi

echo "$(date): Starting rsync..." >> "$LOG_FILE"

# mirror synchronisation
rsync -r --exclude '__pycache__' --exclude '.venv' --exclude '*.parquet' "$LECTURES_DEST/" "$BACKUP_FOLDER/Cours/" >> "$LOG_FILE" 2>&1

if [ $? -eq 0 ]; then
    echo "$(date): Rsync completed successfully" >> "$LOG_FILE"
else
    echo "$(date): Rsync failed with exit code $?" >> "$LOG_FILE"
fi

echo "$(date): Shutdown sync finished" >> "$LOG_FILE"
