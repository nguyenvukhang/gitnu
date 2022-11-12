# gitnu integration tests

test cases found in `./cases`
each file is ran line-by-line in `./cases`.
first line of each file is treated as the tests's title

## test file structure

```
/tmp/gitnu/
├── 01.rec
├── 01.exp
└── 01.diff
/path-to-this-repo
├── Cargo.toml
├── src/
└── tests/
   ├── log/
   │  └── 01.full
   └── cases/
      └── 01.sh
```
