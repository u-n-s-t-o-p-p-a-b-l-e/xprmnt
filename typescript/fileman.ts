#!/use/bin/env node 

import * as fs from 'fs';
import * as path from 'path';

function copyFile (source: string, destination: string): void {
	fs.copyFileSync( source, destination );
	console.log(`File copied from ${source} to ${destination}`);
}


