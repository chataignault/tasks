#!/bin/bash

# Copy claude agent specifications to config folder

if [[ -d ~/.claude/agents/ ]]; then
	cp -r ~/.claude/agents/* ~/code/config/ubuntu/.claude/agents/
else
	echo ".claude folder or agent folder does not exist"
fi

