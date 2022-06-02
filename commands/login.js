const conf = new (require('conf'))()
const chalk = require('chalk');
const prompt = require('prompt');

prompt.start();

function login () {
    let token = conf.get('token');

    console.log(
        chalk.green.bold('Please visit bitfuel.dev and paste your api token here.')
    );

    var schema = {
        properties: {
           token: {
              description: 'Enter your token',
              hidden: true
           }
        }
     };

    //prompt.get([{"token": "Your token"}], function (err, result) {
    prompt.get(schema, function (err, result) {
        
        if (err) { // || //we could check an api route here
            console.log(
                chalk.green.bold('There was an error logging in.')
            );
        } else {
            conf.set('token', result.token);
            console.log(
                chalk.green.bold('Login successful.')
            );
        }

      });
}

module.exports = login