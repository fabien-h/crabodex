---
position: 2
path:
  - Usage
  - CLI
  - Markdown Front Matter headers
---

Crabodex needs a [Front Matter](https://frontmatter.codes/) header in your markdown files. It only uses the ones that have it.

Here is an example of a markdown file with a Front Matter header:

```markdown
---
position: 2
path:
  - Usage
  - CLI
  - Markdown Front Matter headers
---
```

The Front Matter header is a YAML block that starts and ends with three dashes. It contains key-value pairs that Crabodex uses to build the documentation:
- `position`: The position of the markdown file in the documentation tree. This is optional and can be used to arrange the order of your elements in the end document.
- `path`: The path of the markdown file in the documentation tree. It's used to build the table of content and the titles of the sections.

Notes:
- You should not go beyond a level 6 depths
- The path should be unique
- You don't need a file for each level of the path. You can have a file with a path of `['Usage', 'CLI']` and no file with a path of `['Usage']`. The cli will add the missing levels in the table of content and in the body of the page.
- This system is used to regroup documentation elements that belongs together logically but that are placed in different sections of your codebase.
 