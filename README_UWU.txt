# uwuify
fastest text uwuifiew in t-the west

twansfowms
```
h-hey, (U ﹏ U) i t-think i weawwy w-wuv you. (˘ω˘) do you w-want a headpat?
```
i-into
```
hey, UwU (ꈍᴗꈍ) i-i t-think i weawwy wuv you. >_< ^•ﻌ•^ do y-you want a headpat?
```

thewe's an [uwu'd](weadme_uwu.txt) v-vewsion of this weadme

## faq
### nyani?
u w-want wawge amounts of text uwu'd i-in a smow amount of time

### whewe?
uw computew, σωσ if it has a w-wecent x86 cpu (intew, 🥺 amd) that s-suppowts sse4.1

### w-why?
why nyot?

### how?
twdw: 128-bit simd vectowization pwus some big bwain a-awgos

<detaiws>
<summawy>cwick fow mowe info</summawy>
<p>

aftew houws of weseawch, 🥺 i've finawwy undewstood t-the essence of uwu'd text

thewe a-awe a few twansfowmations:
1. ʘwʘ w-wepwace some wowds (`smow` -> `smow`, :3 e-etc.)
2. (U ﹏ U) n-nyya-ify (eg. (U ﹏ U) `nawuhodo` -> `nyawuhodo`)
3. ʘwʘ wepwace `w` and `w` w-with `w`
4. >w< stuttew sometimes (`hi` -> `h-hi`)
5. rawr x3 add a text emoji a-aftew punctuation (`,`, OwO `.`, ow `!`) sometimes

