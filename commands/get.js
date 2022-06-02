const conf = new (require('conf'))()
const chalk = require('chalk');
const prompt = require('prompt');
const axios = require('axios');

prompt.start();

async function get () {
    let token = conf.get('token');

    if (!token) {
        console.log(
            chalk.red.bold('Not logged in --> run bitfuel login.')
        ); 
        return;
    }

    var schema = {
        properties: {
           description: {
              description: 'Enter the description of the command to get: '
           }
        }
     };
    //prompt.get([{"token": "Your token"}], function (err, result) {
    prompt.get(schema, async function (err, result) {
        
        if (err) { // || //we could check an api route here
            console.log(
                chalk.red.bold('There was an error fetching your description.')
            );
        } else {

            var reqResult = await axios({
                method: "get",
                url: "https://bitfuel.dev/api/get?token=" + token + "&prompt=" + result.description 
            });
            
            if (reqResult.status == 200) {
                console.log(
                    chalk.green.bold(reqResult.data.result[0].command)
                );
            }
            else {
                console.log(
                    chalk.red.bold('Get command failed.')
                );
            }
        }

      });
}

module.exports = get;