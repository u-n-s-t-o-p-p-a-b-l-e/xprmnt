#!/usr/bin/env node 

function generatePassword (length: number): string {
	const charset = 
'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+';
	let password = '';
	for (let i = 0; i < length; i++) {
		const randomIndex = Math.floor(Math.random() * charset.length);
		password += charset[randomIndex];
	}
	return password;
}

function main(): void {
	const length = parseInt(process.argv[2]) || 12;
	const password = generatePassword(length);
	console.log(`Generated Password: ${password}`);
}

main();
