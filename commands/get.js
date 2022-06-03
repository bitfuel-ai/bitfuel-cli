const conf = new (require('conf'))()
const chalk = require('chalk');
const prompt = require('prompt');
const axios = require('axios');
var stdin = process.stdin;
var keypress = require('keypress');

prompt.message = '';
prompt.delimiter = chalk.green.bold(':');
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
              description: chalk.green.bold('Enter the description of the command to get')
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
                url: "https://bitfuel.dev/api/get?token=" + token + "&prompt=" + result.description + "&size=20" + "&page=1" 
            });
            
            if (reqResult.status == 200) {

                commands = reqResult.data.result;

                console.log(
                    chalk.green.bold('Use arrow keys to cycle commands - press enter to finish.')
                )

                var command_position = 0;
                process.stdout.write(chalk.green.bold(commands[command_position].command));

                stdin.setRawMode( true );
                stdin.resume();
                stdin.setEncoding( 'utf8' );
                stdin.on('keypress', function (ch, key) {
                    if (key && key.name == 'left') {
                        if (command_position > 0) {
                            command_position--;
                            process.stdout.clearLine(0);
                            process.stdout.cursorTo(0);
                            process.stdout.write(chalk.green.bold(commands[command_position].command));
                        }
                        
                    }

                    if (key && key.name == 'right') {
                        if (command_position < (commands.length - 1)) {
                            command_position++;
                            process.stdout.clearLine(0);
                            process.stdout.cursorTo(0);
                            process.stdout.write(chalk.green.bold(commands[command_position].command));
                        }
                    }

                    if (key && key.name == 'return') {
                        process.stdout.write("\n");
                        process.stdin.pause();
                    }
                });
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