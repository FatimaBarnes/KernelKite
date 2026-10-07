# Security Policy

## Reporting a vulnerability

Please do not open a public issue for a security problem. Use GitHub's private
[security advisory](https://github.com/tiqhmibisimax/KernelKite/security/advisories/new) form, or contact a
maintainer directly if the form is unavailable to you.

Include what an attacker gains in one sentence, the version or commit you are running, the smallest
reproduction you can manage, and whether the input is attacker-controlled in your setup. You should
get an acknowledgement within a few working days and a disclosure timeline before anything is
published.

## Scope

KernelKite samples `/proc` for a process subtree and writes bundles. In scope: a
crafted `/proc` field (or a process that rewrites its own command line) that produces a
misleading measurement, a sampled command that escapes the sampling loop, a bundle that
verifies after being edited, and path handling in the bundle reader.
