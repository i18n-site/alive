#!/usr/bin/env coffee

> path > dirname join

env = JSON.parse process.env.ENV

li = []
for [k,v] from Object.entries(env.n)
  if not [
    'PATH'
    'RUSTFLAGS'
  ].includes(k) and not k.startsWith 'DIRENV_'
    v = JSON.stringify(v)
    li.push "#{k}=#{v}"

console.log li.join('\n')

