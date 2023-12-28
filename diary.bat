@echo off
setlocal enabledelayedexpansion

set "baseFolder=%~dp0"

if exist "%baseFolder%" (
	cd "%baseFolder%"
) else (
	echo The root folder is not well specified
	goto abortmessage
)

REM Set the folder path where you want to create the new file
set "folderPath=%baseFolder%diary"

if NOT exist "%folderPath%" (
	echo Create diary folder...
	mkdir "%folderPath%"
	set "password="
	set /p "password=Enter password :"
	echo !password!>"%folderPath%\.private"
	echo Hide directory...
	attrib +h "%folderPath%"
	goto endinitialsetup
) else (
	set /p password=<"%folderPath%\.private"
	:validatepassword
	set "attempt="
	set /p "attempt=Enter password : "
	if "!password!"=="!attempt!" (
		goto accessgranted
	) else (
		echo This is not the password
		goto validatepassword
	)
)

:accessgranted
REM Get the current date in the format YYYYMMDD
for /f "tokens=1-3 delims=/ " %%a in ('date /t') do set "dateStamp=%%c%%b%%a"

REM Set the name of the new file using the current date
set "fileName=%dateStamp%.txt"

REM Combine the folder path and file name to get the full path
set "fullPath=%folderPath%\%fileName%"

REM Write file header
printf "Record for  %dateStamp% :\n\n" > %fullPath%

REM Start vim with the specified folder and new file
vim "%fullPath%"

endlocal
goto end

:abortmessage
endlocal
echo The process has failed
goto end

:endinitialsetup
endlocal
echo The diary is ready to use
echo Run diary.bat again to write the first note
goto end

:end
pause 
exit

