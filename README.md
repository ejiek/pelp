pelp - a **p*resentation h**elp**er. Makes it easy to create reveljs presentation from Markdown file.

Creates and serves a presentation
New presentations each Monday

## Dependencies:

- pandoc - to build the presentation
- live-server - to update the presentation in the browser (when the source file changes)
- inotifywait - to watch for changes in the source file
- date - to calculate the next Monday
- sed - to replace the date in the template file

Some dependencies might be removed due to functionality being implemented by pelp itselmented by pelp.
