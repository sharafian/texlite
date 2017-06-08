# TexLite
> Markdown that looks like LaTeX

## Example

The file `example.txl` contains some texlite markdown.

```sh
cargo run example.txl > example.html
open example.html
```

## Syntax Spec

- `\x{block}` compiles to `<x>inner text</x>`.
- `block\n\nblock` compiles to `<p>block</p><p>block</p>`.

## TODOs

- Set fields (i.e. `href` on commands)
- Escape special characters
- Print debug output to stderr
- ~Read from file/stdin instead of hard-coded string~
- Represent tokens as linked list instead of vector
- Proper syntax errors
- ~Multiple paragraphs~
- ~Structure code to break up tokenizer, parser, unparser~
- Webserver to view files
