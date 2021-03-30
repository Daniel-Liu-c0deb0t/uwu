# uwuify
fastest t-text uwuifiew in t-the west

twansfowms
```
hey... i-i think i weawwy wuv you. (///ˬ///✿) do you w-want a headpat?
```
into
```
hey... i-i think i w-weawwy wuv you. ( ͡o ω ͡o ) (⑅˘꒳˘) d-do you want a headpat?
```

t-thewe's an [uwu'd](weadme_uwu.txt) v-vewsion of this w-weadme

## faq
### n-nyani?
u want wawge amounts of text uwu'd in a smow amount of time

### whewe?
uw computew, o.O if i-it has a wecent x86 cpu (intew, UwU amd) that suppowts sse4.1

### why?
why nyot?

### h-how?
twdw: 128-bit s-simd vectowization pwus s-some big bwain awgos

<detaiws>
<summawy>cwick fow mowe info</summawy>
<p>

aftew houws of weseawch, (˘ω˘) i-i've finawwy undewstood the e-essence of uwu'd t-text

thewe awe a-a few twansfowmations:
1. (U ᵕ U❁) n-nyya-ify (eg. ʘwʘ `nawuhodo` -> `nyawuhodo`)
2. -.- wepwace `w` a-and `w` with `w`
3. σωσ stuttew sometimes (`hi` -> `h-hi`)
4. UwU a-add a-a text emoji aftew p-punctuation (`,`, σωσ `.`, ow `!`) sometimes
5. OwO wepwace some wowds (`smow` -> `smow`, OwO e-etc.)

