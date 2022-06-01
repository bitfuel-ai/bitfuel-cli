#! /usr/bin/env node

//thx logrocket https://blog.logrocket.com/creating-a-cli-tool-with-node-js/
const { program } = require('commander');
const login = require('./commands/login');

program
    .command('login')
    .description('Login to your BitFuel account.')
    .action(login)

program.parse();

