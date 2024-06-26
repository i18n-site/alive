#!/usr/bin/env coffee

> zx/globals:
  fs > existsSync readFileSync writeFileSync copyFileSync
  path > join
  fs > rmSync
  @3-/mdt/make.js

cwd = process.cwd()

fp = (p)=>
  join cwd,p

package_json = 'package.json'

package_json_fp = fp package_json
json = JSON.parse readFileSync(
  package_json_fp
  'utf8'
)

{version} = json

version = version.split('.')
version.push parseInt(version.pop())+1
json.version = version = version.join '.'

writeFileSync(
  package_json_fp
  JSON.stringify json,null,2
)


await $'git add -u'
await $"git commit -m '#{json.name} v#{version}'"
await $'npm publish --access=public'
