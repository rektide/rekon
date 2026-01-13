#!/usr/bin/env node
import { cli, define } from 'gunshi'
import combineCommand from './command/combine.ts'
import installCommandsCommand from './command/install-commands.ts'

const mainCommand = define({
	name: 'rekon',
	description: 'Rekon CLI tool',
	run: () => {
		console.log('Available commands: combine, install-commands')
		console.log('Run "rekon --help" for more information')
	}
})

await cli(process.argv.slice(2), mainCommand, {
	name: 'rekon',
	version: '1.0.0',
	description: 'Rekon CLI tool',
	subCommands: {
		combine: combineCommand,
		'install-commands': installCommandsCommand
	}
})
