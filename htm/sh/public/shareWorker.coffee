# > ~/conf.js > API
#   @2-/bc/toAll.js
#
# WSURL = 'ws'+(
#   if location.protocol.charAt(4) == 's' then 's:' else ':'
# )+API+'ws/'
#
# + WS,USER
#
# conn = =>
#   user = USER
#   if not user
#     WS?.close()
#     return
#   WS = new WebSocket WSURL + user.slice(0,2).map((i)=>i.toString(36)).join('/')
#   Object.assign(
#     WS
#     binaryType: 'arraybuffer'
#     onmessage:({data})=>
#       # if data instanceof ArrayBuffer
#         # data = new Uint8Array(data)
#       console.log 'data',[data]
#       p = data.indexOf '['
#       n = parseInt data.slice(0,p),36
#       switch n
#         when 0 # USER VER UPDATE
#           USER = [
#             user[0]
#             ...JSON.parse(
#               data.slice(p)+']'
#             )
#           ]
#           toAll(
#             n
#             USER
#           )
#       return
#     onclose: =>
#       if user
#         setTimeout(conn,1e3)
#       return
#     # onopen:=>
#     #   console.log 'open ws'
#     #   return
#   )
#   return
#
# `onconnect=//`
# (e) =>
#   [port] = e.ports
#
#   port.onmessage = ({data}) =>
#     [
#       (user)=> # user websocket init
#         u = USER
#         USER = user
#         if user and u
#           if u[0] == user[0]
#             return
#           WS.close()
#         conn()
#         return
#     ][data.shift()](...data)
#     return
#   return
