const fs = require("fs");
const chalk = require("chalk");
const homedir = require("os").homedir();
const TOKEN_FULL_PATH = homedir + "/.config/bitfuel/key.txt";

const getToken = () => {
    var cachedToken;
    if (cachedToken) return cachedToken;
    var token;
    try {
        token = fs.readFileSync(TOKEN_FULL_PATH, "utf8");
    } catch (err) {
        // console.log(err);
    }

    if (!token || !token.length) {
        console.log(chalk.red.bold("Token Not Found, run 'bitfuel login'"));
        return false;
    }
    cachedToken = token;
    return token;
};

module.exports = getToken;
