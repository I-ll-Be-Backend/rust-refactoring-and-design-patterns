#!/bin/sh
# Compiles every step of every lesson in rust:1.83-slim: the check each lesson passed before its video went out.
# Needs Docker. A step folder holding .nocheck does not compile on purpose and is skipped.
docker run --rm --entrypoint /bin/sh -v "$PWD:/w" rust:1.83-slim -c '
true
cd /w; fail=0
for f in */step-*/*.rs; do
  [ -e "$f" ] || continue
  [ -e "$(dirname "$f")/.nocheck" ] && continue
  d=$(mktemp -d)
  if ( rustc --edition 2021 --crate-type lib --emit=metadata -o $d/x.rmeta $f ) >/tmp/log 2>&1; then echo "ok    $f"; else echo "FAIL  $f"; cat /tmp/log; fail=1; fi
done
exit $fail'
