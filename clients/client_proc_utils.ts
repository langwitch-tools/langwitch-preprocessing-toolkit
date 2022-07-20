import { R } from "./client_deps.ts";
import M from "https://esm.sh/monet@0.9.3";
import { shCapture } from "https://deno.land/x/drake@v1.5.0/mod.ts"; 

export const proc = (args: string[]) => () =>
	Deno.run({ cmd: args, stdin: "piped", stdout: "piped" });
export const aPipe = R.pipeWith((f, res) => Promise.resolve(res).then(f));

export const procPipe = R.pipeWith(
	(
		f,
		res:
			| undefined
			| Deno.Process<{ cmd: string[]; stdin: "piped"; stdout: "piped" }>
	) => {
		const newProc = f() as Deno.Process<{
			cmd: string[];
			stdin: "piped";
			stdout: "piped";
		}>;
		res ? res.stdout.readable.pipeTo(newProc.stdin.writable) : null;
		return newProc;
	}
);

export const dumpData =
	(command: string) => (urls: string[]) =>
		urls.map((u) => () => {
			console.log(command.replace("%url", u));
		});

interface Fetcher<Code extends string> {
	getUrls: (c: Code) => Promise<M.Maybe<string[]>> | M.Maybe<string[]>;
	decompressor: (urls: string[]) => (() => void)[];
	err: () => void;
}
export const createMain =
	<C extends string>(f: Fetcher<C>) =>
	async (code: C) => {
		const urls = await f.getUrls(code);
		urls.map(f.decompressor)
			.map(d => {
				for (const s of d) {
					s();
				}
				return {};
            })
			.orElseRun(f.err);
	};
