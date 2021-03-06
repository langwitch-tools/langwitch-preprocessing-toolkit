import { createMain, dumpData } from "./client_proc_utils.ts";
import M from "https://esm.sh/monet@0.9.3";
import { R } from "./client_deps.ts";

const shards = {
	af: { train: 64, validation: 1 },
	am: { train: 16, validation: 1 },
	ar: { train: 1024, validation: 4 },
	az: { train: 256, validation: 1 },
	be: { train: 128, validation: 1 },
	bg: { train: 1024, validation: 1 },
	"bg-Latn": { train: 4, validation: 1 },
	bn: { train: 512, validation: 1 },
	ca: { train: 512, validation: 1 },
	ceb: { train: 8, validation: 1 },
	co: { train: 8, validation: 1 },
	cs: { train: 1024, validation: 2 },
	cy: { train: 256, validation: 1 },
	da: { train: 1024, validation: 1 },
	de: { train: 2048, validation: 16 },
	el: { train: 1024, validation: 2 },
	"el-Latn": { train: 16, validation: 1 },
	en: { train: 11264, validation: 128 },
	eo: { train: 32, validation: 1 },
	es: { train: 2048, validation: 16 },
	et: { train: 256, validation: 1 },
	eu: { train: 64, validation: 1 },
	fa: { train: 1024, validation: 2 },
	fi: { train: 1024, validation: 1 },
	fil: { train: 64, validation: 1 },
	fr: { train: 2048, validation: 16 },
	fy: { train: 16, validation: 1 },
	ga: { train: 16, validation: 1 },
	gd: { train: 16, validation: 1 },
	gl: { train: 128, validation: 1 },
	gu: { train: 64, validation: 1 },
	ha: { train: 8, validation: 1 },
	haw: { train: 2, validation: 1 },
	hi: { train: 1024, validation: 2 },
	"hi-Latn": { train: 16, validation: 1 },
	hmn: { train: 8, validation: 1 },
	ht: { train: 8, validation: 1 },
	hu: { train: 1024, validation: 2 },
	hy: { train: 128, validation: 1 },
	id: { train: 1024, validation: 4 },
	ig: { train: 4, validation: 1 },
	is: { train: 128, validation: 1 },
	it: { train: 1024, validation: 8 },
	iw: { train: 1024, validation: 1 },
	ja: { train: 1024, validation: 8 },
	"ja-Latn": { train: 8, validation: 1 },
	jv: { train: 8, validation: 1 },
	ka: { train: 256, validation: 1 },
	kk: { train: 256, validation: 1 },
	km: { train: 64, validation: 1 },
	kn: { train: 64, validation: 1 },
	ko: { train: 1024, validation: 1 },
	ku: { train: 16, validation: 1 },
	ky: { train: 64, validation: 1 },
	la: { train: 64, validation: 1 },
	lb: { train: 32, validation: 1 },
	lo: { train: 8, validation: 1 },
	lt: { train: 512, validation: 1 },
	lv: { train: 256, validation: 1 },
	mg: { train: 8, validation: 1 },
	mi: { train: 4, validation: 1 },
	mk: { train: 128, validation: 1 },
	ml: { train: 128, validation: 1 },
	mn: { train: 128, validation: 1 },
	mr: { train: 1024, validation: 1 },
	ms: { train: 512, validation: 1 },
	mt: { train: 128, validation: 1 },
	my: { train: 64, validation: 1 },
	ne: { train: 256, validation: 1 },
	nl: { train: 1024, validation: 4 },
	no: { train: 1024, validation: 1 },
	ny: { train: 4, validation: 1 },
	pa: { train: 32, validation: 1 },
	pl: { train: 1024, validation: 4 },
	ps: { train: 16, validation: 1 },
	pt: { train: 1024, validation: 4 },
	ro: { train: 1024, validation: 2 },
	ru: { train: 4096, validation: 32 },
	"ru-Latn": { train: 32, validation: 1 },
	sd: { train: 64, validation: 1 },
	si: { train: 64, validation: 1 },
	sk: { train: 512, validation: 1 },
	sl: { train: 256, validation: 1 },
	sm: { train: 4, validation: 1 },
	sn: { train: 8, validation: 1 },
	so: { train: 64, validation: 1 },
	sq: { train: 128, validation: 1 },
	sr: { train: 256, validation: 1 },
	st: { train: 2, validation: 1 },
	su: { train: 4, validation: 1 },
	sv: { train: 1024, validation: 2 },
	sw: { train: 32, validation: 1 },
	ta: { train: 256, validation: 1 },
	te: { train: 128, validation: 1 },
	tg: { train: 64, validation: 1 },
	th: { train: 1024, validation: 1 },
	tr: { train: 1024, validation: 4 },
	uk: { train: 1024, validation: 2 },
	und: { train: 3072, validation: 32 },
	ur: { train: 128, validation: 1 },
	uz: { train: 32, validation: 1 },
	vi: { train: 1024, validation: 4 },
	xh: { train: 2, validation: 1 },
	yi: { train: 16, validation: 1 },
	yo: { train: 2, validation: 1 },
	zh: { train: 1024, validation: 2 },
	"zh-Latn": { train: 8, validation: 1 },
	zu: { train: 8, validation: 1 },
};

export type MC4LangCode = keyof typeof shards;

export const getUrls = (c: MC4LangCode): M.Maybe<string[]> =>
	shards[c]
		? M.Maybe.of(
				R.range(1)(shards[c]["train"])
					.map((n) => String(n).padStart(5, "0"))
					.map(
						(n) =>
							`https://huggingface.co/datasets/allenai/c4/resolve/1ddc917116b730e1859edef32896ec5c16be51d0/multilingual/c4-${c}.tfrecord-${n}-of-${String(
								shards[c]["train"]
							).padStart(5, "0")}.json.gz`
					)
		  )
		: M.Maybe.None();

export const main = createMain({
    decompressor: dumpData("curl -s -k -L %url | gunzip -cd | jq '.text'"),
    getUrls,
    err: () => console.log("Couldn't find that language")
});
// also requires jq '.text'
