> [!WARNING]
> Early stages of development. Everything might change. Many things might not work.

pelp - a **p*resentation h**elp**er. Makes it easy to create a revealjs presentation from a Markdown file.

- Creates and serves a presentation
- Updates presentation in a browser when a source file changes
- Helps to create and manage recurring presentations

## Dependencies:

- pandoc - to build the presentation
- live-server - to update the presentation in the browser (when the source file changes)
- inotifywait - to watch for changes in the source file
- date - to calculate the next Monday
- sed - to replace the date in the template file

Some dependencies might be removed due to functionality being implemented by pelp itselmented by pelp.

## Roadmap

- [ ] design & implement the `new` command
- [ ] package for nix
- [ ] add support for recurring presentations
- [ ] add support for templates (at least for recurring presentations)
- [ ] design & implement the `deploy` command
- [ ] brush the project up
  - [ ] nice error handling
  - [ ] less unwrap(), more handling

# Usage as a flake

[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/ejiek/pelp/badge)](https://flakehub.com/flake/ejiek/pelp)

Add pelp to your `flake.nix`:

```nix
{
  inputs.pelp.url = "https://flakehub.com/f/ejiek/pelp/*.tar.gz";

  outputs = { self, pelp }: {
    # Use in your outputs
  };
}

```
