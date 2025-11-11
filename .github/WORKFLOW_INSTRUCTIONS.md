# GitHub Actions Workflow Instructions

Due to GitHub App permissions, the workflow file needs to be added manually.

## Create the workflow file

**File path**: `.github/workflows/deploy-pages.yml`

**Content**:

```yaml
name: Build and Deploy to GitHub Pages

on:
  push:
    branches:
      - master
  workflow_dispatch:

permissions:
  contents: write
  pages: write

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          override: true

      - name: Build WASM
        run: |
          cd frontend
          cargo build --release --target wasm32-unknown-unknown

      - name: List build artifacts (debug)
        run: |
          echo "Listing frontend/target/wasm32-unknown-unknown/release/:"
          ls -lah frontend/target/wasm32-unknown-unknown/release/ || echo "Directory not found"

      - name: Prepare Deployment Directory
        run: |
          mkdir -p deploy
          cp frontend/target/wasm32-unknown-unknown/release/frontend.wasm deploy/
          cp frontend/index.html deploy/

      - name: Download Macroquad JS glue
        run: |
          curl -o deploy/gl.js https://raw.githubusercontent.com/not-fl3/macroquad/master/js/gl.js

      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./deploy
```

## Setup Steps

1. **Create the file via GitHub Web Interface** (recommended):
   - Go to your repository on GitHub
   - Click "Add file" → "Create new file"
   - Name it: `.github/workflows/deploy-pages.yml`
   - Paste the content above
   - Commit directly to master (or your branch)

2. **Enable GitHub Pages**:
   - Go to repository Settings → Pages
   - Source: Select "Deploy from a branch"
   - Branch: Select `gh-pages` and `/ (root)`
   - Click "Save"

3. **Trigger deployment**:
   - Push to master branch (workflow will run automatically)
   - Or manually run the workflow from the Actions tab

**Note**: The `peaceiris/actions-gh-pages` action will automatically create the `gh-pages` branch on the first run.

## Alternative: Add via GitHub Web Interface

1. Go to your repository on GitHub
2. Click "Add file" → "Create new file"
3. Name it: `.github/workflows/deploy-pages.yml`
4. Paste the content above
5. Commit directly to master (or your branch)

## Verify Setup

After adding the workflow:
- Go to the "Actions" tab in your repository
- You should see "Deploy to GitHub Pages" workflow
- Push to master to trigger the first deployment
- Your site will be available at: `https://<username>.github.io/<repository>/`
