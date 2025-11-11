# 🏙️ City Dashboard

A real-time traffic simulation built with Rust and Macroquad. Watch cars navigate through intersections, obey traffic lights, and avoid collisions in a dynamic city environment.

![Traffic Simulation](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Macroquad](https://img.shields.io/badge/Macroquad-orange?style=for-the-badge)

## 🎮 Demo

**Live Demo**: Coming soon at GitHub Pages!

## ✨ Features

- **Real-time Traffic Simulation** - Cars spawn, drive, turn, and follow traffic rules
- **Traffic Light System** - Automated traffic lights with proper timing (green, yellow, red)
- **Collision Avoidance** - Cars maintain safe following distances
- **Left-hand Traffic** - Proper lane discipline implementation
- **LED Display** - Scrolling message display with danger warnings
- **Interactive Controls** - Emergency stop, danger mode, and reset functions
- **Responsive Design** - Works on different screen sizes
- **2.5D Graphics** - Depth effects for visual polish

## 🎯 Controls

| Key | Action |
|-----|--------|
| `Enter` | Toggle emergency stop mode (all lights red) |
| `Shift` | Toggle danger mode (LED warning display) |
| `Escape` | Reset simulation to initial state |

## 🚀 Quick Start

### Prerequisites

- **Rust** (stable toolchain)
- **Cargo** (comes with Rust)

### Running Locally (Native)

```bash
# Clone the repository
git clone https://github.com/DarkhanShakhan/city-dashboard.git
cd city-dashboard/frontend

# Run the simulation
cargo run --release
```

### Running in Browser (WebAssembly)

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Build for web
cd frontend
chmod +x build_wasm.sh
./build_wasm.sh

# Serve locally
cd dist
python3 -m http.server 8000

# Open http://localhost:8000 in your browser
```

For detailed WebAssembly deployment instructions, see [frontend/WASM_DEPLOYMENT.md](frontend/WASM_DEPLOYMENT.md).

## 📁 Project Structure

```
city-dashboard/
├── frontend/                    # Frontend application
│   ├── src/
│   │   ├── main.rs             # Application entry point
│   │   ├── models.rs           # Data structures (Car, Direction, etc.)
│   │   ├── constants.rs        # Configuration constants
│   │   ├── city.rs             # City container & orchestration
│   │   ├── rendering.rs        # Graphics rendering pipeline
│   │   ├── car.rs              # Vehicle behavior & physics
│   │   ├── intersection.rs     # Traffic intersections
│   │   ├── traffic_light.rs    # Traffic light logic
│   │   ├── road.rs             # Road definitions
│   │   ├── block.rs            # City blocks
│   │   ├── spawner.rs          # Car spawning system
│   │   └── input.rs            # Input handling
│   ├── index.html              # WebAssembly HTML template
│   ├── build_wasm.sh           # WASM build script
│   ├── WASM_DEPLOYMENT.md      # Deployment guide
│   └── REFACTORING_SUGGESTIONS.md  # Code improvement ideas
├── .github/
│   └── workflows/
│       └── deploy-pages.yml    # GitHub Pages deployment
└── README.md                    # This file
```

## 🏗️ Architecture

The application follows a clean architecture with clear separation of concerns:

- **Models** (`models.rs`) - Core data structures
- **City** (`city.rs`) - Main container coordinating all elements
- **Rendering** (`rendering.rs`) - Visual output pipeline
- **Simulation** (`car.rs`, `traffic_light.rs`) - Traffic logic
- **Constants** (`constants.rs`) - Centralized configuration

**Key Patterns:**
- Builder pattern for flexible object construction
- Trait-based polymorphism for extensibility
- Percentage-based positioning for responsive design
- Layered rendering (environment → traffic → overlays)

For detailed refactoring suggestions and architecture improvements, see [frontend/REFACTORING_SUGGESTIONS.md](frontend/REFACTORING_SUGGESTIONS.md).

## 🎨 Technical Details

### Technology Stack

- **Language**: Rust (Edition 2024)
- **Framework**: Macroquad 0.4.14
- **Graphics**: 2D rendering with WebGL backend
- **Target**: Native (Windows, macOS, Linux) + WebAssembly

### Simulation Features

**Traffic Behavior:**
- Left-hand traffic enforcement
- Traffic light compliance (red, yellow, green)
- Collision detection and avoidance
- Safe following distance maintenance
- Random turning at intersections (30% probability)
- Intersection navigation with proper lane changes

**Visual Elements:**
- Grass blocks with 2.5D depth effects
- Dashed road center lines
- Zebra-striped crosswalks
- Color-coded vehicles (5 colors)
- Realistic traffic lights with state cycling
- LED dot-matrix display (5x7 character patterns)
- Scrolling text: "WELCOME TO CITY"
- Flashing danger warnings

**Performance:**
- Percentage-based positioning (resolution-independent)
- Efficient HashMap lookups (O(1) for roads/intersections)
- Automatic car cleanup when off-screen
- Frame-rate independent updates using delta time

### Code Statistics

- **Total**: ~4,376 lines across 12 modules
- **Largest module**: rendering.rs (766 lines)
- **Documentation**: Comprehensive inline docs
- **Complexity**: Well-organized with clear responsibilities

## 📦 Building

### Native Build

```bash
cd frontend

# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run --release
```

### WebAssembly Build

See [frontend/WASM_DEPLOYMENT.md](frontend/WASM_DEPLOYMENT.md) for detailed instructions.

## 🌐 Deployment

### GitHub Pages (Automatic)

1. **Enable GitHub Pages** in repository settings
   - Settings → Pages → Source: GitHub Actions

2. **Push to master** - Deployment happens automatically
   ```bash
   git push origin master
   ```

3. **Access** your app at:
   ```
   https://<username>.github.io/<repository>/
   ```

The deployment workflow is defined in `.github/workflows/deploy-pages.yml`.

### Other Hosting Options

The `dist/` directory (after building) can be deployed to:
- Netlify
- Vercel
- Cloudflare Pages
- AWS S3 + CloudFront
- Any static hosting service

## 🔧 Configuration

All simulation parameters are centralized in `frontend/src/constants.rs`:

**Vehicle Constants:**
- Car dimensions: 20×35 pixels
- Speed: 50 pixels/second
- Spawn interval: 1.5 seconds
- Turn probability: 30%
- Safe following distance: 50 pixels

**Traffic Light Timing:**
- Green: 3 seconds
- Yellow: 1 second
- Red: 3 seconds

**Visual Settings:**
- Road width: 60 pixels
- Grass colors, depth effects
- LED display configuration

## 🧪 Testing

```bash
cd frontend

# Run tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🤝 Contributing

Contributions are welcome! Areas for improvement:

1. **High Priority**:
   - Split large modules (rendering.rs, traffic_light.rs)
   - Add type safety with newtypes
   - Optimize car collision detection

2. **Medium Priority**:
   - Add error handling
   - Configuration file support
   - Unit tests

3. **Low Priority**:
   - Debug visualization mode
   - ECS architecture migration
   - Statistics tracking

See [frontend/REFACTORING_SUGGESTIONS.md](frontend/REFACTORING_SUGGESTIONS.md) for detailed suggestions.

## 📝 License

[Specify your license here]

## 🙏 Acknowledgments

- Built with [Macroquad](https://github.com/not-fl3/macroquad) - Simple and easy-to-use game library
- Rust programming language and community
- WebAssembly for enabling web deployment

## 📧 Contact

[Your contact information]

---

**Made with ❤️ and 🦀 Rust**
