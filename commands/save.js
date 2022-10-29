const conf = new (require("conf"))();
const chalk = require("chalk");
const prompt = require("prompt");
const getToken = require("./getToken");
const axios = require("axios");

prompt.message = "";
prompt.delimiter = chalk.green.bold(":");
prompt.start();

async function save() {
    let token = getToken();

    if (!token) {
        // console.log(chalk.red.bold("Not logged in --> run bitfuel login."));
        return;
    }

    var schema = {
        properties: {
            command: {
                description: "Enter the command to save"
            },
            description: {
                description: "Enter the description of the command to save"
            }
        }
    };
    //prompt.get([{"token": "Your token"}], function (err, result) {
    prompt.get(schema, async function (err, result) {
        if (err) {
            // || //we could check an api route here
            process.stdout.write("\n");
            return;
        } else {
            var reqResult;

            try {
                await axios({
                    method: "get",
                    url:
                        "https://bitfuel.dev/api/save?token=" +
                        token +
                        "&command=" +
                        result.command +
                        "&descript=" +
                        result.description +
                        "&codetype=command"
                });
            } catch (e) {
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

            console.log("Command saved successfully.");
        }
    });
}

module.exports = save;
