# LearnWiki MarkItDown Bundle

This directory is the packaging location for LearnWiki's bundled document
converter.

Run `src-tauri/scripts/setup_markitdown.sh` on macOS/Linux or
`src-tauri/scripts/setup_markitdown.ps1` on Windows before creating a release
build. The scripts build a bundled converter executable with only the
MarkItDown extras needed for PDF, DOCX, and PPTX imports.

Tauri may place that executable in the final app bundle as either
`markitdown/learnwiki-markitdown` or `markitdown/bin/learnwiki-markitdown`;
on Windows the executable is named `learnwiki-markitdown.exe`. The app checks
all of these locations.
