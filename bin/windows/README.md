# Windows Automation Scripts

This directory contains batch scripts and configurations for automating workflows on Windows sessions.

## Scripts

### backup_documents.bat

A robust robocopy-based backup script that reads CSV-style configuration files to automate file synchronization tasks.

**Usage:**
```batch
backup_documents.bat <config_file>
```

**Features:**
- CSV-style configuration format (source, destination, options)
- Automatic logging to `C:\code\tasks\bin\logs\backup_documents_log.txt`
- Creates sample configuration if file doesn't exist
- Supports environment variable expansion (e.g., `%USERNAME%`)
- Validates source paths before execution
- Auto-creates destination directories
- Handles multiple backup jobs in sequence
- Detailed exit code reporting

**Configuration Format:**
```csv
# source,destination,options
C:\Users\%USERNAME%\Documents,B:\Documents,/E /R:3 /W:10 /MT:8 /PURGE
C:\Users\%USERNAME%\Pictures,B:\Pictures,/E /R:3 /W:10 /MT:8
```

**Common Robocopy Options:**
- `/E` - Copy subdirectories including empty ones
- `/MIR` - Mirror directory tree (copies and deletes)
- `/R:n` - Number of retries on failed copies
- `/W:n` - Wait time between retries (seconds)
- `/MT:n` - Multi-threaded copies (default 8)
- `/XD` - Exclude directories
- `/XF` - Exclude files
- `/PURGE` - Delete destination files/dirs that no longer exist in source

See [Microsoft Robocopy Documentation](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/robocopy) for more options.

### journal.bat

A password-protected daily journaling script that creates dated text files for note-taking.

**Usage:**
```batch
journal.bat
```

**Features:**
- Creates a hidden `diary` folder on first run
- Password protection (stored in `.private` file)
- Automatically creates/opens today's journal entry (format: YYYYMMDD.txt)
- Uses vim if Git Bash/WSL is available, otherwise falls back to notepad
- Date-stamped entries

**First Run:**
1. Creates the diary folder
2. Prompts for password setup
3. Hides the diary folder (`attrib +h`)

**Subsequent Runs:**
1. Prompts for password
2. Opens or creates today's journal entry

## Configuration Files

The `config/` directory contains pre-configured backup profiles:

### backup_shutdown.txt

Configuration for backing up user directories to drive B: before shutdown/logout.

**Backup Jobs:**
- Lectures/Courses → `B:\Cours` (excludes venv, .venv, data, cache)
- Work experiences → `B:\work_experiences`
- Desktop → `B:\desktop` (excludes .ini files)
- Administrative documents → `B:\actes`

**Usage:**
```batch
backup_documents.bat config\backup_shutdown.txt
```

### backup_startup.txt

Configuration for restoring files from drive B: to user directories on startup/login.

**Restore Jobs:**
- `B:\Cours` → Courses directory
- `B:\work_experiences` → Work directory
- `B:\desktop` → Desktop
- `B:\actes` → Administrative documents

**Usage:**
```batch
backup_documents.bat config\backup_startup.txt
```

## Workflow Examples

### Daily Backup Routine
```batch
REM Run before shutdown
backup_documents.bat config\backup_shutdown.txt
```

### System Restore
```batch
REM Run after fresh login or system restore
backup_documents.bat config\backup_startup.txt
```

### Quick Journal Entry
```batch
REM Open today's journal
journal.bat
```

## Requirements

- Windows OS with robocopy (included in Windows Vista and later)
- Drive B: or appropriate backup destination
- Optional: Git Bash or WSL for vim support in journal.bat

## Log Files

Backup operations are logged to:
```
C:\code\tasks\bin\logs\backup_documents_log.txt
```

Each run appends to the log with timestamps and detailed robocopy output.

## Notes

- The backup configurations use `/MIR` which mirrors directories (deletes files that don't exist in source)
- Always test configurations with a small dataset first
- Ensure drive B: (or your backup destination) is accessible before running backups
- Journal entries are stored in a hidden folder - make sure to remember your password
