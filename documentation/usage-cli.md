---
position: 1
path:
  - Usage
  - CLI
---

At its core, Crabodex is a command line interface (CLI) tool.

To use it locally, you can download the release here : https://github.com/fabien-h/crabodex/releases and use the binary. But this is really a tool meant to be used only in a CI/CD pipeline.

Here are the parameters you can use with the CLI:

- `--root-directory`: The root directory of your project. This is where Crabodex will start scanning for Markdown files. Default is the current directory.
- `--repo-name`: The name of your repository. This is used to generate the title of the documentation. Default is the name of the root directory.
- `--repo-description`: The description of your repository. This is used to generate the description of the documentation. Default is an empty string.
- `--commit-hash`: The hash of the commit. This is used to generate the version of the documentation and a link to the commit page. Default is `latest`.
- `--repo-url`: The URL of your repository. This is used to generate the links to the source code. Default is an empty string.
- `--ignore-folders`: A list of folders to ignore. This is used to exclude folders from the documentation. There are some opinionated default value. You can add more by separating them with a comma. ex: `--ignore-folders docs/,tests/`

Example :

```bash
crabodex --root-directory ./docs --repo-name "My awesome project" --repo-description "This is my awesome project" --commit-hash 1234567890 --repo-url https://github.com/me/my-awesome-project --ignore-folders docs/,tests/
```

This won't do anything locally. This tool is super agnostic and the result of this command is simply a string in the stdout. You can redirect it to a file to save it.

For example, you can redirect the output to a file:

```bash
crabodex  > docs.json
```

If you don't, it will just be displayed in the console.
