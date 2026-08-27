import { copyFile, mkdir } from 'node:fs/promises';
import { resolve } from 'node:path';

const extension = process.platform === 'win32' ? '.exe' : '';
const source = resolve(`target/release/awb${extension}`);
const directory = resolve('dist/bin');
await mkdir(directory, { recursive: true });
await copyFile(source, resolve(directory, `awb${extension}`));
console.log(`Copied release binary to dist/bin/awb${extension}`);
