const conf = new (require("conf"))();
const chalk = require("chalk");
const getToken = require("./getToken");
const prompt = require("prompt");
const fs = require("fs");
const homedir = require("os").homedir();
const CONFIG_PATH = homedir + "/.config";
const BITFUEL_PATH = homedir + "/.config/bitfuel";
const TOKEN_FULL_PATH = homedir + "/.config/bitfuel/key.txt";

prompt.message = "";
prompt.start();

function login() {
    console.log(chalk.green.bold("Please visit bitfuel.dev and paste your api token here."));

    var schema = {
        properties: {
            token: {
                description: chalk.green.bold("Enter your token"),
                hidden: true,
                replace: "*"
            }
        }
    };

    //prompt.get([{"token": "Your token"}], function (err, result) {
    prompt.get(schema, function (err, result) {
        if (err || !result.token || !result.token.length > 0) {
            // || //we could check an api route here
            console.log(
                chalk.red.bold("There was an error logging in. Be sure you provided a token")
            );
        } else {
            try {
                if (!fs.existsSync(CONFIG_PATH)) {
                    fs.mkdirSync(CONFIG_PATH);
                }

                if (!fs.existsSync(BITFUEL_PATH)) {
                    fs.mkdirSync(BITFUEL_PATH);
                }

                fs.writeFileSync(TOKEN_FULL_PATH, result.token);
                console.log(chalk.green.bold("Login successful."));
            } catch (err) {
                console.log(
                    chalk.red.bold("There was an error logging in. Could not write token to file.")
                );
                console.log(err);
            }
        }
    });
}

module.exports = login;
