#!/usr/bin/env node 
function generatePassword(length) {
    var charset = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+';
    var password = '';
    for (var i = 0; i < length; i++) {
        var randomIndex = Math.floor(Math.random() * charset.length);
        password += charset[randomIndex];
    }
    return password;
}
function main() {
    var length = parseInt(process.argv[2]) || 12;
    var password = generatePassword(length);
    console.log("Generated Password: ".concat(password));
}
main();
