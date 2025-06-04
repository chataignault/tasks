@echo off
setlocal enabledelayedexpansion

:: ===================================
:: SIMPLE CSV-STYLE ROBOCOPY SCRIPT
:: ===================================

echo Starting Robocopy Operations...
echo.

:: Configuration
set "CONFIG_FILE=config/backup_documents_config.txt"
set "LOGFILE=logs/backup_documents_log.txt"

:: Check if config file exists
if not exist "%CONFIG_FILE%" (
    echo ERROR: Configuration file "%CONFIG_FILE%" not found!
    echo Creating sample configuration file...
    call :CreateSampleConfig
    echo.
    echo Please edit "%CONFIG_FILE%" with your desired paths and run this script again.
    pause
    exit /b 1
)

:: Initialize log file
echo. > "%LOGFILE%"
echo ================================== >> "%LOGFILE%"
echo Robocopy Operations Started: %DATE% %TIME% >> "%LOGFILE%"
echo ================================== >> "%LOGFILE%"

:: Process configuration
echo Reading configuration...
call :ProcessConfig

echo.
echo ==========================================
echo All operations completed!
echo Check %LOGFILE% for detailed results
echo ==========================================

echo. >> "%LOGFILE%"
echo ================================== >> "%LOGFILE%"
echo Robocopy Operations Completed: %DATE% %TIME% >> "%LOGFILE%"
echo ================================== >> "%LOGFILE%"

pause
exit /b 0

:: ==========================================
:: PROCESS CONFIG FILE
:: ==========================================
:ProcessConfig
set "job_count=0"

for /f "usebackq tokens=1,2,3* delims=," %%A in ("%CONFIG_FILE%") do (
    set "source=%%A"
    set "dest=%%B"
    set "options=%%C"
    
    :: Skip empty lines and comments
    if not "!source!" == "" if not "!source:~0,1!" == "#" (
        set /a job_count+=1
        call :ExecuteJob
    )
)

exit /b 0

:: ==========================================
:: EXECUTE JOB
:: ==========================================
:ExecuteJob
:: Remove leading/trailing spaces
for /f "tokens=* delims= " %%X in ("!source!") do set "source=%%X"
for /f "tokens=* delims= " %%Y in ("!dest!") do set "dest=%%Y"
for /f "tokens=* delims= " %%Z in ("!options!") do set "options=%%Z"

echo.
echo ==========================================
echo [Job !job_count!]
echo Source:      !source!
echo Destination: !dest!
echo Options:     !options!
echo ==========================================

:: Expand environment variables
call set "exp_source=!source!"
call set "exp_dest=!dest!"

:: Validate source
if not exist "!exp_source!" (
    echo ERROR: Source does not exist: !exp_source!
    exit /b 0
)

:: Create destination
if not exist "!exp_dest!" (
    echo Creating destination: !exp_dest!
    mkdir "!exp_dest!" 2>nul
)

:: Set default options if empty
if "!options!" == "" set "options=/E /R:3 /W:10"

:: Add logging if not present
echo !options! | findstr /i "log" >nul
if !errorlevel! neq 0 set "options=!options! /LOG+:!LOGFILE!"

:: Execute robocopy
echo Running robocopy...
robocopy "!exp_source!" "!exp_dest!" !options!

set "exit_code=!errorlevel!"
if !exit_code! leq 1 (
    echo [Job !job_count!] SUCCESS
) else if !exit_code! leq 3 (
    echo [Job !job_count!] SUCCESS with info
) else (
    echo [Job !job_count!] WARNING: Exit code !exit_code!
)

exit /b 0

:: ==========================================
:: CREATE SAMPLE CONFIG
:: ==========================================
:CreateSampleConfig
(
echo # Robocopy Configuration File
echo # Format: source,destination,options
echo # One job per line, comma separated
echo # Lines starting with # are comments
echo #
echo # Available robocopy options:
echo #   /E = Copy subdirectories including empty ones
echo #   /MIR = Mirror directory tree
echo #   /R:n = Number of retries
echo #   /W:n = Wait time between retries  
echo #   /MT:n = Multi-threaded copies
echo #   /XD = Exclude directories
echo #   /XF = Exclude files
echo.
echo C:\Users\%%USERNAME%%\Documents,D:\Backup\Documents,/E /R:3 /W:10 /MT:8
echo C:\Users\%%USERNAME%%\Pictures,D:\Backup\Pictures,/E /R:3 /W:10 /MT:8
echo C:\Projects,\\NetworkDrive\Backup\Projects,/MIR /R:5 /W:15
) > "%CONFIG_FILE%"

exit /b 0
