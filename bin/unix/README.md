# Unix Automation Scripts

Shell scripts for automating workflows on Ubuntu/Fedora systems.

## System Monitoring

- **monitor_usage.sh** - Continuously logs CPU, memory, and GPU usage to daily log files

## LaTeX/Academic Utilities

- **create_notes.sh** - Initialize a LaTeX notes directory with a templated `.tex` file
- **extract_tex.sh** - Extract LaTeX content from documents
- **gather_pdf_notes.sh** - Collect compiled PDFs from course folders and merge them into unified documents
- **clean_latex_notes.sh** - Clean up LaTeX temporary files

## Cleanup

- **clean_artifacts.sh** - Remove Python virtual environments (venv/.venv) and Rust cargo build artifacts from a directory tree
- **nstar.sh** - Quick utility script

## Backup

Scripts in `backup/`:
- **backup_startup.sh** - Run backup operations at system startup
- **backup_shutdown.sh** - Run backup operations at system shutdown

## Tmux Sessions

Scripts in `tmux/`:
- **tmux_4_panes_vertical_screen.sh** - Launch tmux with 4 vertical panes
- **tmux_4_panes_horizontal_screen.sh** - Launch tmux with 4 horizontal panes

## Cron Jobs

Scripts in `cronjobs/`:
- **copy_claude_agents.sh** - Sync Claude agent configurations to backup location
- **cron_ls.sh** - List and manage cron jobs

## Utilities

- **gather_bins.sh** - Collect binary files from various locations
- **todo** - Task management binary

## Usage

Most scripts accept a path argument:
```bash
./create_notes.sh /path/to/project
./clean_artifacts.sh /path/to/clean
```

Scripts without arguments will display usage information.
