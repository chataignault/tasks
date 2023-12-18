@echo off
setlocal

REM Set the folder path where you want to create the new file
set "folderPath=C:\code\tasks\diary"

REM Get the current date in the format YYYYMMDD
for /f "tokens=1-3 delims=/ " %%a in ('date /t') do set "dateStamp=%%c%%b%%a"

REM Set the name of the new file using the current date
set "fileName=%dateStamp%.txt"

REM Combine the folder path and file name to get the full path
set "fullPath=%folderPath%\%fileName%"

REM Start vim with the specified folder and new file
vim "%fullPath%"

endlocal
