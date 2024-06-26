#!/usr/bin/env coffee

> ./s3.cdn:
  @3-/uridir
  fs > createReadStream
  fs/promises > rename writeFile readFile opendir unlink
  path > join dirname
  @3-/blake3 > blake3Hash
  base-x
  ./mime
  knex
  ./env > DIST ROOT PWD
  @3-/ossput:put

BFILE = BaseX '!$-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz'

encode = (n)=>
  bin = Buffer.allocUnsafe 6
  bin.writeUIntBE(n,0,6)
  for i,pos in bin
    if i!=0
      break
  BFILE.encode bin[pos..]

css_js = new Map()
to_replace = []

IGNORE = new Set()

PUBLIC = join(
  ROOT
  'public'
)

for await fp from await opendir PUBLIC
  if not fp.isFile()
    continue
  IGNORE.add fp.name

all = new Set()
for await fp from await opendir DIST
  if not fp.isFile()
    continue
  fp = fp.name
  if IGNORE.has fp
    continue
  all.add fp
  name = fp.split('.')
  ext = name.at -1
  hex = name.at -2
  if ['htm','html','css','js'].includes(ext)
    css_js.set(
      fp
      await readFile(join(DIST,fp),'utf8')
    )
  if not ['s.js','index.html','index.htm'].includes(fp)
    to_replace.push fp


DB = knex {
  client:  'sqlite3'
  useNullAsDefault: true
  connection: {
    filename: join PWD, 'filename_min.db'
  }
}

table = 'id_hash'
if not await DB.schema.hasTable table
  await DB.schema.createTable(
    table
    (table) =>
      table.integer('id').primary()
      table.boolean('uploaded').defaultTo(false)
      table.binary('val').notNullable().unique()
      return
  )

to_replace.sort()

ID = []

for i from to_replace
  fp = join DIST, i
  bin = await readFile fp
  val = Buffer.from blake3Hash bin
  id = (await DB(table).where({val}))[0]?.id or 0
  if not id
    [id] = await DB(table).insert({val})
    key = encode id
    if [
      'I18N'
      'v'
    ].includes(key)  or key.replaceAll('.','')==''
      await DB(table).where({id}).delete()
      ++id
      await DB(table).insert({id,val})

  ID.push id

for [k,v] from css_js.entries()
  for fp,n in to_replace
    v = v.replaceAll(
      fp
      encode(ID[n])
    )
  await writeFile(
    join(DIST,k)
    v
  )

for i,p in to_replace
  fp = join DIST, i
  id = ID[p]
  key = encode(id)
  {uploaded} = t = (await DB(table).where({id}).select())[0]
  if uploaded
    await unlink fp
    continue
  await put(
    key
    =>
      createReadStream fp
    mime i
  )
  await DB(table).where({id}).update({uploaded:true})
  await unlink fp

process.exit()

