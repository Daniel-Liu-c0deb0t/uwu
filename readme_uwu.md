# uwuify (UwU'd Readme.md)
fastest text uwuifiew in t-the west

twansfowms
```
h-hey, >_< i t-think i weawwy w-wuv you. σωσ do you w-want a headpat?
```
i-into
```
hey, 🥺 (ꈍᴗꈍ) i-i t-think i weawwy wuv you. 🥺 ^•ﻌ•^ do y-you want a headpat?
```

thewe's an [uwu'd](readme_uwu.md) v-vewsion of this weadme


## faq
### nyani?
u w-want wawge amounts of text uwu'd i-in a smow amount of time

### whewe?
uw computew, ʘwʘ if it has a w-wecent x86 cpu (intew, :3 amd) that s-suppowts sse4.1

### w-why?
why nyot?

### how?
twdw: 128-bit simd vectowization pwus some big bwain a-awgos

<details>
<summary>cwick fow mowe info</summary>
<p>

aftew houws of weseawch, (U ﹏ U) i've finawwy undewstood t-the essence of uwu'd text

thewe a-awe a few twansfowmations:
1. (U ﹏ U) w-wepwace some wowds (`smow` -> `smow`, ʘwʘ e-etc.)
2. \>w< n-nyya-ify (eg. rawr x3 `nawuhodo` -> `nyawuhodo`)
3. OwO wepwace `l` and `r` w-with `w`
4. ^•ﻌ•^ stuttew sometimes (`hi` -> `h-hi`)
5. \>_< add a text emoji a-aftew punctuation (`,`, OwO `.`, ow `!`) sometimes

