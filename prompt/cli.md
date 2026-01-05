# CLI

General guidelines for CLI authoring

- FILE-OR-PATTERN is a common argument that we want to reach for. many CLI programs expect a file name, but we want to expedite the user experience & hasten their operational speed by allowing them to provide a regex pattern in place of a file name at any point. FILE-OR-PATTERN means that if there is no FILE (or other record of any other file type) found at the provided argument, we treat it as a regex.
  - Often there is an implied EXPECTED directory or directories where we want to look for the pattern, in addition to cwd or sometimes ONLY look in the EXPECTED directories.
  - Use streams! FILE-OR-PATTERN
  - If the argument starts with . or /, do not search anything but the present or absolute directory.
