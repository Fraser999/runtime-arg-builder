# runtime-arg-builder

**NOT PRODUCTION READY**

## Usage

* modify the args in `main()`
* `cargo r`
* it writes the encoded runtime args to a file named `runtime_args.json` in the current working directory (overwrites if file pre-exists)
* the file can then be used as the arg to `--payment-args-complex` or `--session-args-complex` of the `casper-client`

This is a stopgap tool pending `casper-client` improvements to better support complex runtime args.
