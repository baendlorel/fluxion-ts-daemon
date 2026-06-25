# fluxion-daemon

🛡️ **Rust-based Linux daemon process manager with JavaScript launcher for cross-platform support**

## 🌟 Features

- 🦀 **Rust Core** - High-performance, memory-safe daemon written in Rust
- 🔄 **Auto-restart** - Automatically restarts processes when they exit or crash
- 📝 **Comprehensive Logging** - Built-in logging with timestamps to console and file
- 🛡️ **Graceful Shutdown** - Proper signal handling and process cleanup
- ⚙️ **Configurable** - JSON configuration files and command-line options
- 🌐 **Cross-platform** - JavaScript wrapper for easy cross-platform usage
- 🔒 **Process Isolation** - Daemon runs independently from parent process
- 📊 **Statistics** - Track restart counts and process health

## 🏗️ Architecture

```
┌─────────────────┐
│   JavaScript    │ ←── index.js (cross-platform launcher)
│    Wrapper      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Rust Daemon   │ ←── daemon.rs (true background process)
│   (Native)      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Your Process  │ ←── tsx main.ts (monitored application)
│   (tsx main.ts) │
└─────────────────┘
```

## 📦 Installation

### Prerequisites

- **Node.js** (>= 16.0.0)
- **Rust** (for building the daemon) - install from [rustup.rs](https://rustup.rs/)
- **tsx** (for running TypeScript files)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install tsx globally
npm install -g tsx
```

### Install Package

```bash
npm install -g fluxion-daemon
```

Or use it locally in your project:

```bash
npm install fluxion-daemon
```

## 🚀 Usage

### Basic Usage

Simply run the daemon in your project directory:

```bash
npx fluxion-daemon
```

Or if installed globally:

```bash
fluxion-daemon
```

### Advanced Usage

```bash
# Custom command and arguments
fluxion-daemon --command node --args app.js

# Set restart delay and max restarts
fluxion-daemon --delay 2000 --max-restarts 10

# Use JSON configuration file
fluxion-daemon --config daemon-config.json

# Specify working directory
fluxion-daemon --cwd /path/to/project
```

### JavaScript Usage

You can also use it as a Node.js script:

```bash
node index.js --command tsx --args main.ts
```

## ⚙️ Configuration

### Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--command` | `-C` | Command to run | `tsx` |
| `--args` | `-a` | Arguments for command | `main.ts` |
| `--delay` | `-d` | Restart delay (ms) | `1000` |
| `--max-restarts` | `-m` | Maximum restarts | unlimited |
| `--config` | `-c` | JSON config file | - |
| `--cwd` | `-w` | Working directory | current |
| `--log-file` | `-l` | Log file path | `daemon.log` |
| `--help` | `-h` | Show help message | - |

### JSON Configuration

Create a `daemon-config.json` file:

```json
{
  "command": "tsx",
  "args": ["main.ts"],
  "cwd": "./",
  "restart_delay": 1000,
  "max_restarts": null,
  "enable_logging": true,
  "log_file": "daemon.log",
  "env": [
    ["NODE_ENV", "production"],
    ["PORT", "3000"]
  ]
}
```

## 🔧 How It Works

### 1. **JavaScript Launcher** (`index.js`)
- Provides cross-platform entry point
- Parses command-line arguments
- Finds or builds Rust binary
- Launches the Rust daemon

### 2. **Rust Daemon** (`fluxion-daemon`)
- Creates true background daemon process
- Monitors child process lifecycle
- Handles signals and graceful shutdown
- Manages restart logic and logging

### 3. **Process Management**
- Spawns child process (`tsx main.ts`)
- Monitors for exit events
- Automatically restarts on failure
- Maintains process statistics

## 📝 Logging

Logs are written to multiple locations:

### Console Output
- Color-coded output with timestamps
- Real-time process monitoring
- Error and warning highlighting

### File Output
- `daemon.log` - Detailed application logs
- `/tmp/fluxion-daemon.out` - Standard output
- `/tmp/fluxion-daemon.err` - Error output

### Log Levels
- `INFO` - General information (cyan)
- `SUCCESS` - Successful operations (green)
- `WARN` - Warnings and process exits (yellow)
- `ERROR` - Errors and failures (red)

## 🧪 Examples

### Example main.ts

Create a `main.ts` file in your project:

```typescript
console.log('🚀 Starting application...');

let counter = 0;
setInterval(() => {
  counter++;
  console.log(`⚙️  Running... Counter: ${counter}`);

  // Simulate a crash every 10 seconds
  if (counter % 10 === 0) {
    console.log('💥 Simulating crash...');
    throw new Error('Simulated crash');
  }
}, 2000);
```

Run the daemon:

```bash
fluxion-daemon
```

### Development Mode

For development, you can run the Rust daemon in debug mode:

```bash
# Build debug version
npm run build:debug

# Run with debug binary
node index.js
```

## 🔨 Building from Source

### Build Rust Binary

```bash
# Release build (optimized)
cargo build --release

# Debug build (with symbols)
cargo build

# Run tests
cargo test
```

### Build Script

```bash
npm run build
```

The built binary will be located at:
- Linux/macOS: `target/release/fluxion-daemon`
- Windows: `target/release/fluxion-daemon.exe`

## 🛑 Signals

The daemon handles the following signals gracefully:

- `SIGINT` (Ctrl+C) - Graceful shutdown
- `SIGTERM` - Graceful termination
- `SIGHUP` - Graceful restart

## 📊 Process Management

### Check if Daemon is Running

```bash
# Check PID file
cat /tmp/fluxion-daemon.pid

# Find process
ps aux | grep fluxion-daemon
```

### Stop the Daemon

```bash
# Send SIGTERM
kill $(cat /tmp/fluxion-daemon.pid)

# Or use Ctrl+C when running in foreground
```

## 🐛 Troubleshooting

### "tsx not found"

```bash
npm install -g tsx
```

### "Rust not found"

Install Rust from [rustup.rs](https://rustup.rs/)

### Process not restarting

1. Check `daemon.log` for errors
2. Ensure your `main.ts` file exists
3. Verify file permissions
4. Check system resource limits

### Permission denied

```bash
chmod +x index.js
chmod +x target/release/fluxion-daemon
```

## 📁 Project Structure

```
fluxion-daemon/
├── src/
│   ├── main.rs           # Rust entry point
│   ├── daemon.rs         # Daemon implementation
│   ├── process.rs        # Process management
│   ├── config.rs         # Configuration handling
│   └── error.rs          # Error types
├── index.js              # JavaScript launcher
├── Cargo.toml            # Rust dependencies
├── package.json          # NPM configuration
├── README.md             # Documentation
└── main.ts               # Example application
```

## 🔒 Security Considerations

- Daemon runs as a separate process
- Proper signal handling prevents zombie processes
- File permissions respected for all operations
- Environment variables controlled via configuration

## 🚀 Performance

- **Memory**: Minimal footprint (~2-5MB for daemon)
- **CPU**: Efficient event-driven architecture
- **Restart Time**: < 1 second default delay
- **Overhead**: Negligible impact on child process

## 📄 License

MIT License - see LICENSE file for details

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/fluxion-daemon.git
cd fluxion-daemon

# Install dependencies
npm install

# Build Rust binary
cargo build --release

# Run tests
npm test
```

## 📞 Support

For issues, questions, or contributions, please visit:
- GitHub Issues: [github.com/yourusername/fluxion-daemon/issues](https://github.com/yourusername/fluxion-daemon/issues)

---

🛡️ **Built with Rust** - 🚀 **Powered by Node.js** - 💚 **Open Source**
