# uwuify
fastest text uwuifier in the west

transforms
```
Hey, I think I really love you. Do you want a headpat?
```
into
```
hey, (ꈍᴗꈍ) i think i weawwy wuv you. ^•ﻌ•^ do y-you want a headpat?
```

there's an [uwu'd](README_UWU.txt) version of this readme

## faq
### what?
u want large amounts of text uwu'd in a smol amount of time

### where?
ur computer, if it has a recent x86 cpu (intel, amd) that supports sse4.1

### why?
why not?

### how?
tldr: 128-bit simd vectorization plus some big brain algos

<details>
<summary>click for more info</summary>
<p>

after hours of research, i've finally understood the essence of uwu'd text

there are a few transformations:
1. replace some words (`small` -> `smol`, etc.)
2. nya-ify (eg. `naruhodo` -> `nyaruhodo`)
3. replace `l` and `r` with `w`
4. stutter sometimes (`hi` -> `h-hi`)
5. add a text emoji after punctuation (`,`, `.`, or `!`) sometimes

these transformation passes take advantage of sse4.1 vector intrinsics to process 16 bytes at once.
for string searching, i'm using a custom simd implementation of the
[bitap](https://en.wikipedia.org/wiki/Bitap_algorithm) algorithm for matching against multiple strings.
for random number generation, i'm using [XorShift32](https://en.wikipedia.org/wiki/Xorshift). for most
character-level detection within simd registers, its all masking and shifting to simulate basic state
machines in parallel

multithreading is supported, so u can exploit all of ur cpu cores for the noble goal
of uwu-ing massive amounts of text

utf-8 is handled elegantly by simply ignoring non-ascii characters in the input

unfortunately, due to both simd parallelism and multithreading, some words may not be fully uwu'd
if they were lucky enough to cross the boundary of a simd vector or a thread's buffer.
*they won't escape so easily next time*

</p>
</details>

### ok i want uwu'd text, how do i run this myself?
#### install command-line tool
1. install rust: run `curl https://sh.rustup.rs -sSf | sh` on unix,
or go [here](https://www.rust-lang.org/tools/install) for more options
2. run `cargo install uwuify`
3. run `uwuify` which will read from stdin and output to stdout. make sure u
press ctrl + d (unix) or ctrl + z and enter (windows) after u type stuff in stdin to send an EOF

if you are having trouble running `uwuify`, make sure you have `~/.cargo/bin`
in your `$PATH`

it is possible to read and write from files by specifying the input file and
output file, in that order. u can use `--help` for more info. pass in
`-v` for timings

this is on crates.io [here](https://crates.io/crates/uwuify)

#### include as library
1. put `uwuify = "^0.2"` under `[dependencies]` in your `Cargo.toml` file
2. the library is called `uwuifier` (slightly different from the name of the binary!)
use it like so:
```rust
use uwuifier::uwuify_str_sse;
assert_eq!(uwuify_str_sse("hello world"), "hewwo wowwd");
```

documentation is [here](https://docs.rs/uwuify/latest/uwuifier/)

#### build from this repo
<details>
<summary>click for more info</summary>
<p>

1. install rust
2. run `git clone https://github.com/Daniel-Liu-c0deb0t/uwu.git && cd uwu`
3. run `cargo run --release`

##### testing
1. run `cargo test`

##### benchmarking
1. run `mkdir test && cd test`

*warning: large files of 100mb and 1gb, respectively*

2. run `curl -OL http://mattmahoney.net/dc/enwik8.zip && unzip enwik8.zip`
3. run `curl -OL http://mattmahoney.net/dc/enwik9.zip && unzip enwik9.zip`
4. run `cd .. && ./bench.sh`

</p>
</details>

### i don't believe that this is fast. i need proof!!1!
tldr: can be almost as fast as simply copying a file

<details>
<summary>click for more info</summary>
<p>

raw numbers from running `./bench.sh` on a 2019 macbook pro with eight
intel 2.3 ghz i9 cpus and 16 gb of ram are shown below. the dataset
used is the first 100mb and first 1gb of english wikipedia. the same
dataset is used for the [hutter prize](http://prize.hutter1.net/)
for text compression

```
1 thread uwu enwik8
time taken: 178 ms
input size: 100000000 bytes
output size: 115095591 bytes
throughput: 0.55992 gb/s

2 thread uwu enwik8
time taken: 105 ms
input size: 100000000 bytes
output size: 115095591 bytes
throughput: 0.94701 gb/s

4 thread uwu enwik8
time taken: 60 ms
input size: 100000000 bytes
output size: 115095591 bytes
throughput: 1.64883 gb/s

8 thread uwu enwik8
time taken: 47 ms
input size: 100000000 bytes
output size: 115095591 bytes
throughput: 2.12590 gb/s

copy enwik8

real	0m0.035s
user	0m0.001s
sys	0m0.031s

1 thread uwu enwik9
time taken: 2087 ms
input size: 1000000000 bytes
output size: 1149772651 bytes
throughput: 0.47905 gb/s

2 thread uwu enwik9
time taken: 992 ms
input size: 1000000000 bytes
output size: 1149772651 bytes
throughput: 1.00788 gb/s

4 thread uwu enwik9
time taken: 695 ms
input size: 1000000000 bytes
output size: 1149772651 bytes
throughput: 1.43854 gb/s

8 thread uwu enwik9
time taken: 436 ms
input size: 1000000000 bytes
output size: 1149772651 bytes
throughput: 2.29214 gb/s

copy enwik9

real	0m0.387s
user	0m0.001s
sys	0m0.341s
```

*//TODO: compare with other tools*

</p>
</details>

### why isn't this readme uwu'd?
so its readable

if u happen to find uwu'd text more readable, there's always an [uwu'd](README_UWU.txt) version

### ok but why aren't there any settings i can change?!1?!!1
free will is an illusion

### wtf this is so unprofessional how are u gonna get hired at faang now?!
don't worry, i've got u covered

#### Title: uwu is all you need

#### Abstract

Recent advances in computing have made strides in parallelization, whether
at a fine-grained level with SIMD instructions, or at a high level with multiple
CPU cores. Taking advantage of these advances, we explore how the useful
task of performing an uwu transformation on plain text can be scaled up to large
input datasets. Our contributions in this paper are threefold: first, we present,
to our knowledge, the first rigorous definition of uwu'd text. Second, we show
our novel algorithms for uwu-ing text, exploiting vectorization and
multithreading features that are available on modern CPUs. Finally, we provide
rigorous experimental results that show how our implementation could be the
"fastest in the west." In our benchmarks, we observe that our implementation
was almost as a fast as a simple file copy, which is entirely IO-bound.
We believe our work has potential applications in various domains, from data
augmentation and text preprocessing for natural language processing, to
giving authors the ability to convey potentially wholesome or cute meme messages
with minimal time and effort.

*// TODO: write paper*

*// TODO: write more about machine learning so i get funding*

### ok i need to use this for something and i need the license info
mit license

### ok but i have an issue with this or a suggestion or a question not answered here
open an issue, be nice

### projects using this
* [uwu-tray](https://github.com/Olaren15/uwu-tray): a tray icon to uwuify your text
* [uwubot](https://github.com/yaahc/uwubot): discord bot for uwuifying text
* [uwupedia](http://uwupedia.org/): the uwuified encycwopedia
* [discord uwu webhook](https://github.com/bs2kbs2k/discord-uwu-webhook): automatically uwuifies all sent messages on discord via webhooks
* [twent weznowor](https://twitter.com/twent_weznowor): best twitter bot ever
* [alaia](https://github.com/TheRealKizu/Alaia/tree/master): a simple yet powerful intuitive chatbot for discord
* [uwuify-mdbook](https://github.com/alyti/uwuify-mdbook): an mdbook pre-processor for all your uwuify needs
* [uwu-joke](https://github.com/joshualeejunyi/uwu-joke): automatically uwuifies typed text and text copied to your clipboard
* let me know if u make a project with uwuify! i appreciate u all!

### references
* https://honk.moe/tools/owo.html
* https://github.com/IamRifki/uwuizer
* https://github.com/deadshot465/owoify_rs
* https://cutekaomoji.com/characters/uwu/
* https://cutekaomoji.com/characters/owo/
* https://cutekaomoji.com/characters/flower-girl/
* and many more; let me know if i missed anything
