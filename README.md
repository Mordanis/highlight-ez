Create HTML renderings of code with highlighting using
[tree-sitter](https://github.com/tree-sitter/tree-sitter)

The general workflow of this is to simplify the workflow of creating pretty html code blocks
using tree-sitter.

```rust
let my_pyblock = r#"def fib(a):
    if a = 1:
        return 1
    else:
        return fib(a - 1)"#;
let lang = TargetLanguage::Python;
let html = render_html(my_pyblock, lang);
```
