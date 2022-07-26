import { dumpData, createMain } from "./client_proc_utils.ts";
import M from "https://esm.sh/monet@0.9.3";
export type StatMTCode =
	| "aa"
	| "ab"
	| "af"
	| "ak"
	| "am"
	| "ar"
	| "as"
	| "ay"
	| "az"
	| "ba"
	| "be"
	| "bg"
	| "bh"
	| "bi"
	| "blu"
	| "bn"
	| "bo"
	| "br"
	| "bs"
	| "ca"
	| "ceb"
	| "chr"
	| "co"
	| "crs"
	| "cs"
	| "cy"
	| "da"
	| "de"
	| "dv"
	| "dz"
	| "ee"
	| "el"
	| "eo"
	| "es"
	| "et"
	| "eu"
	| "fa"
	| "fi"
	| "fj"
	| "fo"
	| "fr"
	| "fy"
	| "gaa"
	| "ga"
	| "gd"
	| "gl"
	| "gn"
	| "gu"
	| "gv"
	| "ha"
	| "haw"
	| "hi"
	| "hr"
	| "ht"
	| "hu"
	| "hy"
	| "ia"
	| "id"
	| "ie"
	| "ig"
	| "ik"
	| "is"
	| "it"
	| "iu"
	| "iw"
	| "ja"
	| "jw"
	| "ka"
	| "kha"
	| "kk"
	| "kl"
	| "km"
	| "kn"
	| "ko"
	| "kri"
	| "ks"
	| "ku"
	| "ky"
	| "la"
	| "lb"
	| "lg"
	| "lif"
	| "ln"
	| "lo"
	| "loz"
	| "lt"
	| "lua"
	| "luo"
	| "lv"
	| "mfe"
	| "mg"
	| "mi"
	| "mk"
	| "ml"
	| "mn"
	| "mr"
	| "ms"
	| "mt"
	| "my"
	| "na"
	| "ne"
	| "new"
	| "nl"
	| "nn"
	| "no"
	| "nr"
	| "nso"
	| "ny"
	| "oc"
	| "om"
	| "or"
	| "os"
	| "pa"
	| "pam"
	| "pl"
	| "pt"
	| "raj"
	| "rm"
	| "rn"
	| "ro"
	| "ru"
	| "rw"
	| "sa"
	| "sco"
	| "sd"
	| "sg"
	| "si"
	| "sk"
	| "sl"
	| "sm"
	| "so"
	| "sq"
	| "sr"
	| "sr-ME"
	| "ss"
	| "st"
	| "su"
	| "sv"
	| "sw"
	| "syr"
	| "ta"
	| "te"
	| "th"
	| "ti"
	| "tk"
	| "tl"
	| "tlh"
	| "tn"
	| "tr"
	| "ts"
	| "tum"
	| "tw"
	| "ug"
	| "uk"
	| "un"
	| "ur"
	| "uz"
	| "ve"
	| "vi"
	| "vo"
	| "war"
	| "wo"
	| "xh"
	| "yi"
	| "za"
	| "zh"
	| "zh-Hant"
	| "zu"
	| "zzb"
	| "zze"
	| "zzh"
	| "zzp";

export const statMtCodes = new Set([
	"aa",
	"ab",
	"af",
	"ak",
	"am",
	"ar",
	"as",
	"ay",
	"az",
	"ba",
	"be",
	"bg",
	"bh",
	"bi",
	"blu",
	"bn",
	"bo",
	"br",
	"bs",
	"ca",
	"ceb",
	"chr",
	"co",
	"crs",
	"cs",
	"cy",
	"da",
	"de",
	"dv",
	"dz",
	"ee",
	"el",
	"eo",
	"es",
	"et",
	"eu",
	"fa",
	"fi",
	"fj",
	"fo",
	"fr",
	"fy",
	"gaa",
	"ga",
	"gd",
	"gl",
	"gn",
	"gu",
	"gv",
	"ha",
	"haw",
	"hi",
	"hr",
	"ht",
	"hu",
	"hy",
	"ia",
	"id",
	"ie",
	"ig",
	"ik",
	"is",
	"it",
	"iu",
	"iw",
	"ja",
	"jw",
	"ka",
	"kha",
	"kk",
	"kl",
	"km",
	"kn",
	"ko",
	"kri",
	"ks",
	"ku",
	"ky",
	"la",
	"lb",
	"lg",
	"lif",
	"ln",
	"lo",
	"loz",
	"lt",
	"lua",
	"luo",
	"lv",
	"mfe",
	"mg",
	"mi",
	"mk",
	"ml",
	"mn",
	"mr",
	"ms",
	"mt",
	"my",
	"na",
	"ne",
	"new",
	"nl",
	"nn",
	"no",
	"nr",
	"nso",
	"ny",
	"oc",
	"om",
	"or",
	"os",
	"pa",
	"pam",
	"pl",
	"pt",
	"raj",
	"rm",
	"rn",
	"ro",
	"ru",
	"rw",
	"sa",
	"sco",
	"sd",
	"sg",
	"si",
	"sk",
	"sl",
	"sm",
	"so",
	"sq",
	"sr",
	"sr-ME",
	"ss",
	"st",
	"su",
	"sv",
	"sw",
	"syr",
	"ta",
	"te",
	"th",
	"ti",
	"tk",
	"tl",
	"tlh",
	"tn",
	"tr",
	"ts",
	"tum",
	"tw",
	"ug",
	"uk",
	"un",
	"ur",
	"uz",
	"ve",
	"vi",
	"vo",
	"war",
	"wo",
	"xh",
	"yi",
	"za",
	"zh",
	"zh-Hant",
	"zu",
	"zzb",
	"zze",
	"zzh",
	"zzp",
]);

const dates = [
	".2012.raw.xz",
	".2013_20.raw.xz",
	".2013_48.raw.xz",
	".2014_15.raw.xz",
	".2014_23.raw.xz",
	".2014_35.raw.xz",
	".2014_41.raw.xz",
	".2014_42.raw.xz",
	".2014_49.raw.xz",
	".2014_52.raw.xz",
	".2015_06.raw.xz",
	".2015_11.raw.xz",
	".2015_14.raw.xz",
	".2015_18.raw.xz",
	".2015_22.raw.xz",
	".2015_27.raw.xz",
	".2015_32.raw.xz",
	".2015_35.raw.xz",
	".2015_40.raw.xz",
	".2015_48.raw.xz",
	".2016_30.raw.xz",
	".2016_50.raw.xz",
	".2017_17.raw.xz",
];

export const main = createMain({
    getUrls: (code: StatMTCode): M.Maybe<string[]> =>
	statMtCodes.has(code)
		? M.Maybe.of(dates.map((d) => `http://web-language-models.s3-website-us-east-1.amazonaws.com/ngrams/${code}/raw/` + code + d))
		: M.Maybe.None(),
    decompressor: dumpData("curl -s -k %url | xz -cd"),
    err: () => console.log("Couldn't find that code")
})
