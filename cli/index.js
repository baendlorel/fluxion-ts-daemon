import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import os from 'node:os';

const __dirname = dirname(fileURLToPath(import.meta.url));
const platform = os.platform();

switch (platform) {
  case 'linux':
    console.log('🚀 Starting fluxion-daemon on Linux...');

    // Use the binary from the same directory
    const binaryPath = join(__dirname, 'fluxion-daemon');

    const daemon = spawn(binaryPath, ['--command', 'tsx', '--args', 'main.ts'], {
      detached: true,
      stdio: 'ignore',
    });

    daemon.unref();
    console.log('✅ Daemon started successfully');
    break;

  default:
    console.log('❌ Platform not supported yet:', platform);
    process.exit(1);
}
