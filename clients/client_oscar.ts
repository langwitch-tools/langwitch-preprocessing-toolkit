import { createMain, dumpData } from "./client_proc_utils.ts";
import M from "https://esm.sh/monet@0.9.3";

export const getUrls = (langcode: string): Promise<M.Maybe<string[]>> =>
	fetch(
		`https://s3.amazonaws.com/datasets.huggingface.co/oscar/1.0/unshuffled/deduplicated/${langcode}/${langcode}_sha256.txt`
	)
		.then((r) => r.text())
		.then((t) =>
			t.includes("Error")
				? M.Maybe.Nothing()
				: M.Maybe.of(
						t
							.split("\n")
							.filter((s) => s.includes("\t"))
							.map((u) => u.split("\t")[0])
							.map(
								(s) =>
									`https://s3.amazonaws.com/datasets.huggingface.co/oscar/1.0/unshuffled/deduplicated/${langcode}/` +
									s
							)
				  )
		);

/*procPipe([
    proc(["seq", "1", "10"]),
    proc(["./deduplicate.rs"])
])().stdout.readable.pipeTo(Deno.stdout.writable);*/
export const main = createMain({
	getUrls,
	decompressor: dumpData("curl -s %url | gunzip -cd"),
	err: () => console.log("Couldn't find that language code")
});
/*
- OPUS
- CommonCrawl from statmt
- MC4 from huggingface
- oscar corpus
- direct commoncrawl url pattern-searching
- 

*/
