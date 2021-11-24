# uwuify
fastest text uwuifiew in t-the west

twansfowms
```
h-hey, >w< i t-think i weawwy w-wuv you. rawr x3 do you w-want a headpat?
```
i-into
```
hey, OwO (ꈍᴗꈍ) i-i t-think i weawwy wuv you. ^•ﻌ•^ ^•ﻌ•^ do y-you want a headpat?
```

thewe's an [uwu'd](weadme_uwu.txt) v-vewsion of this weadme

## faq
### nyani?
u w-want wawge amounts of text uwu'd i-in a smow amount of time

### whewe?
uw computew, >_< if it has a w-wecent x86 cpu (intew, OwO amd) that s-suppowts sse4.1

### w-why?
why nyot?

### how?
twdw: 128-bit simd vectowization pwus some big bwain a-awgos

<detaiws>
<summawy>cwick fow mowe info</summawy>
<p>

aftew houws of weseawch, >_< i've finawwy undewstood t-the essence of uwu'd text

thewe a-awe a few twansfowmations:
1. (ꈍᴗꈍ) w-wepwace some wowds (`smow` -> `smow`, >w< e-etc.)
2. (U ﹏ U) n-nyya-ify (eg. ^^ `nawuhodo` -> `nyawuhodo`)
3. (U ﹏ U) wepwace `w` and `w` w-with `w`
4. :3 stuttew sometimes (`hi` -> `h-hi`)
5. (✿oωo) add a text emoji a-aftew punctuation (`,`, XD `.`, ow `!`) sometimes

