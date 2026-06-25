import os from 'node:os';

const platform = os.platform();
switch (platform) {
  case 'linux':
    console.log('Linux');
    break;
  default:
    console.log('Not Supported Yet: ' + platform);
    break;
}
