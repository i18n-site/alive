#!/usr/bin/env coffee

> @3-/yml/Yml.js
  @3-/read
  @3-/write
  path > join normalize dirname
  @iarna/toml

CARGO_TOML = 'Cargo.toml'

ROOT = dirname import.meta.dirname

alive_plugin_dir = join ROOT, 'alive_plugin'
plugin_toml = toml.parse read(join alive_plugin_dir,'_'+CARGO_TOML)

enable = []
load = (type, li)=>
  plugin_dir = join ROOT, type
  for i in li
    t = toml.parse read join plugin_dir,i,CARGO_TOML
    name = t.package.name
    console.log name
    plugin_toml.dependencies[name] = { path : normalize("../#{type}/#{i}"), optional : true }
    enable.push("dep:"+name)

  write(
    join(alive_plugin_dir,"src/#{type}.rs")
    "crate::#{type}!(#{li.join(',')});"
  )
  return

conf = Yml(join ROOT, 'conf').plugin
for [k,li] in Object.entries(conf)
  console.log '# '+k
  load k,li

plugin_toml.features.enable = enable
write(
  join(alive_plugin_dir,CARGO_TOML)
  toml.stringify(plugin_toml)
)

