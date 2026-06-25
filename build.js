#!/usr/bin/env node

import { execSync } from 'node:child_process';
import { copyFileSync, mkdirSync, existsSync, chmodSync } from 'node:fs';
import { join, dirname } from 'node:path';

const DIST_DIR = join(import.meta.dirname, 'dist');

console.log('🚀 Building fluxion-daemon...');

try {
  // Step 1: Build Rust binary
  console.log('📦 Building Rust binary...');
  execSync('cargo build --release', { cwd: PROJECT_ROOT, stdio: 'inherit' });

  // Step 2: Create dist directory
  console.log('📁 Creating dist directory...');
  if (!existsSync(DIST_DIR)) {
    mkdirSync(DIST_DIR, { recursive: true });
  }

  // Step 3: Copy Rust binary to dist
  console.log('📋 Copying Rust binary to dist...');
  const rustBinary = join(PROJECT_ROOT, 'target/release/fluxion-daemon');
  const distBinary = join(DIST_DIR, 'fluxion-daemon');

  copyFileSync(rustBinary, distBinary);
  chmodSync(distBinary, 0o755); // Make executable

  // Step 4: Copy cli directory to dist
  console.log('📋 Copying cli files to dist...');
  const cliSource = join(PROJECT_ROOT, 'cli/index.js');
  const cliDest = join(DIST_DIR, 'index.js');

  copyFileSync(cliSource, cliDest);

  console.log('✅ Build completed successfully!');
  console.log(`📦 Output: ${DIST_DIR}`);
  console.log(`🔧 Binary: ${distBinary}`);
  console.log(`📝 CLI: ${cliDest}`);
} catch (error) {
  console.error('❌ Build failed:', error.message);
  process.exit(1);
}
