import {default as $decode} from '@3-/proto/decode.js'
import {u64 as $u64,u32 as $u32,string as $string} from '@3-/proto/decode/types.js'
import BIN1 from "@3-/proto/decode/BIN1.js"

export const State /*
  0 preChange	u64
  1 runed    	u64
  2 costSum  	u64
  3 avg10    	u32
  4 tagLi    	[string]
*/ = $decode(
  [$u64,$u64,$u64,$u32,$string],
  [0,0,0,0,1]
)

export const Err /*
  0 err  	string
  1 state	State
*/ = $decode(
  [$string,State],
  ["",State(BIN1)]
)

export const Site /*
  0 host 	string
  1 okLi 	[State]
  2 errLi	[Err]
*/ = $decode(
  [$string,State,Err],
  ["",1,1]
)

export const Kind /*
  0 name  	string
  1 siteLi	[Site]
*/ = $decode(
  [$string,Site],
  ["",1]
)

export const Li /*
  0 secSincePreCheck	u32
  1 kindLi          	[Kind]
*/ = $decode(
  [$u32,Kind],
  [0,1]
)