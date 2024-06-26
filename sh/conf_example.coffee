#!/usr/bin/env coffee

> path > dirname join
  fs > realpathSync
  @3-/walk > walkRel
  @3-/read
  @3-/write

ROOT = dirname import.meta.dirname
CONF = realpathSync join(
  ROOT
  'conf'
)

do =>
  for kind from ['alter','watch']
    dir = join CONF, kind
    for await fp from walkRel dir
      txt = read join dir, fp

      txt = txt.replace(
        /([-\w]+).fly.dev/g
        (_, prefix)=>
          prefix.replace(
            /\w/g
            (i)=>
              i = i.charCodeAt(0)
              ( ((i<<1)^i) % 26).toString(36)
          )+'.fly.dev'
      )

      is_sh = fp.endsWith '.sh'
      if is_sh
        split = '='
      else
        split = ':'

      li = []
      for i from txt.split('\n')
        p = i.indexOf(split)
        p2 = i.indexOf('://')
        if p2 > p
          p = p2
        if p > 0
          p+=1

          more = i.slice(p)
          if not ( is_sh and more.trimStart().startsWith('$') )

            i = i.slice(0,p) + more.replace(/[a-z]/g, 'x').replace(/[A-Z]/g,'X').replace(
              /\d\d/g,
              (i)=>
                ((i>>3)^(i<<3))%25
            ).replace(
              /\d\b/g,
              (i)=>
                ((i>>2)^(i<<2))%10
            )

        li.push i
        txt = li.join '\n'

      if fp.includes '/'
        project = dirname fp
      else
        p = fp.lastIndexOf('.')
        project = fp.slice(0,p)
      to = join(
        kind
        project
        'conf'
        fp
      )
      console.log to
      write(
        join ROOT, to
        txt
      )
  return

