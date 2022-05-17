# BitFuel

*Never look up a command twice*

## Manifesto

If you understand the essence of a bash command to the extent that you can discuss it in plain english with others, you should not have to search for it again on the web. When you do, your focus is broken and you've been pulled away from the task at hand and into the netherworld of SEO.

BitFuel is a CLI tool that allows you to store bash commands in the cloud where you can retrieve them in plain english, wherever you might be coding.

BitFuel is not autocomplete - we built it on top of strong natural language understanding routines that learn your commands as they are unique to you and the way you describe things. Autocomplete engines suffer from learning the patterns of the masses on the web in a web crawl - the same problem you have when you search the web for a post articulating the command you once remembered.

BitFuel is not an alias - it matches your descriptions intelligently, so you don't need to remember things exactly.

## Installation

Linux:

```
sudo apt install bitfuel
```

Mac:

```
brew install bitfuel
```

## Usage

`bitfuel login` - visit your profile page on our website and retrieve a token unique to your account.

`bitfuel save` - save your command with a description of what the command means to you.

`bitfuel get` - provide a description of the command you are looking for and retrieve it

`bitfuel run` - provide a description of a command and automatically run it - use with caution.

