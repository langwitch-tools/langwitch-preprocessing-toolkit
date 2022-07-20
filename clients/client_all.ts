#!/usr/bin/env nix-shell #! nix-shell -i "deno run -A" -p deno jq curl xz gzip

import * as mc4 from "./client_mc4.ts";
import * as oscar from "./client_oscar.ts";
import * as statmt from "./client_statmt.ts";
import * as opus from "./client_opus.ts";
import { parse } from "./client_deps.ts";
const args = parse(Deno.args);

const main = {
    "mc4": mc4.main,
    "oscar": oscar.main,
    "statmt": statmt.main,
    "opus": opus.main
}[args["source"] as string];

if (main === undefined) {
    console.error("Needs to be mc4, oscar, statmt, or opus");
    Deno.exit(1);
}
//console.log(Deno.stdout.writable);
await main(args["lang"]);
Deno.exit(1);