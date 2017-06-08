# TexLite
> Markdown that looks like LaTeX

## Example

Input:

```
Lorem ipsum \b{dorem} sit amet.
```

Output:

```html
<p>Lorem ipsum <b>dorem</b> sit amet.</p>
```

## TODOs

- Represent tokens as linked list instead of vector
- Proper syntax errors
- Multiple paragraphs
- Structure code to break up tokenizer, parser, unparser
