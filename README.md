# zoth
CLI Cache Tool

## Usage

for example with plantuml
```shell
$ time cat ~/relations.pu | zoth exec -- java -Xmx256m -jar -Dfile.encoding=UTF-8 plantuml.jar -tsvg -p > out.svg
cat ~/relations.pu  0.00s user 0.00s system 34% cpu 0.006 total
./target/release/zoth exec -- java -Xmx256m -jar -Dfile.encoding=UTF-8  -tsvg -p  60.15s user 1.56s system 121% cpu 50.658 total

# second time
$ time cat ~/relations.pu | zoth exec -- java -Xmx256m -jar -Dfile.encoding=UTF-8 plantuml.jar -tsvg -p > out.svg
cat ~/relations.pu  0.00s user 0.00s system 30% cpu 0.008 total
./target/release/zoth exec -- java -Xmx256m -jar -Dfile.encoding=UTF-8  -tsvg -p  0.00s user 0.00s system 46% cpu 0.010 total
```

## TODO

* Purge Cache (zothcl with clear sub-command)
* Configurable (zothcl/zothd)
* Server Mode (zothd)
* HTTP Interface (zothd)

## LICENSE

MIT
