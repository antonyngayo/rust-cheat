## Purpose
```
This is a small rust project that helps me organize my cheat sheets
```

## Usage

### Commands
- `cheat list` or `cheat l` - List all cheat sheet files
- `cheat edit <filename>` or `cheat e <filename>` - Edit a cheat sheet file (creates if doesn't exist)
- `cheat search <query>` or `cheat s <query>` - Fuzzy search for cheat sheet files by name
- `cheat find` or `cheat f` - Interactive fuzzy finder
- `cheat delete <filename>` or `cheat d <filename>` - Delete a cheat sheet file
- `cheat push --message "commit message"` or `cheat p -m "commit message"` - Push changes to git repository
- `cheat <filename>` - Display contents of a cheat sheet

### Search Features
The search functionality uses advanced fuzzy matching with multiple strategies:
- **Exact matches** (100% score) - Perfect filename matches
- **Prefix matches** (90%+ score) - Files starting with your query
- **Substring matches** (70%+ score) - Files containing your query anywhere
- **Fuzzy matches** (50%+ score) - Files with characters appearing in sequence
- **Edit distance** (30%+ score) - Files with similar spellings

Results are filtered to show high-quality matches (>80%) first, with medium-quality fallback (>50%) when needed.

### Examples
```bash
# List all cheat sheets (long and short forms)
cheat list
cheat l

# Edit or create a new cheat sheet
cheat edit docker-commands
cheat e docker-commands

# Search for files containing "git" (fuzzy search)
cheat search git
cheat s git

# Interactive fuzzy finder
cheat find
cheat f

# View a cheat sheet
cheat docker-commands

# Delete a cheat sheet
cheat delete old-notes
cheat d old-notes

# Push changes to git
cheat push --message "Added new docker commands"
cheat p -m "Added new docker commands"
```


