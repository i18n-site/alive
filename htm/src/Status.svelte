<template lang="pug">
mixin info
  +if tag_li.length
    b.t
      +each tag_li as tag
        b {tag}
  b.d {duration}
+if ok_li
  Scroll
    main
      b.time 距离上次检查
        b {last}
        | 秒
      +each err_li as [kind, vps_li]
        +each vps_li as [vps, err_li]
          b.err { kind }
            b.v {vps}
            +each err_li as [err, [duration, runed, cost_sum, avg10, tag_li]]
              +info
              i(@click=tip)
                pre {err}
    +each ok_li as [kind, vps_li]
      h2 { kind }
      main
        +each vps_li as [vps, state_li]
          b.ok.v {vps}
            +each state_li as [duration, runed, cost_sum, avg10, tag_li]
              +info
  +else
    WAIT
//-   i
//-     i
//-       +if err.length
//-         +each err as [kind, hli]
//-           table.e
//-             tr
//-               th(colspan=2) { kind }
//-               th 出错次数
//-               th 持续分钟
//-             +each hli as [host, li]
//-               +each li as [dns_type, err, ts], p
//-                 tr
//-                   +if 0 == dns_type
//-                     td(colspan=2) {host}
//-                     +else
//-                       +if p == 0
//-                         td(rowspan:li.length) { host }
//-                       td.s IPV{dns_type}
//-                   td.s {err}
//-                   td.s {-ts}
//-       +if ok.length
//-         +each ok as [kind, hli]
//-           table
//-             tr
//-               th(colspan=2) { kind }
//-               th
//-                 b 下次检查
//-                 b 单位分钟
//-             +each hli as [host, li]
//-               +each li as [dns_type, err, ts], p
//-                 tr
//-                   +if 0 == dns_type
//-                     td(colspan=2) {host}
//-                     +else
//-                       +if p == 0
//-                         td(rowspan:li.length) { host }
//-                       td.s IPV{dns_type}
//-                   td.s {ts}
</template>

<script lang="coffee">
> @~3/wait:WAIT
  @5-/alive > Li
  @3-/time/readable.js
  @3-/on:On
  @~3/scroll:Scroll

+ last, err_li, ok_li

tip = ->
  s = 's'
  @classList.toggle s
  setTimeout =>
    unbind = On document,{
      click:=>
        @classList.remove s
        unbind()
        return
    }
    return
  return


# {log} = console

run = =>
  [
    last
    li
  ] = await Li()

  err_li = []
  ok_li = []

  for [watch, vps_li] from li
    t_ok = []
    t_err = []
    for [vps, _ok_li, _err_li] from vps_li
      if _ok_li.length
        t_ok.push [vps,_ok_li]
        _ok_li.forEach (i)=>
          i[0] = readable i[0]
          return
      if _err_li.length
        _err_li.forEach (i)=>
          i[1][0] = readable i[1][0]
          return
        t_err.push [vps,_err_li]

    [
      [ ok_li, t_ok ]
      [ err_li, t_err ]
    ].forEach(
      ([l, t])=>
        if t.length
          t.sort()
          l.push [watch, t]
        return
    )

  ok_li.sort()
  err_li.sort()
  return

onMount =>
  try
    await run()
  catch e
    last = 0
    console.error e
  finally
    timer = setInterval(
      =>
        if not ((last+=1) % 30)
          run()
        return
      1000
    )
  =>
    clearInterval timer
    return

</script>

<style lang="stylus">
h2
  color #060
  font-size 20px
  font-variation-settings 'wght' 900, 'BEVL' 90
  line-height 2em
  margin 0 0 0 36px
  position relative

  &:before
    background #060
    border-radius 16px
    box-shadow 0 0 3px inset #000
    content ''
    height 14px
    left -22px
    margin-top 1px
    position absolute
    width 6px

h2, main
  align-self flex-start
  display flex

main
  flex-wrap wrap
  font-family h
  margin 8px 6px
  max-width 100%

  &>b
    border 1px solid #999
    border-radius 16px
    box-shadow 0 0 6px inset #bbb
    color #666
    font-size 14px
    line-height 3
    margin 7px
    padding 0 14px
    text-wrap nowrap

    &>b
      margin-left 7px

    &.time
      font-family s

      &>b
        display inline-block
        margin 0
        min-width 1.5em
        text-align center

    &.err
      background-color #c00
      border-color #a00
      box-shadow 0 0 6px inset #f00
      color #fff
      padding-right 6px

      &>i
        background #fff
        border-radius 30px
        box-shadow 0 0 3px #600 inset
        cursor pointer
        display inline-block
        height 26px
        margin-bottom -8px
        margin-left 12px
        padding 0
        position relative
        width 26px

        &:before
          background var(--svgTip) 50% 50% / 16px no-repeat
          content ''
          display inline-block
          height 16px
          left 3px
          position absolute
          top 5px
          width 20px

        &:hover
          box-shadow 0 0 6px inset rgba(255, 0, 0, 0.8)

        &>pre
          background rgba(0, 0, 0, 0.8)
          border 1px solid rgba(0, 0, 0, 0.9)
          border-radius 6px
          display none
          font-size 12px
          font-style normal
          left -200px
          line-height 1.5
          margin-right -5px
          max-width 250px
          position absolute
          text-wrap balance
          top 16px
          word-break break-word
          z-index 9

        :global(&.s) > pre, &:hover > pre
          display block

        :global(&.s)
          background #ef0 !important

    &.ok
      background-color #060
      border-color #060
      box-shadow 0 0 6px inset #0c0
      color #fff

b
  &.v
    border-bottom 1px solid rgba(225, 225, 225, 0.5)
    position relative

  &>b.v
    padding-bottom 6px

  &.t > b
    &:first-child
      margin-left 0

    margin-left 3px

  &.t, &.d
    background rgba(0, 0, 0, 0.3)
    border-radius 6px
    color #fff
    display inline-block
    font-size 12px
    line-height 1.5
    padding 3px 6px
    position relative

  &.d
    background rgba(255, 255, 0, 0.2)
    color #ccc
    letter-spacing 1px
    padding 3px 6px 2px 22px
    user-select none

    &:before
      background var(--svgTime) 50% 50% / 12px no-repeat
      content ''
      filter invert(100%)
      height 13px
      left 6px
      margin-top -7px
      position absolute
      top 50%
      width 13px
</style>
