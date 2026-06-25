#!/usr/bin/env node

/**
 * Fluxion Daemon - Cross-platform launcher for Rust daemon binary
 *
 * This JavaScript wrapper serves as a convenient entry point for launching
 * the Rust-based daemon process across different platforms.
 */

import { spawn, execSync } from 'child_process';
import { dirname, join, resolve } from 'path';
import { fileURLToPath } from 'url';
import { existsSync, chmodSync } from 'fs';
import { platform } from 'os';

const __dirname = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = process.cwd();

// Configuration for Rust binary
const CONFIG = {
  command: 'tsx',
  args: ['main.ts'],
  restartDelay: 1000,
  maxRestarts: null,
  workingDir: PROJECT_ROOT,
  logFile: join(PROJECT_ROOT, 'daemon.log')
};

/**
 * Simple logger with timestamps and colors
 */
function log(message, level = 'INFO') {
  const timestamp = new Date().toISOString();
  const logMessage = `[${timestamp}] [${level}] ${message}`;

  const colors = {
    INFO: '\x1b[36m',    // cyan
    SUCCESS: '\x1b[32m', // green
    WARN: '\x1b[33m',    // yellow
    ERROR: '\x1b[31m'    // red
  };
  const reset = '\x1b[0m';
  const color = colors[level] || colors.INFO;

  console.log(`${color}${logMessage}${reset}`);
}

/**
 * Find the Rust binary executable
 */
function findRustBinary() {
  const currentPlatform = platform();

  // Possible binary locations
  const possiblePaths = [
    // Local development paths
    join(__dirname, 'target', 'release', 'fluxion-daemon'),
    join(__dirname, 'target', 'debug', 'fluxion-daemon'),
    join(__dirname, 'fluxion-daemon'),
    join(__dirname, 'bin', 'fluxion-daemon'),
    // Installed package paths
    join(__dirname, '..', '.bin', 'fluxion-daemon'),
  ];

  // Add platform-specific extensions
  if (currentPlatform === 'win32') {
    possiblePaths.splice(0, 0, ...possiblePaths.map(p => `${p}.exe`));
  }

  for (const path of possiblePaths) {
    if (existsSync(path)) {
      log(`Found Rust binary: ${path}`, 'SUCCESS');
      return path;
    }
  }

  return null;
}

/**
 * Build the Rust binary if needed
 */
function buildRustBinary() {
  log('Rust binary not found. Attempting to build...', 'WARN');

  try {
    // Check if cargo is available
    execSync('cargo --version', { stdio: 'ignore' });

    log('Building Rust daemon with cargo...', 'INFO');
    execSync('cargo build --release', {
      cwd: __dirname,
      stdio: 'inherit'
    });

    const binaryPath = join(__dirname, 'target', 'release', 'fluxion-daemon');
    if (existsSync(binaryPath)) {
      // Make binary executable
      chmodSync(binaryPath, 0o755);
      log(`Successfully built Rust binary: ${binaryPath}`, 'SUCCESS');
      return binaryPath;
    }

  } catch (error) {
    log(`Failed to build Rust binary: ${error.message}`, 'ERROR');
  }

  return null;
}

/**
 * Parse command line arguments
 */
function parseArgs() {
  const args = process.argv.slice(2);
  const rustArgs = [];

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg === '--command' || arg === '-C') {
      CONFIG.command = args[++i];
    } else if (arg === '--args' || arg === '-a') {
      CONFIG.args = args[++i].split(/\s+/);
    } else if (arg === '--delay' || arg === '-d') {
      CONFIG.restartDelay = parseInt(args[++i]);
    } else if (arg === '--max-restarts' || arg === '-m') {
      CONFIG.maxRestarts = parseInt(args[++i]);
    } else if (arg === '--help' || arg === '-h') {
      printUsage();
      process.exit(0);
    } else if (arg.startsWith('-')) {
      // Pass through to Rust binary
      rustArgs.push(arg);
      if (i + 1 < args.length && !args[i + 1].startsWith('-')) {
        rustArgs.push(args[++i]);
      }
    }
  }

  return rustArgs;
}

