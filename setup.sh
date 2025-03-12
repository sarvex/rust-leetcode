#!/bin/bash

# Remove all files except JS and TS files
find . -type f -not -name "*.rs" -delete

# Process Code files - move up three levels and rename
for ext in rs; do
  find . -name "*.$ext" -type f | while read -r file; do
    dir=$(basename "$(dirname "$file")")
    up=$(dirname "$(dirname "$(dirname "$(dirname "$file")")")")
    mv -n "$file" "$up/$dir.$ext"
  done
done

echo "Setup completed successfully!"
