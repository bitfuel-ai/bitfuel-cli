const conf = new (require('conf'))()
const chalk = require('chalk');
const prompt = require('prompt');
const axios = require('axios');

prompt.message = '';
prompt.delimiter = chalk.green.bold(':');
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
                description: chalk.green.bold('Enter the command to save')
             },
           description: {
              description: chalk.green.bold('Enter the description of the command to save')
           }
        }
     };
    //prompt.get([{"token": "Your token"}], function (err, result) {
    prompt.get(schema, async function (err, result) {
        
        if (err) { // || //we could check an api route here
            process.stdout.write("\n");
            return;
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