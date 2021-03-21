# uwu
fastest text uwu-ifier in the west

## faq
### what?
u want large amounts of text uwu'd in a smol amount of time

### where?
your computer, if it has a recent x86 cpu (intel, amd) that support sse4.1

### why?
you *don't* want cute text?

### how?
tldr: 128-bit simd vectorization plus some big brain algos

after hours of research, i've finally understood the essence of uwu'd text

there are a few transformations:
1. nya-ify (eg. `naruhodo` -> `nyaruhodo`)
2. replace `l` and `r` with `w`
3. stutter sometimes (`hi` -> `h-hi`)
4. add a text emoji after punctuation (`,`, `.`, or `!`) sometimes
5. replace `small` with `smol`, `cute` with `cuuuuute`, `fluff` with `floof`

these transformation passes take advantage of sse4.1 vector intrinsics to process 16 bytes at once

multithreading is supported, so you can exploit all of your cpu cores for the noble goal
of uwu-ing massive amounts of text

utf-8 is handled elegantly by simply ignoring non-ascii characters in the input

unfortunately, due to both simd parallelism and multithreading, some words may not be fully uwu'd
if they were lucky enough to cross the boundary of a simd vector or a thread's buffer.
*they won't escape so easily next time*

### why isn't this readme uwu'd?
so its readable

if u happen to find uwu'd text more readable, there's always an [uwu'd](README_UWU.md) version

### ok i want uwu'd text, how do i run this myself?
1. clone this repo

### ok but why aren't there any of the settings i can change?!1?!!1
free will is an illusion

### ok i need to use this for something and i need the license info
mit license, but idk why u would need this for anything

### ok but i have an issue with this or a suggestion
open an issue, be nice
