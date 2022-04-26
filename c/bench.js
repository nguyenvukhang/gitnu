#!/usr/bin/env node

const use = 'CPP'
const fs = require('fs')
const { exec } = require('child_process')
const process = require('process')

const executables = {
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
timed(executables[use])
const endTime = performance.now()
const result = endTime - startTime
console.debug('Elapsed time:', result)
process.chdir(original)

const d = new Date()
const D = d.toLocaleDateString()
const T = d.toLocaleTimeString()

const data = JSON.parse(fs.readFileSync('time.json'))

exec('git rev-parse --verify HEAD', (_, commitId) => {
  const packet = {
    use,
    result,
    timestamp: `${D} ${T}`,
    commit: commitId.trimEnd(),
  }
  data.push(packet)
  fs.writeFile('time.json', JSON.stringify(data), (err) => {
    if (err) {
      console.error(err)
    }
  })
})
