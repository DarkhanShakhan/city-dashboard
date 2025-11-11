# WebAssembly Deployment Guide

This guide explains how to build and deploy the City Dashboard frontend as a WebAssembly application to GitHub Pages.

## Prerequisites

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target
- Internet connection (for downloading Macroquad's JS glue code)

## Quick Start

### Install WASM Target

```bash
rustup target add wasm32-unknown-unknown
```

### Build for WebAssembly

#### Option 1: Using the build script (Recommended)

```bash
cd frontend
chmod +x build_wasm.sh
./build_wasm.sh
```

This script will:
- Install the WASM target if not present
- Build the frontend in release mode for WASM
- Create a `dist/` directory with all necessary files
- Download Macroquad's JavaScript glue code

#### Option 2: Manual build

```bash
cd frontend

# Build WASM binary
cargo build --release --target wasm32-unknown-unknown

# Create dist directory
mkdir -p dist

# Copy files
cp target/wasm32-unknown-unknown/release/frontend.wasm dist/
cp index.html dist/

# Download Macroquad JS glue
curl -o dist/gl.js https://raw.githubusercontent.com/not-fl3/macroquad/master/js/gl.js
```

### Test Locally

After building, you can test the application locally:

```bash
cd frontend/dist
python3 -m http.server 8000
```

Then open your browser to `http://localhost:8000`

**Note**: You must serve the files via HTTP/HTTPS. Opening `index.html` directly as a file won't work due to CORS restrictions.

## GitHub Pages Deployment

### Automatic Deployment (CI/CD)

The repository includes a GitHub Actions workflow that automatically builds and deploys to GitHub Pages on every push to the `master` branch.

**Setup Steps:**

1. **Enable GitHub Pages** in your repository settings:
   - Go to: `Settings` → `Pages`
   - Source: `GitHub Actions`

2. **Push to master branch**:
   ```bash
   git push origin master
   ```

3. **Wait for the workflow** to complete:
   - Go to the `Actions` tab in your repository
   - Monitor the "Deploy to GitHub Pages" workflow

4. **Access your deployed app**:
   - Your app will be available at: `https://<username>.github.io/<repository-name>/`
   - For this repo: `https://darkhanshakhan.github.io/city-dashboard/`

### Manual Deployment

If you prefer to deploy manually:

1. Build the WASM application:
   ```bash
   cd frontend
   ./build_wasm.sh
   ```

2. The `dist/` directory contains all files needed for deployment

3. Deploy the contents of `dist/` to any static hosting service:
   - GitHub Pages
   - Netlify
   - Vercel
   - Cloudflare Pages
   - AWS S3 + CloudFront
   - etc.

## File Structure

After building, the `dist/` directory will contain:

```
dist/
├── index.html          # HTML page that loads the WASM module
├── frontend.wasm       # Compiled Rust application
└── gl.js              # Macroquad's WebGL JavaScript glue code
```

## Troubleshooting

### Build Fails

**Error**: `error: linking with 'rust-lld' failed`

**Solution**: Make sure you have the WASM target installed:
```bash
rustup target add wasm32-unknown-unknown
```

### Black Screen or Loading Forever

**Possible causes:**
1. **WASM file not found** - Check browser console for 404 errors
2. **CORS issues** - Make sure you're serving via HTTP, not opening files directly
3. **Browser compatibility** - Use a modern browser (Chrome, Firefox, Safari, Edge)

**Solution**: Check the browser developer console (F12) for error messages.

### Canvas Not Displaying

**Cause**: Canvas element might not be initialized properly

**Solution**:
- Refresh the page
- Check that `gl.js` is properly loaded
- Ensure JavaScript is enabled in your browser

### Performance Issues

**Tips for better performance:**
- Build with `--release` flag (production mode)
- Use a modern browser with good WebGL support
- Close unnecessary browser tabs/applications

## Browser Compatibility

The application requires:
- WebAssembly support
- WebGL 2.0 support
- Modern JavaScript (ES6+)

**Supported browsers:**
- Chrome/Chromium 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Optimizations

### Reducing WASM File Size

1. **Enable LTO (Link Time Optimization)**:

   Add to `Cargo.toml`:
   ```toml
   [profile.release]
   lto = true
   opt-level = 'z'  # Optimize for size
   ```

2. **Strip debug symbols**:
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   wasm-strip target/wasm32-unknown-unknown/release/frontend.wasm
   ```

3. **Use wasm-opt** (from Binaryen):
   ```bash
   wasm-opt -Oz -o frontend_optimized.wasm frontend.wasm
   ```

### Enable Compression

Most hosting services automatically compress files, but you can pre-compress:

```bash
# Brotli compression (best)
brotli -q 11 dist/frontend.wasm

# Gzip compression (fallback)
gzip -9 -k dist/frontend.wasm
```

## Development Workflow

### Watch and Rebuild

For development, you can use `cargo watch`:

```bash
cargo install cargo-watch

# Auto-rebuild on file changes
cd frontend
cargo watch -x 'build --target wasm32-unknown-unknown'
```

### Live Reload

Use a development server with live reload:

```bash
# Install basic-http-server
cargo install basic-http-server

# Serve with auto-reload
cd frontend/dist
basic-http-server -a 0.0.0.0:8000
```

## CI/CD Workflow

The GitHub Actions workflow (`.github/workflows/deploy-pages.yml`) performs these steps:

1. **Checkout code** from the repository
2. **Setup Rust** with wasm32 target
3. **Cache dependencies** for faster builds
4. **Build WASM** in release mode
5. **Prepare distribution files** (HTML, WASM, JS)
6. **Deploy to GitHub Pages** using the official action

**Workflow triggers:**
- Push to `master` branch
- Manual trigger via GitHub UI (`workflow_dispatch`)

## Additional Resources

- [Macroquad Documentation](https://github.com/not-fl3/macroquad)
- [WebAssembly](https://webassembly.org/)
- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)

## Controls

When running in the browser:

- **Enter** - Toggle emergency stop mode (all lights red)
- **Shift** - Toggle danger mode (LED warning display)
- **Escape** - Reset simulation

## Support

If you encounter issues:

1. Check the browser console for errors (F12)
2. Verify all files are present in the `dist/` directory
3. Ensure you're using a supported browser
4. Try clearing your browser cache
5. Check the GitHub Actions logs if auto-deployment fails

## License

Same as the main project.
