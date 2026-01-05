# Plan for combine.sh Script

## Overview
Create a Node.js CLI tool using gunshi that combines all markdown plans from the `prompt/` directory into a single markdown file. The tool should accept file patterns or filenames as arguments, validate files exist, and ensure all plans have proper markdown headers.

## Beads Tickets

- **rekon-yv0** (Epic): Create combine.sh script to merge prompt plans

## Implementation Tasks

### 1. Create combine.ts file with gunshi
- Import gunshi CLI utilities
- Define command structure with file pattern arguments
- Create async generator `prompts` that returns list of files
- Create async generator that yields `{name, content}` objects
- Create `writeCombined` function to consume the stream and write output

### 2. Implement file validation logic
- Search prompt/ directory for matching files using glob patterns
- Filter out files that don't exist or can't be read
- Handle both explicit filenames and glob patterns

### 3. Implement header generation
- Check if plan content starts with '#'
- If not, generate header based on filename
- Add first line: "this is the prompt called <filename>"

### 4. Create combine.sh wrapper
- Make it executable with proper shebang
- Call combine.ts with passed arguments
- Ensure proper error handling

### 5. Testing
- Test with all prompt/*.md files
- Test with individual filenames
- Test with glob patterns
- Test handling of plans without headers
