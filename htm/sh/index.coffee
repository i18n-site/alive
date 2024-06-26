do =>
  CDN=''
  script = 'script'
  s = document.createElement(script)
  s.type = 'module'
  s.src = "#{CDN}#{await (await fetch(CDN+'v')).text()}"
  m=document.getElementsByTagName(script)[0]
  m.parentNode.insertBefore(s, m)
  return
