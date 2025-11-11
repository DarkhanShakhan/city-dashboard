# GitHub Actions Workflow Instructions

Due to GitHub App permissions, the workflow file needs to be added manually.

## Create the workflow file

**File path**: `.github/workflows/deploy-pages.yml`

**Content**:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [master]
  workflow_dispatch:

# Sets permissions of the GITHUB_TOKEN to allow deployment to GitHub Pages
permissions:
  contents: read
  pages: write
  id-token: write

# Allow only one concurrent deployment
concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          override: true

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: frontend/target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Build WASM
        run: |
          cd frontend
          cargo build --release --target wasm32-unknown-unknown

      - name: Prepare dist directory
        run: |
          mkdir -p dist
          cp frontend/target/wasm32-unknown-unknown/release/frontend.wasm dist/
          cp frontend/index.html dist/

      - name: Download Macroquad JS glue
        run: |
          curl -o dist/gl.js https://raw.githubusercontent.com/not-fl3/macroquad/master/js/gl.js

      - name: Setup Pages
        uses: actions/configure-pages@v4

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: ./dist

      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## Setup Steps

1. **Create the directory** (if it doesn't exist):
   ```bash
   mkdir -p .github/workflows
   ```

2. **Create the file** and paste the content above:
   ```bash
   # Using your preferred editor
   nano .github/workflows/deploy-pages.yml
   # or
   vim .github/workflows/deploy-pages.yml
   # or use GitHub web interface
   ```

3. **Commit the workflow**:
   ```bash
   git add .github/workflows/deploy-pages.yml
   git commit -m "Add GitHub Pages deployment workflow"
   git push
   ```

4. **Enable GitHub Pages**:
   - Go to repository Settings → Pages
   - Source: Select "GitHub Actions"

5. **Trigger deployment**:
   - Push to master branch, or
   - Manually run the workflow from the Actions tab

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
