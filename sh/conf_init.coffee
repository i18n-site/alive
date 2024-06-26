#!/usr/bin/env coffee

> @3-/yml/Yml.js
  path > join dirname
  fs > existsSync copyFileSync mkdirSync
  @3-/walk > walkRel

ROOT = dirname import.meta.dirname

CONF = join ROOT, 'conf'

do =>
  for [kind, li] from Object.entries Yml(CONF).plugin
    for plugin from li
      conf = join(
        ROOT,kind,plugin,'conf'
      )
      if not existsSync conf
        continue
      for await i from walkRel conf
        to_fp = join(
          ROOT
          'conf'
          kind
          i
        )
        if not existsSync to_fp
          to_dir = dirname to_fp
          if not existsSync to_dir
            mkdirSync to_dir, {recursive : true}
          copyFileSync join(conf,i), to_fp

  return
