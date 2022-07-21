![langwitch-header](https://user-images.githubusercontent.com/48640397/179381158-1af681c4-95c2-45e6-9bc5-30abb82d495e.png)

The scripts in this repo are what allowed me to generate 26 billion bitexts in a few days on a 2015 iMac.

Through filtering, they allow you to turn this:

```
src)="50"> [ Booking ]
(trg)="50"> [ Захиалах ]
(src)="51"> Tel : + 886-3-4698023
(trg)="51"> Тэлефон : + 886-3-4698023
(src)="52"> MESSAGE Core Message DATE 2019.09.08
(trg)="52"> Послание ¬ ° ¬ г ¬ Я ¬ а ¬ У ¬ Я ¬ а ¬ Ц ¬ б ¬ а ¬ г ¬ Э ¬ С ¬ Я ¬ Ъ ¬ Ц Дата 2019.07.07
(src)="53"> Benedetta – blessed ,
(trg)="53"> насны хүүхдийн өвчин
(src)="54"> Outstanding !
(trg)="54"> Өд шиг хөнгөн хөдөлгөөн
(src)="55"> Duration 00 : 11 : 55
(trg)="55"> Үргэлжлэх хугацаа 00 : 11 : 55
(src)="56"> Tel : 0086-22-26994727
(trg)="56"> Тэл : 0086-22-26994727
(src)="57"> Who are those who actively follow the Dharma ,
(trg)="57"> Төвдийн Үндэсний Бослого Хөдөлгөөний 52 жилийн ойд зориулж хэлсэн үг
```

Into this:

```
There is no cure yet.	Одоогоор эмчлэх аргийг нь олоогүй байга.
I was elected as a citizen's representative from Songinokhairkhan district.	Сонгинохайрхан дүүргээс би иргэдийн төлөөлөгчөөр сонгогдсон.
How much money did we blow?	Хэдэн төгрөгийг бид агаарт хийсгэсэн вэ?
Genetic engineering experiments should be banned worldwide.	Гений инженерийн туршилтыг дэлхий нийтээр хориглох нь зүйтэй.
The government is a place where you measure and cut.	Засгийн газар бол долоо хэмжиж нэг огтолдог л газар.
Universal truth does not depend on anyone.	Түгээмэл үнэн хэн нэгнээс хамаарч оршдоггүй юм.
Note that a lot of bacteria remain on the walls of the sink.	Угаагуурын хананд маш их хэмжээний нян үлдэж байдгийг анхаарна уу.
Recently, the Mongolians were happy to add one more thing to fix.	Саяхан монголчууд засч залруулах юмаа нэгээр нэмж хүүхэд өтгөсгүй баярлав.
The case related to Mungantuya was dismissed and he was acquitted.	Мөнгөнтуяад холбогдох хэргийг хэрэгсэхгүй болгож, түүнийг цагаатгасан юм.
Calcium can be found in many foods today.	Кальцийг бид өнөөдөр олон хоол хүнсэнд олж болно.
```

I am not a patient person, and these scripts reflect that fact. I made them with several expectations:

* I shouldn't need to wait around for imports and dependencies to load. A script should start instantly, and stop instantly.
* A script should have a throughput of at least 100 Mb/s. Any less than that is a waste of time, and should be justified.
* A script should use, at the very most, 15Mb of memory.
* A script should just take input from stdin, flush to stdout, and accept simple, fully declarative flags.
* A script should not make assumptions about what I want. It should fail, loudly and instantly, on invalid input.
* A script should be an atomic building-block for pipelines of data, and nothing else.

Most of the things in this repo are pretty boring. Like:

- clean.rs - a bulk-standard text normaliser, throughput of around 90-120Mb/s
- deduplicate.rs - a line-by-line deduplicator, throughput of around 230-300Mb/s
- lengthfilter.rs - you can guess this one, throughput of around 300-340Mb/s
- nosame.rs - checks translation-pairs for co-occurring words on either side, throughput of around 80-110Mb/s

The more interesting ones are:

- translate.rs - uses a nifty backdoor to Google's NMT service and hammers it with 200 async workers. Ultimately limited by your network speed and Google's capacity for requests, but usually around 4.8 million chars / second. Likely the fastest available.
- wikunzip.rs - processes raw XML dumps, converts the mediawiki format to plain-text. This one is pretty unreliable, it wavers between 100 and 270Mb/s. Also likely the fastest available, but that's mostly thanks to the libraries I'm using.
- sort.rs - this one is slower. It uses a bundle of heuristics to rank sentences by their quality using an absolute threshold, and it works surprisingly well. Throughput of around 50-60Mb/s.
- langfilter.rs - this one is ridiculously slow, it can only tag and identify 5k documents/s. Albeit, those "documents" can be 100Mb without any appreciable performance drop, so it's tricky to attach a number to it. It's an easily extensible language detector and implicit quality-filterer with pretty darn good accuracy, that requires no pretraining, and can identify a language reliably with a single 1.2Kb file as a reference. Its memory usage stays constant at around 12Mb whether you're comparing each sentence against 20 languages or 500, which is nice.

Deno scripts:

- still writing this
