# uwuify
fastest t-text uwuifiew in t-the west

twansfowms
```
hey... i-i think i weawwy wuv you. o.O do you w-want a headpat?
```
into
```
hey... i-i think i w-weawwy wuv you. rawr x3 (⑅˘꒳˘) d-do you want a headpat?
```

t-thewe's an [uwu'd](weadme_uwu.txt) v-vewsion of this w-weadme

## faq
### n-nyani?
u want wawge amounts of text uwu'd in a smow amount of time

### whewe?
uw computew, σωσ if i-it has a wecent x86 cpu (intew, (˘ω˘) amd) that suppowts sse4.1

### why?
why nyot?

### h-how?
twdw: 128-bit s-simd vectowization pwus s-some big bwain awgos

<detaiws>
<summawy>cwick fow mowe info</summawy>
<p>

aftew houws of weseawch, rawr x3 i-i've finawwy undewstood the e-essence of uwu'd t-text

thewe awe a-a few twansfowmations:
1. OwO n-nyya-ify (eg. (///ˬ///✿) `nawuhodo` -> `nyawuhodo`)
2. -.- wepwace `w` a-and `w` with `w`
3. rawr x3 stuttew sometimes (`hi` -> `h-hi`)
4. -.- a-add a-a text emoji aftew p-punctuation (`,`, (˘ω˘) `.`, ow `!`) sometimes
5. σωσ wepwace some wowds (`smow` -> `smow`, (˘ω˘) e-etc.)

these twansfowmation p-passes take advantage of sse4.1 vectow intwinsics to pwocess 16 bytes at once. rawr x3
f-fow stwing seawching, (///ˬ///✿) i'm using a custom simd i-impwementation of the
[bitap](https://en.wikipedia.owg/wiki/bitap_awgowithm) awgowithm f-fow matching a-against muwtipwe s-stwings. (˘ω˘)
fow wandom nyumbew genewation, o.O i'm using [xowshift32](https://en.wikipedia.owg/wiki/xowshift). ( ͡o ω ͡o ) fow most
chawactew-wevew detection w-within simd wegistews, >w< i-its aww masking a-and shifting t-to simuwate b-basic state
machines i-in pawawwew

muwtithweading is suppowted, (U ﹏ U) so u-u can expwoit aww of uw cpu cowes f-fow the nyobwe goaw
of uwu-ing m-massive amounts o-of text

utf-8 is handwed ewegantwy by simpwy ignowing nyon-ascii c-chawactews in the input

unfowtunatewy, OwO due t-to both simd pawawwewism and muwtithweading, some wowds may nyot b-be fuwwy uwu'd
if they wewe wucky e-enough to cwoss t-the boundawy o-of a simd vectow o-ow a thwead's buffew. OwO
*they won't e-escape so easiwy n-nyext time*

</p>
</detaiws>

### o-ok i want uwu'd text, rawr x3 how d-do i wun this mysewf?
1. -.- instaww wust: wun `cuww h-https://sh.wustup.ws -ssf | s-sh` on unix, OwO
ow go [hewe](https://www.wust-wang.owg/toows/instaww) f-fow mowe options
2. (⑅˘꒳˘) wun `cawgo i-instaww uwuify`
3. UwU w-wun `uwuify` which wiww wead f-fwom stdin and output t-to stdout. (///ˬ///✿) m-make suwe u
pwess ctww + d (unix) o-ow ctww + z and entew (windows) a-aftew u type s-stuff in stdin to s-send an eof

if you awe having t-twoubwe wunning `uwuify`, ( ͡o ω ͡o ) m-make suwe you have `~/.cawgo/bin`
i-in y-youw `$path`

it i-is possibwe to w-wead and wwite fwom f-fiwes by specifying the input fiwe and
output f-fiwe, o.O in that owdew. UwU u can use `--hewp` f-fow mowe info

#### buiwd fwom this wepo
<detaiws>
<summawy>cwick fow mowe info</summawy>
<p>

1. (˘ω˘) instaww wust
2. (U ᵕ U❁) wun `git c-cwone https://github.com/daniew-wiu-c0deb0t/uwu.git && c-cd uwu`
3. ʘwʘ wun `cawgo wun --wewease`

##### t-testing
1. -.- w-wun `cawgo test`

##### b-benchmawking
1. σωσ wun `mkdiw test && cd t-test`

*wawning: wawge fiwes of 100mb a-and 1gb, UwU w-wespectivewy*

2. wun `cuww -ow h-http://mattmahoney.net/dc/enwik8.zip && u-unzip enwik8.zip`
3. σωσ w-wun `cuww -ow http://mattmahoney.net/dc/enwik9.zip && unzip enwik9.zip`
4. OwO wun `cd .. && ./bench.sh`

</p>
</detaiws>

### i don't b-bewieve that this is fast. OwO i nyeed p-pwoof!!1! o.O
twdw: c-can be awmost as fast as simpwy copying a fiwe

<detaiws>
<summawy>cwick f-fow m-mowe info</summawy>
<p>

waw nyumbews fwom wunning `./bench.sh` o-on a 2019 macbook pwo with eight
intew 2.3 ghz i9 cpus and 16 gb o-of wam awe shown bewow. (U ﹏ U) the dataset
u-used is the f-fiwst 100mb and f-fiwst 1gb of engwish wikipedia. σωσ the same
dataset i-is used fow the [huttew p-pwize](http://pwize.huttew1.net/)
fow t-text compwession

```
1 t-thwead uwu enwik8
time taken: 178 ms
input s-size: 100000000 bytes
output size: 115095591 bytes
thwoughput: 0.55992 gb/s

2 thwead uwu enwik8
t-time taken: 105 ms
input size: 100000000 bytes
output size: 115095591 bytes
t-thwoughput: 0.94701 g-gb/s

4 thwead u-uwu enwik8
time t-taken: 60 ms
i-input size: 100000000 bytes
output s-size: 115095591 b-bytes
thwoughput: 1.64883 g-gb/s

8 thwead uwu enwik8
time taken: 47 m-ms
input size: 100000000 bytes
o-output size: 115095591 bytes
t-thwoughput: 2.12590 g-gb/s

copy enwik8

weaw	0m0.035s
usew	0m0.001s
sys	0m0.031s

1 thwead uwu e-enwik9
time taken: 2087 m-ms
input size: 1000000000 b-bytes
output size: 1149772651 b-bytes
thwoughput: 0.47905 gb/s

2 t-thwead uwu enwik9
time taken: 992 ms
input size: 1000000000 bytes
output size: 1149772651 b-bytes
thwoughput: 1.00788 g-gb/s

4 thwead uwu enwik9
time taken: 695 ms
input size: 1000000000 bytes
output size: 1149772651 bytes
thwoughput: 1.43854 gb/s

8 thwead uwu enwik9
time taken: 436 ms
input size: 1000000000 b-bytes
output size: 1149772651 b-bytes
thwoughput: 2.29214 gb/s

copy enwik9

w-weaw	0m0.387s
usew	0m0.001s
sys	0m0.341s
```

*//todo: c-compawe with othew toows*

</p>
</detaiws>

### w-why isn't t-this weadme uwu'd?
so its weadabwe

i-if u happen t-to find uwu'd t-text mowe weadabwe, t-thewe's awways an [uwu'd](weadme_uwu.txt) v-vewsion

### o-ok but why awen't thewe any of the settings i can change?!1?!!1
fwee w-wiww is an iwwusion

### w-wtf this is so unpwofessionaw how awe u gonna get hiwed a-at faang nyow?! ʘwʘ
d-don't wowwy, (U ﹏ U) i've got u covewed

#### t-titwe: uwu is aww you nyeed

#### abstwact

w-wecent advances in computing h-have made stwides in pawawwewization, (ꈍᴗꈍ) whethew
at a fine-gwained w-wevew with simd i-instwuctions, -.- ow a-at a high wevew with muwtipwe
cpu cowes. taking advantage of these advances, o.O we e-expwowe how the u-usefuw
task of p-pewfowming an uwu t-twansfowmation on pwain text can be scawed up to wawge
input datasets. (⑅˘꒳˘) ouw contwibutions i-in this p-papew awe thweefowd: fiwst, ( ͡o ω ͡o ) we p-pwesent,
to ouw k-knowwedge, (///ˬ///✿) the fiwst wigowous d-definition of uwu'd t-text. >w< second, w-we show
ouw nyovew awgowithms fow uwu-ing text, σωσ e-expwoiting vectowization a-and
muwtithweading f-featuwes t-that awe a-avaiwabwe on modewn cpus. finawwy, o.O we pwovide
wigowous e-expewimentaw w-wesuwts that s-show how ouw impwementation couwd be the
"fastest i-in the west." i-in ouw benchmawks, -.- w-we obsewve that o-ouw impwementation
w-was awmost as a fast as a s-simpwe fiwe copy, o.O w-which is entiwewy io-bound. ( ͡o ω ͡o )
we b-bewieve ouw wowk has potentiaw a-appwications in vawious domains, f-fwom data
augmentation and text p-pwepwocessing fow nyatuwaw wanguage p-pwocessing, o.O to
giving authows the abiwity t-to convey potentiawwy w-whowesome ow kawaii~ meme messages
with minimaw t-time and effowt. (U ﹏ U)

*// todo: wwite papew*

*// todo: wwite mowe about machine weawning so i g-get funding*

### o-ok i nyeed to u-use this fow something a-and i need t-the wicense info
mit wicense

### ok but i have a-an issue with t-this ow a suggestion ow a question n-nyot answewed hewe
open an issue, b-be nyice

### wefewences
* h-https://honk.moe/toows/owo.htmw
* https://github.com/iamwifki/uwuizew
* h-https://github.com/deadshot465/owoify_ws
* h-https://kawaii~kaomoji.com/chawactews/uwu/
* h-https://kawaii~kaomoji.com/chawactews/owo/
* https://kawaii~kaomoji.com/chawactews/fwowew-giww/
* a-and many mowe; w-wet me know if i-i missed anything
