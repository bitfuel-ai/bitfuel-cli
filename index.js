#! /usr/bin/env node

//thx logrocket https://blog.logrocket.com/creating-a-cli-tool-with-node-js/
const { program } = require('commander');
const login = require('./commands/login');
const save = require('./commands/save.js');
const get = require('./commands/get.js');



program
    .command('login')
    .description('Login to your BitFuel account.')
    .action(login)

program
    .command('save')
    .description('Save a command to bitfuel.')
    .action(save)

program
    .command('get')
    .description('Get a command from bitfuel.')
    .action(get)

program.parse();

