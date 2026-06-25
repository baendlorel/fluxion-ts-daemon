import { spawn } from 'node:child_process';
import { dirname, join, resolve } from 'node:path';
import os from 'node:os';

const platform = os.platform();

switch (platform) {
  case 'linux':
    console.log('🚀 Starting fluxion daemon on Linux...');

    // Use the binary from the same directory
    const binaryPath = resolve('fluxion-daemon');

    spawn(binaryPath, ['--command', 'tsx', '--args', 'main.ts'], {
      detached: true,
      stdio: 'ignore',
    }).unref();

    console.log('✅ Daemon started successfully');
    break;

  default:
    console.log('❌ Platform not supported yet:', platform);
    process.exit(1);
}
