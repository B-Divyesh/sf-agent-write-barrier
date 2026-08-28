# Changelog

All notable changes follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project uses semantic versioning.

## [Unreleased]

### Fixed

- Versioned the offline shell cache from its complete precache contents and revalidate document navigations so content-only documentation releases reach existing clients.
- Added Azure Static Web Apps cache and response-policy configuration: immutable hashed assets, revalidated HTML/service worker, CSP, framing, permissions, and same-origin resource protections.

## [0.1.0] - 2026-08-27

### Added

- Linux Landlock write enforcement with an explicit unsafe audit fallback.
- Open JSON policy format with allowed and watched roots.
- Full before/after receipts covering tracked, ignored, untracked, and `.git` paths.
- Policy initialization, environment checks, receipt inspection, and JSON output.
- Static documentation site with a local receipt simulator.