these twansfowmation p-passes take advantage of sse4.1 vectow intwinsics to pwocess 16 bytes at once. o.O
f-fow stwing seawching, (U ﹏ U) i'm using a custom simd i-impwementation of the
[bitap](https://en.wikipedia.owg/wiki/bitap_awgowithm) awgowithm f-fow matching a-against muwtipwe s-stwings. σωσ
fow wandom nyumbew genewation, ʘwʘ i'm using [xowshift32](https://en.wikipedia.owg/wiki/xowshift). (U ﹏ U) fow most
chawactew-wevew detection w-within simd wegistews, (ꈍᴗꈍ) i-its aww masking a-and shifting t-to simuwate b-basic state
machines i-in pawawwew

muwtithweading is suppowted, -.- so u-u can expwoit aww of uw cpu cowes f-fow the nyobwe goaw
of uwu-ing m-massive amounts o-of text

utf-8 is handwed ewegantwy by simpwy ignowing nyon-ascii c-chawactews in the input

unfowtunatewy, o.O due t-to both simd pawawwewism and muwtithweading, some wowds may nyot b-be fuwwy uwu'd
if they wewe wucky e-enough to cwoss t-the boundawy o-of a simd vectow o-ow a thwead's buffew. (⑅˘꒳˘)
*they won't e-escape so easiwy n-nyext time*

</p>
</detaiws>

### o-ok i want uwu'd text, ( ͡o ω ͡o ) how d-do i wun this mysewf?
#### instaww command-wine t-toow
1. (///ˬ///✿) instaww w-wust: wun `cuww https://sh.wustup.ws -ssf | s-sh` on unix, >w<
ow go [hewe](https://www.wust-wang.owg/toows/instaww) f-fow mowe options
2. w-wun `cawgo instaww uwuify`
3. w-wun `uwuify` which w-wiww wead fwom s-stdin and output to stdout. σωσ m-make suwe u
pwess ctww + d (unix) o-ow ctww + z and e-entew (windows) a-aftew u type stuff in stdin to s-send an eof

if y-you awe having twoubwe wunning `uwuify`, o.O m-make suwe y-you have `~/.cawgo/bin`
i-in youw `$path`

i-it i-is possibwe to wead and wwite fwom fiwes by specifying t-the input fiwe and
output f-fiwe, -.- in that owdew. o.O u can use `--hewp` fow mowe info. ( ͡o ω ͡o ) pass in
`-v` fow timings

this is on cwates.io [hewe](https://cwates.io/cwates/uwuify)

#### incwude as w-wibwawy
1. o.O put `uwuify = "^0.2"` u-undew `[dependencies]` in youw `cawgo.tomw` fiwe
2. (U ﹏ U) t-the wibwawy i-is cawwed `uwuifiew` (swightwy d-diffewent fwom the nyame of the binawy!)
use it w-wike so:
```wust
use uwuifiew::uwuify_stw_sse;
assewt_eq!(uwuify_stw_sse("hewwo w-wowwd"), (U ﹏ U) "hewwo w-wowwd");
```

documentation is [hewe](https://docs.ws/uwuify/watest/uwuifiew/)

#### b-buiwd fwom t-this wepo
<detaiws>
<summawy>cwick f-fow mowe info</summawy>
<p>

1. (U ﹏ U) instaww wust
2. (U ᵕ U❁) wun `git cwone https://github.com/daniew-wiu-c0deb0t/uwu.git && cd uwu`
3. (U ᵕ U❁) wun `cawgo w-wun --wewease`

##### testing
1. wun `cawgo t-test`

##### b-benchmawking
1. (U ᵕ U❁) wun `mkdiw test && cd test`

*wawning: w-wawge fiwes o-of 100mb and 1gb, (///ˬ///✿) wespectivewy*

2. >w< wun `cuww -ow h-http://mattmahoney.net/dc/enwik8.zip && unzip enwik8.zip`
3. òωó wun `cuww -ow http://mattmahoney.net/dc/enwik9.zip && u-unzip enwik9.zip`
4. (˘ω˘) wun `cd .. && ./bench.sh`

</p>
</detaiws>

### i d-don't bewieve that t-this is fast. ʘwʘ i-i nyeed pwoof!!1! (U ᵕ U❁)
twdw: can be awmost as fast a-as simpwy copying a-a fiwe

<detaiws>
<summawy>cwick fow mowe info</summawy>
<p>

w-waw nyumbews fwom w-wunning `./bench.sh` on a 2019 macbook pwo with e-eight
intew 2.3 ghz i9 cpus and 16 gb of wam awe shown bewow. (˘ω˘) the dataset
used is the fiwst 100mb a-and fiwst 1gb of engwish wikipedia. (ꈍᴗꈍ) the same
dataset is used fow the [huttew p-pwize](http://pwize.huttew1.net/)
f-fow text compwession

```
1 thwead u-uwu enwik8
t-time taken: 178 m-ms
input size: 100000000 bytes
o-output size: 115095591 b-bytes
thwoughput: 0.55992 g-gb/s

2 thwead uwu enwik8
time taken: 105 ms
input s-size: 100000000 b-bytes
output size: 115095591 b-bytes
thwoughput: 0.94701 g-gb/s

4 thwead uwu enwik8
time taken: 60 ms
input size: 100000000 bytes
o-output size: 115095591 b-bytes
thwoughput: 1.64883 g-gb/s

8 thwead u-uwu enwik8
time taken: 47 ms
i-input size: 100000000 bytes
output size: 115095591 bytes
thwoughput: 2.12590 gb/s

c-copy enwik8

weaw	0m0.035s
usew	0m0.001s
s-sys	0m0.031s

1 thwead uwu enwik9
time taken: 2087 ms
input size: 1000000000 bytes
output size: 1149772651 bytes
thwoughput: 0.47905 gb/s

2 thwead uwu enwik9
time taken: 992 ms
input s-size: 1000000000 bytes
output s-size: 1149772651 bytes
thwoughput: 1.00788 gb/s

4 t-thwead uwu enwik9
time taken: 695 m-ms
input size: 1000000000 b-bytes
output size: 1149772651 bytes
t-thwoughput: 1.43854 gb/s

8 t-thwead uwu enwik9
t-time taken: 436 m-ms
input size: 1000000000 b-bytes
output size: 1149772651 b-bytes
t-thwoughput: 2.29214 gb/s

copy enwik9

weaw	0m0.387s
usew	0m0.001s
sys	0m0.341s
```

*//todo: compawe w-with othew t-toows*

</p>
</detaiws>

### why isn't this weadme uwu'd?
so its weadabwe

if u-u happen to find u-uwu'd text mowe weadabwe, (U ᵕ U❁) thewe's a-awways an [uwu'd](weadme_uwu.txt) vewsion

### ok but why awen't t-thewe any settings i can change?!1?!!1
f-fwee wiww is an iwwusion

### wtf this is so unpwofessionaw h-how awe u g-gonna get hiwed a-at faang now?! UwU
don't wowwy, (U ﹏ U) i've got u covewed

#### titwe: uwu is aww you nyeed

#### a-abstwact

w-wecent advances i-in computing have m-made stwides in pawawwewization, (U ﹏ U) whethew
at a fine-gwained wevew with simd instwuctions, UwU o-ow a-at a high wevew with muwtipwe
cpu c-cowes. -.- taking a-advantage of these advances, σωσ we e-expwowe how the u-usefuw
task of pewfowming a-an uwu twansfowmation on pwain text can b-be scawed up to w-wawge
input datasets. òωó o-ouw contwibutions i-in this p-papew awe thweefowd: fiwst, OwO we pwesent,
to ouw k-knowwedge, (˘ω˘) the f-fiwst wigowous definition o-of uwu'd text. (ꈍᴗꈍ) second, we show
ouw nyovew a-awgowithms fow u-uwu-ing text, >w< e-expwoiting vectowization a-and
muwtithweading f-featuwes that awe avaiwabwe o-on modewn c-cpus. rawr x3 finawwy, we pwovide
wigowous e-expewimentaw wesuwts that s-show how ouw impwementation couwd b-be the
"fastest in the west." i-in ouw benchmawks, (U ᵕ U❁) we obsewve that o-ouw impwementation
was awmost as a fast as a s-simpwe fiwe copy, σωσ w-which is entiwewy io-bound. ( ͡o ω ͡o )
we bewieve ouw wowk h-has potentiaw appwications in vawious domains, (U ᵕ U❁) fwom data
augmentation and text pwepwocessing fow n-nyatuwaw wanguage p-pwocessing, o.O t-to
giving authows t-the abiwity to c-convey potentiawwy whowesome ow kawaii~ meme messages
w-with minimaw t-time and effowt. (˘ω˘)

*// todo: w-wwite papew*

*// todo: wwite mowe a-about machine weawning so i g-get funding*

### ok i nyeed to u-use this fow something a-and i nyeed t-the wicense info
mit wicense

### o-ok but i have a-an issue with t-this ow a suggestion o-ow a question nyot answewed hewe
open an issue, ( ͡o ω ͡o ) be nyice

### pwojects using t-this
* [uwu-tway](https://github.com/owawen15/uwu-tway): a tway icon to uwuify youw text
* [uwubot](https://github.com/yaahc/uwubot): discowd bot fow uwuifying text
* [uwupedia](http://uwupedia.owg/): the uwuified encycwopedia
* wet me know if you make a-a pwoject with uwuify! o.O i appweciate u-u aww! (U ᵕ U❁)

### w-wefewences
* https://honk.moe/toows/owo.htmw
* https://github.com/iamwifki/uwuizew
* h-https://github.com/deadshot465/owoify_ws
* h-https://kawaii~kaomoji.com/chawactews/uwu/
* https://kawaii~kaomoji.com/chawactews/owo/
* https://kawaii~kaomoji.com/chawactews/fwowew-giww/
* and m-many mowe; wet me know if i missed anything