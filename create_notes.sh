#!/bin/bash

# Check if a path is provided as an argument
if [ $# -eq 0 ]; then
	echo "Usage: $0 <path>"
	exit 1
fi

# Get the path from the command line argument
path="$1"
# Create the "notes" folder
notes_folder="$path/notes"

if [ -d "$notes_folder" ]; then
	echo "notes folder already exists !"
	exit 0
else
	mkdir -p "$notes_folder"

	notes_file="$notes_folder/notes.tex"
	echo "\documentclass[12pt]{article}
\title{ - Lecture notes}
\usepackage[english]{babel}
\date{}
\usepackage[utf8]{inputenc}
\usepackage{amsfonts}
\usepackage{amsmath}
\usepackage{geometry} \geometry{hmargin=2cm,vmargin=2cm}
\usepackage{mathtools}
\usepackage{url}

\begin{document}

\maketitle


\begin{thebibliiography}{}

\end{thebibliography}

\end{document}" > "$notes_file"

	echo "notes initialised in : $path"
fi

