# VUSL security policy

VUSL documents observations and semantic relationships. They do not grant authority. Native authentication, authorization, enforcement, execution, and effect verification remain independent responsibilities of the relevant native system. A matching identifier, digest, report, trace, UI action, or parser result must not be treated as permission or proof of an external effect.

Mappings must expose loss and uncertainty. Refuse an operation when a critical constraint cannot be preserved or understood. Do not silently widen permissions, discard authority-relevant fields, join independently sourced evidence without a qualified relation, or turn unknown or conflicting state into success. Keep deployment revision, effective configuration, authorization, and actual outcome separate.

The current D0a profile is experimental and offline. It omits native bytes and cannot claim lossless translation. Future profiles that retain native bytes or sensitive provenance require separate storage, privacy, retention, consent, and deletion review. Do not place secrets, credentials, private reports, customer data, or provider payloads in this repository.

## Reporting a concern

Do not publish sensitive vulnerability details, exploit material, credentials, private reports, or personal data in a public issue or pull request. If this repository has an enabled private security reporting feature, use it. If it does not, contact a maintainer through the established private channel for this project. If no private channel is available, provide only a minimal non-sensitive report and request private coordination before sharing details.

Security reports are triaged against the affected profile, vectors, parser boundary, native contract, and evidence scope. A report or review does not authorize provider access, production changes, or release.