/**
 * Print usage information
 */
function printUsage() {
  console.log(`
🚀 Fluxion Daemon - Cross-platform Process Guardian

Usage: fluxion-daemon [OPTIONS] [RUST_OPTIONS]

Options:
  --command, -C <CMD>     Command to run (default: tsx)
  --args, -a <ARGS>       Arguments for command (default: main.ts)
  --delay, -d <MS>        Restart delay in milliseconds (default: 1000)
  --max-restarts, -m <N>  Maximum restarts (default: unlimited)
  --help, -h              Show this help message

Rust Options (passed to Rust binary):
  -c, --config <FILE>    Configuration file (JSON)
  -w, --cwd <DIR>        Working directory
  -l, --log-file <FILE>  Log file path

Examples:
  fluxion-daemon
  fluxion-daemon --command node --args app.js
  fluxion-daemon --max-restarts 10 --delay 2000
  fluxion-daemon -c daemon-config.json

For more information, visit: https://github.com/yourusername/fluxion-daemon
  `);
}

/**
 * Launch the Rust daemon
 */
function launchRustDaemon(rustArgs) {
  let binaryPath = findRustBinary();

  if (!binaryPath) {
    binaryPath = buildRustBinary();
  }

  if (!binaryPath) {
    log('❌ Could not find or build Rust daemon binary', 'ERROR');
    log('💡 Please ensure Rust is installed and run: cargo build --release', 'INFO');
    process.exit(1);
  }

  // Build Rust arguments
  const rustCommand = [binaryPath, ...rustArgs];

  // Add default command and args if not provided
  rustCommand.push('--command', CONFIG.command);
  rustCommand.push('--args', CONFIG.args.join(' '));

  if (CONFIG.restartDelay !== 1000) {
    rustCommand.push('--delay', CONFIG.restartDelay.toString());
  }

  if (CONFIG.maxRestarts !== null) {
    rustCommand.push('--max-restarts', CONFIG.maxRestarts.toString());
  }

  if (CONFIG.workingDir) {
    rustCommand.push('--cwd', CONFIG.workingDir);
  }

  if (CONFIG.logFile) {
    rustCommand.push('--log-file', CONFIG.logFile);
  }

  log('🚀 Launching Rust daemon...', 'INFO');
  log(`📝 Command: ${rustCommand.join(' ')}`, 'INFO');

  try {
    // Spawn the Rust daemon
    const daemonProcess = spawn(rustCommand[0], rustCommand.slice(1), {
      stdio: 'inherit',
      cwd: __dirname,
      env: { ...process.env }
    });

    daemonProcess.on('error', (error) => {
      log(`❌ Failed to launch daemon: ${error.message}`, 'ERROR');
      process.exit(1);
    });

    daemonProcess.on('exit', (code, signal) => {
      if (code !== null && code !== 0) {
        log(`📝 Daemon exited with code ${code}`, 'WARN');
      } else if (signal) {
        log(`📝 Daemon killed by signal: ${signal}`, 'WARN');
      } else {
        log('✅ Daemon exited successfully', 'SUCCESS');
      }
      process.exit(code || 0);
    });

    log('✅ Daemon launched successfully', 'SUCCESS');
    log('📝 Check daemon.log for detailed logs', 'INFO');
    log('⌨️  Press Ctrl+C to stop\n', 'INFO');

  } catch (error) {
    log(`❌ Error launching daemon: ${error.message}`, 'ERROR');
    process.exit(1);
  }
}

/**
 * Main entry point
 */
function main() {
  log('🛡️  Fluxion Daemon Launcher', 'INFO');
  log('🌐 Platform: ' + platform(), 'INFO');

  const rustArgs = parseArgs();
  launchRustDaemon(rustArgs);
}

// Start the launcher
main();
