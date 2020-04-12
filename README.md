# edict_romaji

edict_romaji adds Romaji field to EDICT Dictionary File.

## Usage:

```bash
$ unzip -p edict.zip edict | sed -n '2,$p' | edict_romaji
```

## Cargo.toml:

for rust cargo tool
