Fixtures used inside of unit tests.

## `simple.wasm`

This is the simplest Wasm module that can be produced while still being compliant
with the official spec.

This is produced by the creating a file named `simple.wat` with the following contents:

```
(module)
```

and then invoke:

```console
wat2wasm simple.wat
```

## `store`

This directory contains a store folder structure that is used for testing.
`windows` contains a store that is used for testing on Windows.
`default` contains a store that is used for testing on all other platforms.
