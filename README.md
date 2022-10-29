# :fire: BitFuel CLI :fire:

_Never look up a command twice_

bitfuel.dev

## Manifesto

BitFuel is a CLI tool that allows you to store bash commands in the cloud where you can retrieve them in plain english, wherever you might be coding.

BitFuel is not autocomplete - we built it on top of strong natural language understanding routines that learn your commands as they are unique to you and the way you describe things. Autocomplete engines suffer from learning the patterns of the masses on the web in a web crawl - the same problem you have when you search the web for a post articulating the command you once remembered.

BitFuel is not an alias - it matches your descriptions intelligently, so you don't need to remember things exactly.

## Installation

```
npm install bitfuel
```

## Usage

`bitfuel login` - visit your settings page on our website to generate a token unique to your account.

`bitfuel save` - save your command with a description of what the command means to you.

`bitfuel get` - provide a description of the command you are looking for and retrieve it

## VSCODE extension

We also provide a VSCode extension to save and retrieve your code snippets with natural language without leaving your text editor.

https://github.com/bitfuel-ai/bitfuel-vscode
