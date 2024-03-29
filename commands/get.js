const conf = new (require("conf"))();
const chalk = require("chalk");
const prompt = require("prompt");
const axios = require("axios");
const getToken = require("./getToken");
var stdin = process.stdin;
var keypress = require("keypress");
const { exec } = require("child_process");

prompt.message = "";
prompt.delimiter = chalk.green.bold(":");
prompt.start();

async function get() {
    let token = getToken();

    if (!token || !token.length) {
        // console.log(chalk.red.bold("Not logged in --> run bitfuel login."));
        return;
    }

    var schema = {
        properties: {
            description: {
                description: "Enter the description of the command to get"
            }
        }
    };

    prompt.get(schema, async function (err, result) {
        if (err) {
            // || // could check an api route here
            console.log("There was an error fetching your description.");
        } else {
            var reqResult;
            try {
                reqResult = await axios({
                    method: "get",
                    url:
                        "https://bitfuel.dev/api/get?token=" +
                        token +
                        "&prompt=" +
                        result.description +
                        "&size=20" +
                        "&page=1" +
                        "&codetype=command"
                });
            } catch (e) {
                console.log(e.response.status, e.response.data.error);
                if (e.response.status == 400) {
                    console.log(
                        "Request failed because no token was sent. Did you run 'bitfuel login?'"
                    );
                    return;
                } else if (e.response.status == 401) {
                    console.log(
                        "Token was invalid. Did you delete this token? Generate a new one at https://bitfuel.com and run 'bitfuel login' to fix."
                    );
                    return;
                } else {
                    console.log(e.response.data.error);
                    return;
                }
            }
            commands = reqResult.data.result;

            console.log("Use arrow keys to cycle commands -> press enter to run.");

            var command_position = 0;
            process.stdout.write(commands[command_position].command);

            stdin.setRawMode(true);
            stdin.resume();
            stdin.setEncoding("utf8");
            stdin.on("keypress", function (ch, key) {
                if (key && key.name == "down") {
                    if (command_position > 0) {
                        command_position--;
                        process.stdout.clearLine(0);
                        process.stdout.cursorTo(0);
                        process.stdout.write(commands[command_position].command);
                    }
                }

                if (key && key.name == "up") {
                    if (command_position < commands.length - 1) {
                        command_position++;
                        process.stdout.clearLine(0);
                        process.stdout.cursorTo(0);
                        process.stdout.write(commands[command_position].command);
                    }
                }

                if ((key && key.name == "return") || (key && key.name == "c" && key.ctrl)) {
                    process.stdout.write("\n");
                    exec(commands[command_position].command, (error, stdout, stderr) => {
                        if (error) {
                            console.log(error.message);
                            return;
                        }
                        if (stderr) {
                            console.log(stderr);
                            return;
                        }
                        console.log(stdout);
                    });
                    process.stdin.pause();
                }
            });
        }
    });
}

module.exports = get;
