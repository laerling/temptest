+ call main
[

 1 main [->+>+<<]>[-<+>] +>-[[-]<->]< [-
  set return to exit <[-]>
  call printfac +++++>
 ]<

 5 printfac [->+>+<<]>[-<+>] +>-----[[-]<->]< [-
  {?} _0_
  >+++++ number to factorize (TODO: User input)
  Set as argument for printing and function call [->>>>+<<<<<+>]
  {?} {arg} _0_ 0 0 0 {arg}
  print stuff +++[->++++[->+++++++>+++<<]<]>>.[----<+++++>]<-.---.>>----.<<
              "The "  {?} {arg} 0 _101_ 0 32 {arg}
	      +.-----.++.+++++++++++++++++.-----.+++.---------.--------.+++++++++++.>>.<<
              "factorial "  {?} {arg} 0 _108_ 0 32 {arg}
	      +++.---------.>>.>
	      "of "  {?} {arg} 0 102 0 32 _{arg}_
	      print arg >>++++++++++<<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>>++++++++++<[->-[>+>>]>[+[-<+>]>+>>]<<<<<]>[-]>>[>++++++[-<++++++++>]<.<<+>+>[-]]<[<[->-<]++++++[->++++++++<]>.[-]]<<++++++[-<++++++++>]<.[-]<<[-]<
	                {?} {arg} 0 102 0 32 _0_
              <.<<+++.++++++++++.>>.
              " is "  {?} {arg} 0 115 0 _32_
	      [-]<<[-]<<
  {?} _{arg}_
  set return address to printfac1 <[-]++++>
  {printfac1} _{arg}_
  call fac >++>
  {printfac1} {arg} {fac} _0_
 ]<

 4 printfac1 [->+>+<<]>[-<+>] +>----[[-]<->]< [-
  {return address} {fac result} {?} _0_
  print result <[-]>++++++++++<<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>>++++++++++<[->-[>+>>]>[+[-<+>]>+>>]<<<<<]>[-]>>[>++++++[-<++++++++>]<.<<+>+>[-]]<[<[->-<]++++++[->++++++++<]>.[-]]<<++++++[-<++++++++>]<.[-]<<[-]
  {return address} 0 _0_
  ".\n" +++++[-<+++++++++>]<+.--[---->+<]>-.[-]<
  {return address} _0_
 ]<

 2 fac [->+>+<<]>[-<+>] +>--[[-]<->]< [-
  {arg} {?} _0_
  +<<[
   - we can already decrement arg at this point
   _{arg dec}_ {?} x
   [->>>+>>+<<<<<]>>>[-<<<+>>>]
   {arg dec} {?} x _0_ 0 {arg dec}
   <[->+>+<<]>[-<+>]
   {arg dec} {?} x _0_ x {arg dec}
   >>[-<[-<+<+>>]<[->+<]>>]<[-]<<<<
   _{arg dec}_ {?} y
  ]>>
  0 {?} _{result}_
  set return address <[-]<<[->+<]
  _0_ {ret addr} 0 {result}
  set result as argument >>>[-<<<+>>>]<
  {result} {ret addr} _0_
 ]<

 3 print [->+>+<<]>[-<+>] +>---[[-]<->]< [-
  print arg <[-]>++++++++++<<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>>++++++++++<[->-[>+>>]>[+[-<+>]>+>>]<<<<<]>[-]>>[>++++++[-<++++++++>]<.<<+>+>[-]]<[<[->-<]++++++[->++++++++<]>.[-]]<<++++++[-<++++++++>]<.[-]<<[-<+>]<[-]
  exit [-]>
 ]<

]