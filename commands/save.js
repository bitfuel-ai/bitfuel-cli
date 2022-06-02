const conf = new (require('conf'))()
const chalk = require('chalk');
const prompt = require('prompt');
const axios = require('axios');

prompt.start();

async function save () {
    let token = conf.get('token');

    if (!token) {
        console.log(
            chalk.red.bold('Not logged in --> run bitfuel login.')
        ); 
        return;
    }

    var schema = {
        properties: {
            command: {
                description: 'Enter the command to save: '
             },
           description: {
              description: 'Enter the description of the command to save: '
           }
        }
     };
    //prompt.get([{"token": "Your token"}], function (err, result) {
    prompt.get(schema, async function (err, result) {
        
        if (err) { // || //we could check an api route here
            console.log(
                chalk.green.bold('There was an error fetching your description.')
            );
        } else {

            var reqResult = await axios({
                method: "get",
                url: "https://bitfuel.dev/api/save?token=" + token + "&command=" + result.command + "&descript=" + result.description 
            });
            
            if (reqResult.status == 200) {
                console.log(
                    chalk.green.bold('Command saved successfully.')
                );
            }
            else {
                console.log(
                    chalk.red.bold('Command save failed.')
                );
            }
        }

      });
}

module.exports = save;