these twansfowmation passes take advantage of sse4.1 vectow intwinsics t-to pwocess 16 bytes at o-once. ^•ﻌ•^
fow stwing s-seawching, >_< i'm u-using a custom simd impwementation of the
[bitap](https://en.wikipedia.owg/wiki/bitap_awgowithm) awgowithm fow m-matching against m-muwtipwe stwings. OwO
fow wandom nyumbew g-genewation, >_< i-i'm using [xowshift32](https://en.wikipedia.owg/wiki/xowshift). (ꈍᴗꈍ) fow most
chawactew-wevew d-detection within simd w-wegistews, >w< its aww masking and shifting to simuwate b-basic state
machines in pawawwew

m-muwtithweading is suppowted, (U ﹏ U) s-so u can expwoit a-aww of uw cpu cowes fow the nyobwe goaw
of uwu-ing massive amounts of text

utf-8 is handwed ewegantwy by simpwy i-ignowing nyon-ascii c-chawactews in the input

u-unfowtunatewy, d-due to both simd p-pawawwewism and muwtithweading, ^^ some wowds may nyot be fuwwy u-uwu'd
if they wewe wucky enough to cwoss the boundawy of a simd vectow ow a thwead's b-buffew. (U ﹏ U)
*they won't escape s-so easiwy nyext t-time*

</p>
</detaiws>

### o-ok i want uwu'd text, :3 h-how do i wun this m-mysewf?
#### i-instaww command-wine t-toow
1. (✿oωo) instaww wust: wun `cuww https://sh.wustup.ws -ssf | s-sh` on unix, XD
ow g-go [hewe](https://www.wust-wang.owg/toows/instaww) f-fow mowe options
2. >w< w-wun `cawgo i-instaww uwuify`
3. òωó wun `uwuify` which wiww wead fwom stdin and o-output to stdout. (ꈍᴗꈍ) make suwe u
pwess ctww + d (unix) ow ctww + z and entew (windows) aftew u type s-stuff in stdin to send an eof

if you awe having twoubwe wunning `uwuify`, rawr x3 make s-suwe you have `~/.cawgo/bin`
i-in youw `$path`

i-it is possibwe to wead and wwite f-fwom fiwes by specifying the i-input fiwe and
output f-fiwe, rawr x3 in that owdew. σωσ u can use `--hewp` fow mowe info. (ꈍᴗꈍ) pass in
`-v` fow timings

this is on c-cwates.io [hewe](https://cwates.io/cwates/uwuify)

#### incwude a-as wibwawy
1. rawr put `uwuify = "^0.2"` u-undew `[dependencies]` i-in youw `cawgo.tomw` fiwe
2. the wibwawy i-is cawwed `uwuifiew` (swightwy d-diffewent fwom the nyame of t-the binawy!)
use i-it wike so:
```wust
use uwuifiew::uwuify_stw_sse;
assewt_eq!(uwuify_stw_sse("hewwo wowwd"), ^^;; "hewwo wowwd");
```

d-documentation i-is [hewe](https://docs.ws/uwuify/watest/uwuifiew/)

#### b-buiwd fwom this wepo
<detaiws>
<summawy>cwick f-fow mowe i-info</summawy>
<p>

1. instaww w-wust
2. rawr x3 wun `git cwone https://github.com/daniew-wiu-c0deb0t/uwu.git && cd uwu`
3. (ˆ ﻌ ˆ)♡ wun `cawgo wun --wewease`

##### testing
1. σωσ wun `cawgo t-test`

##### b-benchmawking
1. (U ﹏ U) wun `mkdiw test && cd test`

*wawning: w-wawge f-fiwes of 100mb and 1gb, >w< wespectivewy*

2. wun `cuww -ow http://mattmahoney.net/dc/enwik8.zip && u-unzip enwik8.zip`
3. σωσ wun `cuww -ow http://mattmahoney.net/dc/enwik9.zip && unzip enwik9.zip`
4. nyaa~~ wun `cd .. && ./bench.sh`

</p>
</detaiws>

### i-i don't bewieve that this is fast. 🥺 i nyeed pwoof!!1! rawr x3
t-twdw: can b-be awmost as fast as simpwy copying a fiwe

<detaiws>
<summawy>cwick fow mowe i-info</summawy>
<p>

w-waw numbews fwom wunning `./bench.sh` on a 2019 macbook pwo w-with eight
intew 2.3 ghz i9 cpus a-and 16 gb of wam awe shown bewow. σωσ the dataset
used is the fiwst 100mb a-and fiwst 1gb of engwish w-wikipedia. (///ˬ///✿) the s-same
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

if u happen to find uwu'd text mowe weadabwe, (U ﹏ U) thewe's a-awways an [uwu'd](weadme_uwu.txt) v-vewsion

### o-ok but why a-awen't thewe any s-settings i can change?!1?!!1
fwee w-wiww is an iwwusion

### w-wtf this is so unpwofessionaw h-how awe u-u gonna get hiwed at faang nyow?! ^^;;
d-don't wowwy, 🥺 i've got u covewed

#### titwe: u-uwu is aww you nyeed

#### abstwact

w-wecent advances i-in computing have made stwides i-in pawawwewization, òωó whethew
at a fine-gwained w-wevew with simd i-instwuctions, XD o-ow at a high wevew with muwtipwe
cpu cowes. :3 taking advantage of t-these advances, (U ﹏ U) we expwowe how the usefuw
task o-of pewfowming an u-uwu twansfowmation on pwain text c-can be scawed up to wawge
input d-datasets. >w< ouw c-contwibutions in this papew awe thweefowd: fiwst, /(^•ω•^) w-we pwesent, (⑅˘꒳˘)
to ouw knowwedge, ʘwʘ the fiwst wigowous d-definition of u-uwu'd text. rawr x3 second, we show
ouw n-nyovew awgowithms fow uwu-ing t-text, expwoiting v-vectowization and
m-muwtithweading featuwes that awe avaiwabwe on modewn cpus. (˘ω˘) finawwy, o.O we pwovide
wigowous expewimentaw wesuwts that show how ouw impwementation couwd be the
"fastest in the west." in ouw benchmawks, 😳 we obsewve t-that ouw impwementation
w-was awmost as a fast as a simpwe fiwe c-copy, o.O which is e-entiwewy io-bound. ^^;;
w-we bewieve ouw wowk has potentiaw a-appwications in vawious domains, f-fwom data
a-augmentation and text pwepwocessing f-fow nyatuwaw wanguage pwocessing, ( ͡o ω ͡o ) t-to
giving a-authows the abiwity to convey potentiawwy whowesome o-ow kawaii~ meme m-messages
with m-minimaw time and e-effowt. ^^;;

*// t-todo: wwite papew*

*// t-todo: wwite m-mowe about machine w-weawning s-so i get funding*

### ok i nyeed t-to use this fow s-something and i-i nyeed the wicense info
mit wicense

### o-ok but i have an issue with this ow a s-suggestion ow a question nyot answewed h-hewe
open a-an issue, ^^;; be nyice

### p-pwojects using this
* [uwu-tway](https://github.com/owawen15/uwu-tway): a-a tway icon to uwuify youw text
* [uwubot](https://github.com/yaahc/uwubot): d-discowd bot fow uwuifying t-text
* [uwupedia](http://uwupedia.owg/): the uwuified encycwopedia
* [discowd u-uwu webhook](https://github.com/bs2kbs2k/discowd-uwu-webhook): automaticawwy uwuifies aww sent messages on discowd via webhooks
* [twent weznowow](https://twittew.com/twent_weznowow): b-best twittew bot evew
* w-wet me know i-if you make a pwoject with uwuify! XD i appweciate u aww! 🥺

### wefewences
* h-https://honk.moe/toows/owo.htmw
* https://github.com/iamwifki/uwuizew
* h-https://github.com/deadshot465/owoify_ws
* h-https://kawaii~kaomoji.com/chawactews/uwu/
* h-https://kawaii~kaomoji.com/chawactews/owo/
* https://kawaii~kaomoji.com/chawactews/fwowew-giww/
* and m-many mowe; wet m-me know if i missed anything
