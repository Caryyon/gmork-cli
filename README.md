# Gmork Cli

A cli tool to gather many commands and configs into a simple plugin based arch.
Perfect for teams who all use similar tools and what to share scripts and machine setups.

## Install

```bash
cargo install gmork-cli
```

## Command Concepts

The basis for this project is to give someone the ability to define the required
scripts, software, permissions and so on for a group/organizations workflow.
This will likely leverage things like `Brewfile`s and `package.json`s.
Most importantly it should be extremely easy to write and configure a plugin
to execute the commands defined.

## Plugins

`THIS IS ALPHA SOFTWARE THE API COULD CHANGE AT ANY TIME`

A plugin should boil down the required steps to getting one specific thing setup
and configured on the executing partners machine.

Example configuration file
```json
[
  { 
    announcement: 'Installing Homebrew',
    command: '/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
    successRequired: true
  },
  {
    announcment: 'Installing Node.js',
    command: 'brew install node',
    successRequired: true
  }
]
```

In the above example you can see that an array of objects will be the steps of the installation process.
Each object containing a set group of keys that have meaningful values. In the above example we see the 
config object offering the announcement of 'Installing Homebrew' this makes it easy for configuring users
to write in common language to express what is happeneing. Then we see the command followed by the bash
command for curl'ing the install for homebrew. The last key defines if the process must succeed to move
on to the next process. This would be used for something like `vtop` where you would need `npm` installed
prior to installing.

## Usage

To the new or end user we want the experience to be as painless as possible.

```bash
gmork --init <config file location | repo url>
```
In the above code the user doesn't have to know much. That's the point! They are new they aren't going 
to know much by default. This command allows you to give them a config file or repo url that holds all the 
up to date configurations of the team. It's essentially set it and forget it.



