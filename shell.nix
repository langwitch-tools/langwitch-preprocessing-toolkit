with import <nixpkgs> {};
mkShell {
  packages = [
    rust-script
    rustup
    cargo

    deno

    openssl
    cacert
    libiconv

    xz
    gzip
    bzip2
    curlFull
    jq
  ];
  buildInputs = [] ++ lib.optionals stdenv.isDarwin [ 
      darwin.apple_sdk.frameworks.IOKit
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.CoreServices
      darwin.apple_sdk.frameworks.CoreFoundation
  ] ;
}