these twansfowmation passes take advantage of sse4.1 vectow intwinsics t-to pwocess 16 bytes at o-once. >_<
fow stwing s-seawching, (ꈍᴗꈍ) i'm u-using a custom simd impwementation of the
[bitap](https://en.wikipedia.org/wiki/bitap_algorithm) awgowithm fow m-matching against m-muwtipwe stwings. >w<
fow wandom nyumbew g-genewation, (U ﹏ U) i-i'm using [xorshift32](https://en.wikipedia.org/wiki/xorshift). ^^ fow most
chawactew-wevew d-detection within simd w-wegistews, (U ﹏ U) its aww masking and shifting to simuwate b-basic state
machines in pawawwew

m-muwtithweading is suppowted, :3 s-so u can expwoit a-aww of uw cpu cowes fow the nyobwe goaw
of uwu-ing massive amounts of text

utf-8 is handwed ewegantwy by simpwy i-ignowing nyon-ascii c-chawactews in the input

u-unfowtunatewy, d-due to both simd p-pawawwewism and muwtithweading, (✿oωo) some wowds may nyot be fuwwy u-uwu'd
if they wewe wucky enough to cwoss the boundawy of a simd vectow ow a thwead's b-buffew. XD
*they won't escape s-so easiwy nyext t-time*


</p>
</details>

### o-ok i want uwu'd text, >w< h-how do i wun this m-mysewf?
#### i-instaww command-wine t-toow
1. òωó instaww wust: wun `curl https://sh.rustup.rs -sSf | sh` on unix,  (ꈍᴗꈍ)
ow g-go [hewe](https://www.rust-lang.org/tools/install) f-fow mowe options
2. rawr x3 w-wun `cargo install uwuify`
3. rawr x3 wun `uwuify` which will read from stdin and o-owtput to stdout. σωσ make suwe u
pwess ctww + d (unix) ow ctww + z and entew (windows) aftew u type s-stuff in stdin to send an eof

if you awe having twoubwe wunning `uwuify`, (ꈍᴗꈍ) make s-suwe you have `~/.cargo/bin`
i-in youw `$PATH`

-it is possibwe to wead and wwite f-fwom fiwes by specifying the i-input fiwe and
output f-fiwe, rawr in that owdew. ^^;; u can use `--help` fow mowe info. rawr x3 pass in
`-v` fow timings

this is o-on crates.io [hewe](https://crates.io/crates/uwuify)

#### include as library
1. put `uwuify = "^0.2"` undew `[dependencies]` i-in youw `Cargo.toml` fiwe
2. the wibwawy i-is cawwed `uwuifier` (swightwy d-diffewent fwom the nyame of t-the binawy!)
use i-it wike so:
```rust
use uwuifier::uwuify_str_sse;
assert_eq!(uwuify_str_sse("hello world"), "hewwo wowwd");
```

d-documentation i-is [hewe](https://docs.rs/uwuify/latest/uwuifier/)

#### b-buiwd fwom this wepo
<details>
<summary>cwick f-fow mowe i-info</summary>
<p>

1. instaww rust
2. (U ﹏ U) wun `git clone https://github.com/Daniel-Liu-c0deb0t/uwu.git && cd uwu`
3. \>w< wun `cargo run --release`

##### t-testing
1. wun `cargo test`

##### b-benchmawking
1. run `mkdir test && cd test`

*wawning: w-wawge f-fiwes of 100mb and 1gb, 🥺 wespectivewy*

2. wun `curl -OL http://mattmahoney.net/dc/enwik8.zip && unzip enwik8.zip`
3. rawr x3 wun `curl -OL http://mattmahoney.net/dc/enwik9.zip && unzip enwik9.zip`
4. σωσ wun `cd .. && ./bench.sh`

</p>
</details>

### i-i don't bewieve that this is fast. (///ˬ///✿) i nyeed pwoof!!1! (U ﹏ U)
t-twdw: can b-be awmost as fast as simpwy copying a fiwe

<details>
<summary>cwick for mowe i-info</summary>
<p>

w-waw numbews fwom wunning `./bench.sh` on a 2019 macbook pwo w-with eight
intew 2.3 ghz i9 cpus a-and 16 gb of wam awe shown bewow. ^^;; the dataset
used is the fiwst 100mb a-and fiwst 1gb of engwish w-wikipedia. 🥺 the s-same
dataset is used fow the [huttew p-pwize](http://prize.hutter1.net/)
fow text c-compwession

```
1 t-thwead uwu enwik8
t-time taken: 178 ms
input size: 100000000 b-bytes
o-output size: 115095591 bytes
thwoughput: 0.55992 g-gb/s

2 thwead u-uwu enwik8
time t-taken: 105 ms
input size: 100000000 bytes
output s-size: 115095591 bytes
thwoughput: 0.94701 gb/s

4 t-thwead uwu e-enwik8
time taken: 60 ms
input size: 100000000 bytes
output size: 115095591 b-bytes
t-thwoughput: 1.64883 g-gb/s

8 t-thwead uwu enwik8
time taken: 47 m-ms
input size: 100000000 bytes
output size: 115095591 bytes
thwoughput: 2.12590 gb/s

copy enwik8

weaw	0m0.035s
u-usew	0m0.001s
sys	0m0.031s

1 t-thwead uwu enwik9
time taken: 2087 m-ms
input size: 1000000000 bytes
o-output size: 1149772651 bytes
t-thwoughput: 0.47905 g-gb/s

2 thwead u-uwu enwik9
time t-taken: 992 ms
i-input size: 1000000000 bytes
output size: 1149772651 bytes
thwoughput: 1.00788 gb/s

4 thwead uwu enwik9
time taken: 695 ms
input s-size: 1000000000 b-bytes
output s-size: 1149772651 bytes
thwoughput: 1.43854 g-gb/s

8 thwead uwu enwik9
time taken: 436 ms
input s-size: 1000000000 b-bytes
output size: 1149772651 bytes
thwoughput: 2.29214 g-gb/s

copy enwik9

weaw	0m0.387s
usew	0m0.001s
s-sys	0m0.341s
```

*//todo: c-compawe with othew toows*


</p>
</details>

### w-why isn't this w-weadme uwu'd?
so its weadabwe

if u happen to find uwu'd text mowe weadabwe, òωó thewe's a-awways an [uwu'd](readme_uwu.md) v-vewsion

### o-ok but why a-awen't thewe any s-settings i can change?!1?!!1
fwee w-wiww is an iwwusion

### w-wtf this is so unpwofessionaw h-how awe u-u gonna get hiwed at faang nyow?! XD
d-don't wowwy, :3 i've got u covewed

#### titwe: u-uwu is aww you nyeed

#### abstwact

w-wecent advances i-in computing have made stwides i-in pawawwewization, (U ﹏ U) whethew
at a fine-gwained w-wevew with simd i-instwuctions, >w< o-ow at a high wevew with muwtipwe
cpu cowes. /(^•ω•^) taking advantage of t-these advances, (⑅˘꒳˘) we expwowe how the usefuw
task o-of pewfowming an u-uwu twansfowmation on pwain text c-can be scawed up to wawge
input d-datasets. ʘwʘ ouw c-contwibutions in this papew awe thweefowd: fiwst, rawr x3 w-we pwesent, (˘ω˘)
to ouw knowwedge, o.O the fiwst wigowous d-definition of u-uwu'd text. 😳 second, we show
ouw n-nyovew awgowithms fow uwu-ing t-text, expwoiting v-vectowization and
m-muwtithweading featuwes that awe avaiwabwe on modewn cpus. o.O finawwy, ^^;; we pwovide
wigowous expewimentaw wesuwts that show how ouw impwementation couwd be the
"fastest in the west." in ouw benchmawks, ( ͡o ω ͡o ) we obsewve t-that ouw impwementation
w-was awmost as a fast as a simpwe fiwe c-copy, ^^;; which is e-entiwewy io-bound. ^^;;
w-we bewieve ouw wowk has potentiaw a-appwications in vawious domains, f-fwom data
a-augmentation and text pwepwocessing f-fow nyatuwaw wanguage pwocessing, XD t-to
giving a-authows the abiwity to convey potentiawwy whowesome o-ow kawaii~ meme m-messages
with m-minimaw time and e-effowt. 🥺

*// t-todo: wwite papew*

*// t-todo: wwite m-mowe about machine w-weawning s-so i get funding*

### ok i nyeed t-to use this fow s-something and i-i nyeed the wicense info
mit wicense

### o-ok but i have an issue with this ow a s-suggestion ow a question nyot answewed h-hewe
open a-an issue, (///ˬ///✿) be nyice

### projects using this
* [uwu-tway](https://github.com/Olaren15/uwu-tray): a tray icon to uwuify your text
* [uwubot](https://github.com/yaahc/uwubot): discord bot for uwuifying text
* [uwupedia](http://uwupedia.org/): the uwuified encycwopedia
* [discowd uwu webhook](https://github.com/bs2kbs2k/discord-uwu-webhook): automatically uwuifies all sent messages on discord via webhooks
* [twent weznowor](https://twitter.com/twent_weznowor): best twitter bot ever
* [awaia](https://github.com/TheRealKizu/Alaia/tree/master): a simple yet powerful intuitive chatbot for discord
* [uwuify-mdbook](https://github.com/alyti/uwuify-mdbook): an mdbook pre-processor for all your uwuify needs
* [uwu-joke](https://github.com/joshualeejunyi/uwu-joke): automatically uwuifies typed text and text copied to your clipboard
* [discordbot (go)](https://github.com/angch/discordbot): discord (and telegram and slack) bot for fun
* wet me know if u make a pwoject w-with uwuify! (U ᵕ U❁) i appweciate u-u aww!

### wefewences
* https://honk.moe/tools/owo.html
* https://github.com/IamRifki/uwuizer
* https://github.com/deadshot465/owoify_rs
* https://cutekaomoji.com/characters/uwu/
* https://cutekaomoji.com/characters/owo/
* https://cutekaomoji.com/characters/flower-girl/
* and m-many mowe; wet me know if i missed anything
