import { createMain, dumpData } from "./client_proc_utils.ts";
import M from "https://esm.sh/monet@0.9.3";
export interface OpusDoc {
	sourceTokens: number; //i.e words
	url: string;
	alignmentPairs: number; //i.e sentences
	source: string; //i.e language code
	corpus: string; //something like 'Wikipedia'
	latest: string; //i.e Date
	target: string;
}

type CamelToSnake<T extends string, P extends string = ""> = string extends T
	? string
	: T extends `${infer C0}${infer R}`
	? CamelToSnake<
			R,
			`${P}${C0 extends Lowercase<C0> ? "" : "_"}${Lowercase<C0>}`
	  >
	: P;

type SnakeCasedKeys<T> = {
	[K in keyof T as CamelToSnake<string & K>]: T[K] extends {}
		? SnakeCasedKeys<T[K]>
		: T[K];
};
export type StringFields<T> = T extends { [K in keyof T]: T[K] }
	? { [K in keyof T]: string }
	: never;

export type QueryStringParam = `&${string}=${string}`;
export const getCorpora =
	(extras: "" | QueryStringParam = "") =>
	(langCode: string) =>
		fetch(
			`https://opus.nlpl.eu/opusapi/?target=${langCode}${extras}&preprocessing=mono&version=latest`
		)
			.then((r) => r.json())
			.then(
				(r): Promise<OpusDoc[]> =>
					r["corpora"]
						.map((e: SnakeCasedKeys<StringFields<OpusDoc>>) => ({
							sourceTokens: Number(e["source_tokens"]), //i.e words
							url: e["url"],
							alignmentPairs: Number(e["alignment_pairs"]), //i.e sentences
							source: e["source"], //i.e language code
							corpus: e["corpus"], //something like 'Wikipedia'
							latest: e["latest"], //i.e Date
							target: e["target"],
						}))
						.filter((e: OpusDoc) => e.url.includes(".txt.gz"))
			);

export const main = createMain({
    getUrls: (c: string) => getCorpora()(c).then(c => c == [] ? M.Maybe.None() : M.Maybe.of(c.map(i => i["url"]))),
    decompressor: dumpData("curl -s -k -L %url | gunzip -cd"),
    err: () => console.log("Couldn't find that language")
});