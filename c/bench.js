#!/usr/bin/env node

const use = "CPP"

const fs = require('fs')
const { exec } = require('child_process')
const process = require('process')


const commands = {
  CPP: '../../dist/cgit',
  PYTHON: 'gitnu',
  GIT: 'git status',
}

function timed(command) {
  for (let i = 0; i < 1000; i++) {
    exec(command)
  }
}

const original = process.cwd()
process.chdir('repo/some/thing')
const startTime = performance.now()
timed(commands[use])
const endTime = performance.now()
const result = endTime - startTime
console.debug('Elapsed time:', result)
process.chdir(original)

const content = use + ' : ' + result.toString() + '\n'

fs.appendFile('time.txt', content, (err) => {
  if (err) {
    console.error(err)
  }
})
