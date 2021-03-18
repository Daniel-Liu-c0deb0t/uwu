# uwu
fastest text uwu-ifier in the west

## faq
### what?
u want ur large corpus of text uwu'd in a smol amount of time

### where?
computers with recent x86 cpus (intel, amd) that support sse

### why?
you *don't* want cute text?

### how?
tldr: 128-bit simd vectorization plus some big brain algos

after hours of research, i've finally understood the essence of uwu'd text

there are 4 transformations:
1. replace `l` and `r` with `w`
2. sometimes stutter (`hi` -> `h-hi`)
3. sometimes replace `,` or `.` with a text emoji
4. replace `small` with `smol`, `cute` with `cuuuuute`, `fluff` with `floof`

these transformations can be pretty easily applied as multiple passes that take advantage of
sse vector intrinsics to process 16 bytes at once

utf-8 is handled elegantly by simply not changing non-ascii characters

uwu-ification is done with multiple threads after chunks of a file are read in

the goal is to be this universe's fastest text uwu-ifier

### why isn't this readme uwu'd?
so its readable

if u happen to find uwu'd text more readable, there's always an [uwu'd]() version

### ok i want uwu'd text, how do i run this myself?
1. clone this repo

### ok but why aren't there any of the settings i can change?!1?!!1
free will is an illusion

### ok but i have an issue with this or a suggestion
open an issue, be nice

### ok i need to use this for something and i need the license info
mit license, but idk why u would need this for anything

### ok but mit rejecc me `-1i32 as u32` yrs ago and im still sad
??? go uwu some more text
