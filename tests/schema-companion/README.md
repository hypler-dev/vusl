# Schema companion tests

`scripts/test_schema_companion.py` validates only the checked-in mapping-review
authoring companion. It loads every frozen native member through the existing
native corpus loader, preserves each raw input SHA-256, and writes one external
applicability report per invocation.

Run with explicitly provisioned, hash-pinned Python tooling:

```bash
PYTHONDONTWRITEBYTECODE=1 /absolute/path/to/python scripts/test_schema_companion.py --self-test
PYTHONDONTWRITEBYTECODE=1 /absolute/path/to/python scripts/test_schema_companion.py \
  --require-python 3.11.17 --output /absolute/external/schema-applicability.json
```

The test never installs dependencies or resolves schemas from network/file
locations. `tooling-lock.json` pins the six-package closure and nine Draft
2020-12 resource identities. The schema uses only local `$defs` references;
the registry denies every unexpected retrieval.

`applicability.json` pins the frozen manifest/cases hashes and every allowed
exception by member ID and raw hash. It does not change native expectations:
all accepted members must schema-pass, listed malformed raw members remain
unrepresentable, listed core-only members are narrow authoring limitations,
and every other refused member must schema-fail.

The report keeps three outcomes separate: `structural_rejection` means the
decoded instance fails the companion; `core_only` means it passes the companion
but the native oracle refuses it; `not_representable` means malformed raw bytes
were not passed to the schema. A companion pass never proves native acceptance,
canonical identity, byte-limit compliance, duplicate-name safety, cross-item
term identity, or host scope.
