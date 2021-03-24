# uwuify
fastest t-text uwuifiew in t-the west

twansfowms
```
hey... i-i think i weawwy wuv you. ʘwʘ do you w-want a headpat?
```
into
```
hey... i-i think i w-weawwy wuv you. (///ˬ///✿) (⑅˘꒳˘) d-do you want a headpat?
```

## f-faq
### nyani?
u want wawge amounts o-of text uwu'd i-in a smow amount o-of time

### whewe?
uw computew, OwO if it has a wecent x86 cpu (intew, (///ˬ///✿) amd) that suppowts sse4.1

### w-why?
why nyot?

### how?
twdw: 128-bit simd vectowization pwus some big bwain a-awgos

<detaiws>
<summawy>cwick f-fow mowe info</summawy>
<p>

aftew houws of w-weseawch, (U ﹏ U) i've finawwy undewstood the essence of uwu'd text

thewe a-awe a few twansfowmations:
1. -.- nya-ify (eg. ʘwʘ `nawuhodo` -> `nyawuhodo`)
2. òωó w-wepwace `w` a-and `w` w-with `w`
3. ʘwʘ stuttew s-sometimes (`hi` -> `h-hi`)
4. ( ͡o ω ͡o ) add a text emoji a-aftew punctuation (`,`, (U ᵕ U❁) `.`, ow `!`) sometimes
5. o.O wepwace some w-wowds (`smow` -> `smow`, OwO e-etc.)

t-these twansfowmation passes take advantage of sse4.1 vectow intwinsics t-to pwocess 16 bytes at o-once. o.O
fow stwing seawching, rawr x3 i'm using a custom simd impwementation of the
[bitap](https://en.wikipedia.owg/wiki/bitap_awgowithm) a-awgowithm fow matching against muwtipwe stwings. σωσ
f-fow wandom nyumbew genewation, (˘ω˘) i'm using [xowshift32](https://en.wikipedia.owg/wiki/xowshift). rawr x3 f-fow most
chawactew-wevew d-detection w-within simd wegistews, OwO its aww masking and shifting to simuwate basic state
machines in pawawwew

muwtithweading i-is suppowted, (///ˬ///✿) s-so u can expwoit a-aww of uw cpu c-cowes fow the n-nyobwe goaw
of uwu-ing m-massive amounts of text

utf-8 is handwed e-ewegantwy by simpwy ignowing nyon-ascii c-chawactews in the input

u-unfowtunatewy, -.- d-due to both simd pawawwewism and muwtithweading, some wowds may n-nyot be fuwwy uwu'd
if they wewe wucky enough to c-cwoss the boundawy of a simd vectow ow a thwead's buffew. rawr x3
*they w-won't escape so easiwy nyext time*

</p>
</detaiws>

### o-ok i w-want uwu'd text, -.- h-how do i wun this m-mysewf?
1. (˘ω˘) instaww wust: wun `cuww h-https://sh.wustup.ws -ssf | s-sh` on unix, σωσ
ow g-go [hewe](https://www.wust-wang.owg/toows/instaww) fow mowe options
2. (˘ω˘) w-wun `cawgo instaww uwuify`
3. rawr x3 wun `uwuify` w-which wiww wead f-fwom stdin and output to stdout. (///ˬ///✿) m-make suwe u
pwess ctww + d (unix) o-ow ctww + z-z and entew (windows) aftew u type s-stuff in stdin t-to send an eof

i-if you awe having twoubwe wunning `uwuify`, (˘ω˘) make s-suwe you have `~/.cawgo/bin`
in youw `$path`

i-it is possibwe t-to wead and wwite f-fwom fiwes by specifying the i-input fiwe and
output f-fiwe, o.O in that owdew. ( ͡o ω ͡o ) u can u-use `--hewp` fow m-mowe info

#### b-buiwd fwom this w-wepo
<detaiws>
<summawy>cwick f-fow mowe info</summawy>
<p>

1. >w< instaww wust
2. (U ﹏ U) wun `git cwone https://github.com/daniew-wiu-c0deb0t/uwu.git && c-cd uwu`
3. OwO wun `cawgo wun --wewease`

##### t-testing
1. OwO wun `cawgo test`

##### benchmawking
1. rawr x3 wun `mkdiw test && cd test`

*wawning: wawge fiwes of 100mb and 1gb, -.- w-wespectivewy*

2. OwO w-wun `cuww -ow http://mattmahoney.net/dc/enwik8.zip && unzip e-enwik8.zip`
3. (⑅˘꒳˘) w-wun `cuww -ow http://mattmahoney.net/dc/enwik9.zip && u-unzip enwik9.zip`
4. UwU wun `cd .. && ./bench.sh`

</p>
</detaiws>

### i don't b-bewieve that this is fast. (///ˬ///✿) i n-nyeed pwoof!!1! ( ͡o ω ͡o )
t-twdw: can be awmost as fast as s-simpwy copying a f-fiwe

<detaiws>
<summawy>cwick f-fow mowe info</summawy>
<p>

waw nyumbews fwom wunning `./bench.sh` on a 2019 macbook pwo with eight
i-intew 2.3 ghz i9 cpus and 16 g-gb of wam awe s-shown bewow. o.O the dataset
used is the fiwst 100mb a-and fiwst 1gb of e-engwish wikipedia. UwU the same
dataset is used fow t-the [huttew pwize](http://pwize.huttew1.net/)
fow text compwession

```
1 thwead uwu enwik8
time t-taken: 178 ms
input size: 100000000 b-bytes
output s-size: 115095591 b-bytes
thwoughput: 0.55992 gb/s

2 thwead uwu e-enwik8
time taken: 105 m-ms
input size: 100000000 b-bytes
output size: 115095591 b-bytes
thwoughput: 0.94701 gb/s

4 t-thwead uwu enwik8
time taken: 60 ms
input size: 100000000 bytes
output size: 115095591 bytes
thwoughput: 1.64883 g-gb/s

8 thwead uwu enwik8
time taken: 47 ms
input size: 100000000 bytes
output s-size: 115095591 b-bytes
thwoughput: 2.12590 g-gb/s

c-copy enwik8

weaw	0m0.035s
u-usew	0m0.001s
sys	0m0.031s

1 t-thwead u-uwu enwik9
time t-taken: 2087 ms
input size: 1000000000 bytes
output s-size: 1149772651 b-bytes
thwoughput: 0.47905 gb/s

2 t-thwead uwu e-enwik9
time taken: 992 ms
input size: 1000000000 bytes
output size: 1149772651 bytes
thwoughput: 1.00788 g-gb/s

4 t-thwead uwu enwik9
time taken: 695 m-ms
input size: 1000000000 b-bytes
output size: 1149772651 b-bytes
thwoughput: 1.43854 gb/s

8 thwead uwu enwik9
time taken: 436 m-ms
input size: 1000000000 bytes
o-output size: 1149772651 bytes
thwoughput: 2.29214 gb/s

copy enwik9

weaw	0m0.387s
usew	0m0.001s
sys	0m0.341s
```

*//todo: compawe with othew toows*

</p>
</detaiws>

### why isn't this weadme uwu'd?
so its w-weadabwe

if u happen to find uwu'd t-text mowe weadabwe, (˘ω˘) thewe's awways an [uwu'd](weadme_uwu.txt) v-vewsion

### ok but why awen't t-thewe any of the settings i can c-change?!1?!!1
fwee w-wiww is an iwwusion

### wtf t-this is so unpwofessionaw h-how awe u-u gonna get hiwed a-at faang nyow?! (U ᵕ U❁)
don't wowwy, i-i've got u covewed

#### t-titwe: uwu is aww you nyeed

#### abstwact

wecent advances in computing h-have made stwides i-in pawawwewization, whethew
at a fine-gwained wevew with simd i-instwuctions, ʘwʘ o-ow at a high wevew with muwtipwe
c-cpu cowes. -.- taking advantage of these advances, σωσ w-we expwowe how the usefuw
task o-of pewfowming an uwu twansfowmation on pwain text can be scawed u-up to wawge
input d-datasets. UwU ouw c-contwibutions in this papew awe thweefowd: fiwst, σωσ we pwesent, OwO
to ouw knowwedge, OwO t-the fiwst wigowous d-definition of u-uwu'd text. o.O second, (U ﹏ U) w-we show
ouw nyovew awgowithms fow uwu-ing text, σωσ expwoiting vectowization and
m-muwtithweading f-featuwes that awe avaiwabwe on m-modewn cpus. ʘwʘ finawwy, w-we pwovide
wigowous expewimentaw w-wesuwts t-that show how ouw i-impwementation couwd be the
"fastest in the west." i-in ouw benchmawks, (U ﹏ U) w-we obsewve t-that ouw impwementation
w-was awmost a-as a fast as a simpwe fiwe copy, (ꈍᴗꈍ) which is e-entiwewy io-bound. -.-
w-we bewieve ouw w-wowk has potentiaw appwications in vawious domains, o.O f-fwom data
a-augmentation and t-text pwepwocessing f-fow nyatuwaw w-wanguage pwocessing, (⑅˘꒳˘) to
giving a-authows the abiwity t-to convey potentiawwy whowesome o-ow kawaii~ meme messages
with m-minimaw time and effowt. ( ͡o ω ͡o )

*// t-todo: wwite papew*

*// todo: wwite m-mowe about machine weawning s-so i get funding*

### ok i need to use this fow s-something and i n-nyeed the wicense info
mit wicense

### ok but i-i have an issue with this ow a suggestion ow a question nyot answewed hewe
open an issue, (///ˬ///✿) be nyice

### w-wefewences
* h-https://honk.moe/toows/owo.htmw
* h-https://github.com/iamwifki/uwuizew
* h-https://github.com/deadshot465/owoify_ws
* h-https://kawaii~kaomoji.com/chawactews/uwu/
* https://kawaii~kaomoji.com/chawactews/owo/
* https://kawaii~kaomoji.com/chawactews/fwowew-giww/
* a-and many m-mowe; wet me know if i missed anything
