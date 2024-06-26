#!/usr/bin/env coffee

> ./env > PUBLIC SRC
  path > join basename
  @3-/walk:@ > walkRel
  fs > createReadStream readdirSync readFileSync existsSync
  radix-64:Radia64
  @3-/blake3 > Blake3
  @3-/zipint > zip
  @3-/read
  @3-/write
  @3-/pool > Pool
  @3-/ossput:put

{encodeInt,decodeToInt} = Radia64()

i18n_dir = join(PUBLIC,'.18')

upload = (ver, file_li)=>
  pool = Pool 50
  for i from file_li
    fp = join(i18n_dir, i)
    await pool(
      put
      ver+'/'+i
      =>
        createReadStream fp
      'text/css'
    )
  await pool.done
  return

js_dir = join SRC,'i18n'

var_js = join(js_dir,'var.js')
{ver,posId} = await import(var_js)
hasher = new Blake3
hasher.update zip posId

file_li = []
for await i from walkRel i18n_dir
  if basename(i).startsWith('.')
    continue
  hasher.update readFileSync(join(i18n_dir,i))
  file_li.push i

hash_fp = join(js_dir,'.hash')
hash = Buffer.from(hasher.finalize()).toString('base64')

+ do_write

if existsSync hash_fp
  if hash != read(hash_fp).trim()
    do_write = 1
    ver = encodeInt(decodeToInt(ver)+1)
else
  do_write = 1

if do_write
  await upload ver, file_li
  write(
    hash_fp
    hash
  )
  write(
    var_js
    """\
    export const ver = '#{ver}' // #{decodeToInt ver}
    export const posId = #{JSON.stringify(posId)}
    """
  )

console.log 'ver', decodeToInt(ver)