these twansfowmation passes take advantage of sse4.1 vectow intwinsics t-to pwocess 16 bytes at o-once. >w<
fow stwing s-seawching, òωó i'm u-using a custom simd impwementation of the
[bitap](https://en.wikipedia.owg/wiki/bitap_awgowithm) awgowithm fow m-matching against m-muwtipwe stwings. (ꈍᴗꈍ)
fow wandom nyumbew g-genewation, rawr x3 i-i'm using [xowshift32](https://en.wikipedia.owg/wiki/xowshift). rawr x3 fow most
chawactew-wevew d-detection within simd w-wegistews, σωσ its aww masking and shifting to simuwate b-basic state
machines in pawawwew

m-muwtithweading is suppowted, (ꈍᴗꈍ) s-so u can expwoit a-aww of uw cpu cowes fow the nyobwe goaw
of uwu-ing massive amounts of text

utf-8 is handwed ewegantwy by simpwy i-ignowing nyon-ascii c-chawactews in the input

u-unfowtunatewy, d-due to both simd p-pawawwewism and muwtithweading, rawr some wowds may nyot be fuwwy u-uwu'd
if they wewe wucky enough to cwoss the boundawy of a simd vectow ow a thwead's b-buffew. ^^;;
*they won't escape s-so easiwy nyext t-time*

</p>
</detaiws>

### o-ok i want uwu'd text, rawr x3 h-how do i wun this m-mysewf?
#### i-instaww command-wine t-toow
1. (ˆ ﻌ ˆ)♡ instaww wust: wun `cuww https://sh.wustup.ws -ssf | s-sh` on unix, σωσ
ow g-go [hewe](https://www.wust-wang.owg/toows/instaww) f-fow mowe options
2. (U ﹏ U) w-wun `cawgo i-instaww uwuify`
3. >w< wun `uwuify` which wiww wead fwom stdin and o-output to stdout. σωσ make suwe u
pwess ctww + d (unix) ow ctww + z and entew (windows) aftew u type s-stuff in stdin to send an eof

if you awe having twoubwe wunning `uwuify`, nyaa~~ make s-suwe you have `~/.cawgo/bin`
i-in youw `$path`

i-it is possibwe to wead and wwite f-fwom fiwes by specifying the i-input fiwe and
output f-fiwe, 🥺 in that owdew. rawr x3 u can use `--hewp` fow mowe info. σωσ pass in
`-v` fow timings

this is on c-cwates.io [hewe](https://cwates.io/cwates/uwuify)

#### incwude a-as wibwawy
1. (///ˬ///✿) put `uwuify = "^0.2"` u-undew `[dependencies]` i-in youw `cawgo.tomw` fiwe
2. the wibwawy i-is cawwed `uwuifiew` (swightwy d-diffewent fwom the nyame of t-the binawy!)
use i-it wike so:
```wust
use uwuifiew::uwuify_stw_sse;
assewt_eq!(uwuify_stw_sse("hewwo wowwd"), (U ﹏ U) "hewwo wowwd");
```

d-documentation i-is [hewe](https://docs.ws/uwuify/watest/uwuifiew/)

#### b-buiwd fwom this wepo
<detaiws>
<summawy>cwick f-fow mowe i-info</summawy>
<p>

1. instaww w-wust
2. ^^;; wun `git cwone https://github.com/daniew-wiu-c0deb0t/uwu.git && cd uwu`
3. 🥺 wun `cawgo wun --wewease`

##### testing
1. òωó wun `cawgo t-test`

##### b-benchmawking
1. XD wun `mkdiw test && cd test`

*wawning: w-wawge f-fiwes of 100mb and 1gb, :3 wespectivewy*

2. wun `cuww -ow http://mattmahoney.net/dc/enwik8.zip && u-unzip enwik8.zip`
3. (U ﹏ U) wun `cuww -ow http://mattmahoney.net/dc/enwik9.zip && unzip enwik9.zip`
4. >w< wun `cd .. && ./bench.sh`

</p>
</detaiws>

### i-i don't bewieve that this is fast. /(^•ω•^) i nyeed pwoof!!1! (⑅˘꒳˘)
t-twdw: can b-be awmost as fast as simpwy copying a fiwe

<detaiws>
<summawy>cwick fow mowe i-info</summawy>
<p>

w-waw numbews fwom wunning `./bench.sh` on a 2019 macbook pwo w-with eight
intew 2.3 ghz i9 cpus a-and 16 gb of wam awe shown bewow. ʘwʘ the dataset
used is the fiwst 100mb a-and fiwst 1gb of engwish w-wikipedia. rawr x3 the s-same
dataset is used fow the [huttew p-pwize](http://pwize.huttew1.net/)
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
</detaiws>

### w-why isn't this w-weadme uwu'd?
so its weadabwe

if u happen to find uwu'd text mowe weadabwe, (˘ω˘) thewe's a-awways an [uwu'd](weadme_uwu.txt) v-vewsion

### o-ok but why a-awen't thewe any s-settings i can change?!1?!!1
fwee w-wiww is an iwwusion

### w-wtf this is so unpwofessionaw h-how awe u-u gonna get hiwed at faang nyow?! o.O
d-don't wowwy, 😳 i've got u covewed

#### titwe: u-uwu is aww you nyeed

#### abstwact

w-wecent advances i-in computing have made stwides i-in pawawwewization, o.O whethew
at a fine-gwained w-wevew with simd i-instwuctions, ^^;; o-ow at a high wevew with muwtipwe
cpu cowes. ( ͡o ω ͡o ) taking advantage of t-these advances, ^^;; we expwowe how the usefuw
task o-of pewfowming an u-uwu twansfowmation on pwain text c-can be scawed up to wawge
input d-datasets. ^^;; ouw c-contwibutions in this papew awe thweefowd: fiwst, XD w-we pwesent, 🥺
to ouw knowwedge, (///ˬ///✿) the fiwst wigowous d-definition of u-uwu'd text. (U ᵕ U❁) second, we show
ouw n-nyovew awgowithms fow uwu-ing t-text, expwoiting v-vectowization and
m-muwtithweading featuwes that awe avaiwabwe on modewn cpus. ^^;; finawwy, ^^;; we pwovide
wigowous expewimentaw wesuwts that show how ouw impwementation couwd be the
"fastest in the west." in ouw benchmawks, rawr we obsewve t-that ouw impwementation
w-was awmost as a fast as a simpwe fiwe c-copy, (˘ω˘) which is e-entiwewy io-bound. 🥺
w-we bewieve ouw wowk has potentiaw a-appwications in vawious domains, f-fwom data
a-augmentation and text pwepwocessing f-fow nyatuwaw wanguage pwocessing, nyaa~~ t-to
giving a-authows the abiwity to convey potentiawwy whowesome o-ow kawaii~ meme m-messages
with m-minimaw time and e-effowt. :3

*// t-todo: wwite papew*

*// t-todo: wwite m-mowe about machine w-weawning s-so i get funding*

### ok i nyeed t-to use this fow s-something and i-i nyeed the wicense info
mit wicense

### o-ok but i have an issue with this ow a s-suggestion ow a question nyot answewed h-hewe
open a-an issue, /(^•ω•^) be nyice

### p-pwojects using this
* [uwu-tway](https://github.com/owawen15/uwu-tway): a-a tway icon to uwuify youw text
* [uwubot](https://github.com/yaahc/uwubot): d-discowd bot fow uwuifying t-text
* [uwupedia](http://uwupedia.owg/): the uwuified encycwopedia
* [discowd u-uwu webhook](https://github.com/bs2kbs2k/discowd-uwu-webhook): automaticawwy uwuifies aww sent messages on discowd via webhooks
* [twent weznowow](https://twittew.com/twent_weznowow): b-best twittew bot evew
* [awaia](https://github.com/theweawkizu/awaia/twee/mastew): a-a simpwe yet powewfuw i-intuitive chatbot fow discowd
* [uwuify-mdbook](https://github.com/awyti/uwuify-mdbook): an mdbook pwe-pwocessow fow aww y-youw uwuify nyeeds
* [uwu-joke](https://github.com/joshuaweejunyi/uwu-joke): automaticawwy u-uwuifies t-typed text and t-text copied to youw cwipboawd
* [discowdbot (go)](https://github.com/angch/discowdbot): discowd (and t-tewegwam a-and swack) bot fow fun
* wet me k-know if u make a pwoject with uwuify! ^•ﻌ•^ i appweciate u-u aww! UwU

### wefewences
* https://honk.moe/toows/owo.htmw
* https://github.com/iamwifki/uwuizew
* h-https://github.com/deadshot465/owoify_ws
* h-https://kawaii~kaomoji.com/chawactews/uwu/
* h-https://kawaii~kaomoji.com/chawactews/owo/
* https://kawaii~kaomoji.com/chawactews/fwowew-giww/
* and m-many mowe; wet m-me know if i missed a-anything
