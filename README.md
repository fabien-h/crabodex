# Crabodex

![Static Badge](https://img.shields.io/badge/Github-fabien--h%2Fcrabodex-dddddd?logo=github)
![Static Badge](https://img.shields.io/badge/licence-MIT-dddddd?logo=opensourceinitiative&logoColor=%23ffffff)
![Tests](https://img.shields.io/badge/tests-passing-green)
![Build](https://img.shields.io/badge/build-passing-green)


Crabodex is a versatile and efficient documentation generator. It scans your project directory, processes Markdown files, and generates a one-page HTML documentation.

The end goal is to have a centralised document with all the business rules that exist in your repository. It is aimed as much at developers as it is at business analysts, product owners, and other stakeholders. They can know everything that has been implemented and how it works without having to read the code.

It tackles the main problem of documentations : keeping them up to date. With crabodex, you are able to colocate code and documentation. Each documentation file seats next to the code it describes. You can easily check at PR time that the documentation is updated when you change the code.


## Usage

### CLI

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
crabodex > docs.json
```

If you don't, it will just be displayed in the console.

#### Local installation

On MacOS and Linux, you can install the CLI locally using

`curl -sSL https://raw.githubusercontent.com/fabien-h/crabodex/main/scripts/install.sh | bash`

On Windows, you can install the CLI locally using

`Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/fabien-h/crabodex/main/scripts/install.ps1" -UseBasicParsing).Content`

#### Markdown Front Matter headers

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

#### Default ignored folders

By default, those folders are ignored:

```
".git/",
".svn/",
".hg/",
"build/",
"dist/",
"out/",
"bin/",
"target/",
".idea/",
".vscode/",
".vs/",
".eclipse/",
"node_modules/",
```

### Usage in a GitHub action

GitHub Action example:

```yaml
# Git it the name you want
name: documentation

# Usually a good idea
env:
  FORCE_JAVASCRIPT_ACTIONS_TO_NODE20: true

# Trigger the action on push to main, push also means merge
on:
  push:
    branches: [ main ]

jobs:
  # Name it the way you want
  generate-and-deploy-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      # This steps programmatically gets the latest release of crabodex
      # You can hardcode a version if you prefer
      - name: Get latest release
        id: get_release
        uses: actions/github-script@v7
        with:
          github-token: ${{secrets.GITHUB_TOKEN}}
          script: |
            const release = await github.rest.repos.getLatestRelease({
              owner: "fabien-h",
              repo: "crabodex",
            });
            core.setOutput('release_tag', release.data.tag_name);

      # Download the latest release of crabodex from the release
      - name: Download crabodex from release
        run: |
          RELEASE_TAG="${{ steps.get_release.outputs.release_tag }}"
          OS=$(echo $RUNNER_OS | tr '[:upper:]' '[:lower:]')
          ARCH=$(uname -m)

          if [ "$ARCH" = "x86_64" ]; then
            ARCH="amd64"
          elif [ "$ARCH" = "aarch64" ]; then
            ARCH="arm64"
          fi

          if [ "$OS" = "windows" ]; then
            BINARY_NAME="crabodex-${OS}-${ARCH}.exe"
          else
            BINARY_NAME="crabodex-${OS}-${ARCH}"
          fi

          DOWNLOAD_URL="https://github.com/${{ github.repository }}/releases/download/${RELEASE_TAG}/${BINARY_NAME}"

          echo "Downloading from: $DOWNLOAD_URL"

          curl -L "$DOWNLOAD_URL" -o crabodex

          if [ "$OS" != "windows" ]; then
            chmod +x crabodex
          fi

      - name: Verify crabodex
        run: |
          ls -l crabodex*
          file crabodex*
          if [ "$RUNNER_OS" != "Windows" ]; then
            ./crabodex --version
          else
            ./crabodex.exe --version
          fi

      # Generate the documentation using the downloaded binary
      - name: Generate documentation
        env:
          REPO_NAME: ${{ github.repository }}
          REPO_DESCRIPTION: ${{ github.event.repository.description }}
          GITHUB_SERVER_URL: ${{ github.server_url }}
          ACTIONS_RUNNER_DEBUG: true
        # This step generates the documentation, especially the --ignore-folders
        # and choose what to do with the output data
        run: |
          COMMIT_HASH_SHORT=$(git rev-parse --short HEAD)
          mkdir docs
          ./crabodex \
            --ignore-folders tests \
            --repo-name "$REPO_NAME" \
            --repo-description "$REPO_DESCRIPTION" \
            --commit-hash "$COMMIT_HASH_SHORT" \
            --repo-url "$GITHUB_SERVER_URL/$REPO_NAME" \
            > ./docs/index.html

      # Deploy the documentation to GitHub Pages
      # This step is optional, you can choose to deploy the documentation anywhere
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs
          force_orphan: true
```

In this example, the documentation is simply deployed as a GitHub page. If this is what you want, don't forget a couple of things :
- Enable GitHub pages in the settings of your repository.
- Allow your actions to write in the repository settings.

## License

This project is licensed under the MIT License.
