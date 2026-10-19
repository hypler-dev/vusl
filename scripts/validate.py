#!/usr/bin/env python3
"""Check local specification links and the pinned extraction inventory offline.

This is repository integrity validation, not D0a parser conformance.
"""
import hashlib
import json
import re
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]


def anchors(text):
    result = set(re.findall(r'<a\s+id="([^"]+)"', text))
    counts = {}
    for heading in re.findall(r'^#{1,6}\s+(.+)$', text, re.M):
        slug = re.sub(r'[^\w\- ]', '', heading.strip().lower()).replace(' ', '-')
        count = counts.get(slug, 0)
        counts[slug] = count + 1
        result.add(slug if count == 0 else f'{slug}-{count}')
    return result


def main():
    manifest = json.loads((ROOT / 'conformance/manifest.json').read_text())
    assert manifest['schema'] == 'vusl.extraction-manifest.v1'
    assert re.fullmatch(r'[0-9a-f]{40}', manifest['baseCommit'])
    assert manifest['sourceRepository'] == 'https://github.com/hypler-dev/rangoon'
    files = manifest['files']
    assert len(files) == 7
    for item in files:
        assert item['sourcePath'].startswith('docs/') and '..' not in Path(item['sourcePath']).parts
        assert re.fullmatch(r'[0-9a-f]{64}', item['sourceSha256'])
        target = (ROOT / item['path']).resolve()
        assert target.is_relative_to(ROOT), item['path']
        raw = target.read_bytes()
        assert hashlib.sha256(raw).hexdigest() == item['sha256'], item['path']
        if item['bytePreserved']:
            assert item['sha256'] == item['sourceSha256'], item['path']
            json.loads(raw)
    links = 0
    for source in ROOT.rglob('*.md'):
        if '.git' in source.parts:
            continue
        text = source.read_text()
        assert all(line == line.rstrip() for line in text.splitlines()), source
        for raw in re.findall(r'!?\[[^\]]*\]\(([^)]+)\)', text):
            url = urlsplit(raw)
            if url.scheme or url.netloc:
                continue
            target = (source.parent / unquote(url.path)).resolve() if url.path else source
            assert target.is_relative_to(ROOT) and target.exists(), (source, raw)
            if url.fragment and target.suffix == '.md':
                assert unquote(url.fragment) in anchors(target.read_text()), (source, raw)
            links += 1
    assert (ROOT / 'assets/vusl-logo.png').read_bytes().startswith(b'\x89PNG\r\n\x1a\n')
    print(f'PASS: 7 extraction hashes, 3 preserved JSON vectors, {links} local links, PNG identity and Markdown whitespace')


if __name__ == '__main__':
    main()
