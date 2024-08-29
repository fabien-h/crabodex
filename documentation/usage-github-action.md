---
path:
  - Usage
  - GitHub Actions
---

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