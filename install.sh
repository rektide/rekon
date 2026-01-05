#!/bin/bash

# Install script to symlink project prompts to OpenCode config directory
# This makes project-specific prompts available globally in OpenCode

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROMPT_DIR="$SCRIPT_DIR/prompt"
OPENCODE_PROMPT_DIR="$HOME/.config/opencode/prompts"

# Parse arguments
CLEANUP=false
HELP=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --cleanup|-c)
            CLEANUP=true
            shift
            ;;
        --help|-h)
            HELP=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            HELP=true
            shift
            ;;
    esac
done

# Show help
if [ "$HELP" = true ]; then
    cat <<EOF
Usage: install.sh [OPTIONS]

Symlinks project prompts to OpenCode config directory.

Options:
  --cleanup, -c    Remove all symlinks in OpenCode prompts directory
  --help, -h       Show this help message

Examples:
  ./install.sh           Install prompts
  ./install.sh --cleanup   Remove all symlinks

EOF
    exit 0
fi

# Cleanup mode
if [ "$CLEANUP" = true ]; then
    echo "Cleaning up symlinks in $OPENCODE_PROMPT_DIR..."
    
    if [ ! -d "$OPENCODE_PROMPT_DIR" ]; then
        echo "No OpenCode prompts directory found (nothing to clean)"
        exit 0
    fi
    
    count=0
    for link in "$OPENCODE_PROMPT_DIR"/*; do
        if [ -L "$link" ]; then
            rm -f "$link"
            echo "✓ Removed $(basename "$link")"
            ((count++))
        fi
    done
    
    echo ""
    echo "Removed $count symlink(s)"
    exit 0
fi

# Install mode
# Create OpenCode prompts directory if it doesn't exist
if [ ! -d "$OPENCODE_PROMPT_DIR" ]; then
    mkdir -p "$OPENCODE_PROMPT_DIR"
fi

echo "Installing prompts from $PROMPT_DIR to $OPENCODE_PROMPT_DIR"
echo ""

# Check if prompt directory exists
if [ ! -d "$PROMPT_DIR" ]; then
    echo "Error: Prompt directory '$PROMPT_DIR' not found"
    exit 1
fi

# Symlink each prompt file
count=0
for prompt_file in "$PROMPT_DIR"/*.md; do
    if [ -f "$prompt_file" ]; then
        filename=$(basename "$prompt_file")
        target="$OPENCODE_PROMPT_DIR/$filename"
        
        # Remove existing symlink or file
        if [ -e "$target" ] || [ -L "$target" ]; then
            rm -rf "$target"
        fi
        
        # Create symlink
        ln -s "$prompt_file" "$target"
        echo "✓ Linked $filename"
        ((count++))
    fi
done

echo ""
echo "Installation complete! Linked $count prompt(s)"
echo "Prompts are now available in OpenCode at: $OPENCODE_PROMPT_DIR"
echo ""
echo "To use a prompt, configure it in opencode.json:"
echo '  "mode": {'
echo '    "your-mode": {'
echo '      "prompt": "{file:./prompts/your-prompt.md}"'
echo '    }'
echo '  }'
