1. Wazne jest czy ten blad wystepuje moze wystapic przy samych obliczeniach czy konieczny jest store.
2. Typ operacji, wydaje sie ze konieczne jest dzielenie.
3. Error jakie wyrzuca CLI (ink_e2e, drink, wrapper).
4. Czy ten blad sie utrzymuje na wszystkich wersjach ink'ach



ink-e2e
1. Same obliczenia wystarczaja
2. Error jest dla kazdego typu operacjii
3.
```
---- bug::e2e_tests::check_timestamps_without_store stdout ----
n = 1
thread 'bug::e2e_tests::check_timestamps_without_store' panicked at lib.rs:129:22:
updating failed: CallExtrinsic
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```