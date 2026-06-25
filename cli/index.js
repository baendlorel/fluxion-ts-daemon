import { spawn } from 'node:child_process';
import os from 'node:os';

const platform = os.platform();
switch (platform) {
  case 'linux':
    console.log('Linux');
    spawn('fluxion-daemon', ['--command="tsx main.ts"'], {
      detached: true,
      stdio: 'ignore',
    });
    break;
  default:
    console.log('Not Supported Yet: ' + platform);
    break;
}
