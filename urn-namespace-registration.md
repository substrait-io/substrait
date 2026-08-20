# Formal URN Namespace Registration: `substrait`

> **Status:** Working draft. This document is the completed
> [RFC 8141, Appendix A](https://datatracker.ietf.org/doc/html/rfc8141#appendix-A)
> registration template for a **Formal URN Namespace**. Per
> [RFC 8141 §6.2](https://datatracker.ietf.org/doc/html/rfc8141#section-6.2)
> it is to be sent to the **urn@ietf.org** discussion list for Expert Review.
> Items marked **TODO** require a decision or a real point of contact before
> submission.
>
> This registration does **not** require a published RFC; Expert Review of this
> template is sufficient for IANA to register the namespace identifier (NID).

---

## Registration Template

**Namespace Identifier:**

`substrait`

(Conforms to the formal-NID rules of
[RFC 8141 §5.1](https://datatracker.ietf.org/doc/html/rfc8141#section-5.1): it is
longer than two characters, is composed only of letters, does not begin with
`urn-`, `X-`, `xn--`, or two letters followed by `-`, and is not already
registered.)

**Version:**

1

**Date:**

2026-07-08

**Registrant:**

The Substrait project — an open, community-governed specification for
representing relational-algebra (query) plans across languages and systems.

- Web: <https://substrait.io>
- Source repository: <https://github.com/substrait-io/substrait>
- Contact: **TODO** — named individual or role and a stable email address that
  IANA/the Designated Experts can reach (e.g. a maintainers' alias or mailing
  list). RFC 8141 expects a durable point of contact.

**Purpose:**

The `substrait` namespace provides persistent, location-independent, and
collision-free identifiers for **Substrait extension definitions**.

Substrait is a cross-language specification for relational algebra. Beyond its
built-in types and functions, Substrait supports *simple extensions*: YAML
documents that define additional data types, type variations, and (scalar,
aggregate, window, and table) functions. A serialized Substrait plan references
each extension it uses by an identifier that is embedded directly in the plan
(the `urn` field of the `substrait.extensions.SimpleExtensionURN` protobuf
message). The identified resource is the extension definition — its named types,
type variations, and function signatures — not any particular file location.

*Community of use.* The consumers and producers of these identifiers are systems
that use Substrait as an interchange format for query plans, including the four
reference SDKs (`substrait-java`, `substrait-go`, `substrait-python`,
`substrait-rs`) and the query engines and tools built on them.

*Benefit.* Substrait plans are long-lived, portable artifacts that are exchanged
between independently developed systems. An identifier that is decoupled from any
host, URL scheme, or transport lets a plan remain valid and unambiguous even when
the definition is hosted elsewhere, mirrored, cached, or bundled directly into a
consumer. Reverse-domain-name ownership within the identifier prevents collisions
between extensions published by different organizations.

*Relationship to existing systems.* This namespace replaces two earlier,
non-conforming schemes used by Substrait:

1. HTTP(S) URIs that pointed at the physical location of a YAML file (now
   deprecated in the protobuf definitions), and
2. an interim identifier of the form `extension:<owner>:<id>` that is URN-*like*
   but is not a valid URN (it lacks the `urn:` prefix and a registered NID).

Registering `substrait` lets these identifiers become syntactically valid URNs
without changing their semantics. The scheme is distinct from and does not
overlap with any other registered URN namespace.

*Resolution.* No resolution service is operated or mandated today; see
**Resolution** below.

*Standardization.* Substrait is a community-governed specification. This
registration is being pursued through RFC 8141 Expert Review and is not, at this
time, part of an IETF standards-track document.

**Syntax:**

A Substrait URN refines the `assigned-name` production of
[RFC 8141 §2](https://datatracker.ietf.org/doc/html/rfc8141#section-2). The
namespace-specific string (NSS) begins with a *resource-type* token, which
reserves room for future Substrait resource types; version 1 defines exactly one,
`extension`.

```abnf
; Substrait URN — a refinement of RFC 8141 "assigned-name".
substrait-urn = "urn:substrait:" substrait-nss

substrait-nss = resource-type ":" owner ":" id

resource-type = "extension"                 ; the only type defined in version 1

owner         = label *( "." label )        ; reverse domain name, e.g. io.substrait
label         = alphanum *( alphanum / "-" )

id            = 1*id-char
id-char       = alphanum / "-" / "_" / "."

alphanum      = ALPHA / DIGIT               ; per RFC 5234
```

- Every character used above (`ALPHA`, `DIGIT`, `-`, `.`, `_`, and the `:`
  separators) is a `pchar` as defined in
  [RFC 8141 §2](https://datatracker.ietf.org/doc/html/rfc8141#section-2), so
  `substrait-nss` is a conforming NSS.
- `owner` SHOULD follow the
  [reverse domain name convention](https://en.wikipedia.org/wiki/Reverse_domain_name_notation)
  for a domain controlled by the publisher (e.g. `io.substrait`, `com.example`,
  `org.apache.arrow`). The prefix `io.substrait` is reserved for extensions
  published by the Substrait project itself.
- `id` is the publisher-chosen identifier of the extension (e.g.
  `functions_arithmetic`, `extension_types`).

An equivalent (informative) regular expression, applied case-insensitively over
ASCII, is:

```text
^urn:substrait:extension:[A-Za-z0-9][A-Za-z0-9-]*(\.[A-Za-z0-9][A-Za-z0-9-]*)*:[A-Za-z0-9._-]+$
```

The ABNF above is normative; the regex is provided for implementers.

*Special-character encoding.* All permitted characters are `pchar`s that do not
require percent-encoding. Conforming Substrait URNs **MUST NOT** use
percent-encoding; the `q`-, `f`-, and `r`-components of RFC 8141 are not used.

*URN-equivalence.* Equivalence is determined per
[RFC 8141 §3](https://datatracker.ietf.org/doc/html/rfc8141#section-3): the
`urn:` prefix and the NID `substrait` are compared case-insensitively. In
addition, the entire `substrait-nss` is compared **case-insensitively** using
ASCII case folding — this is consistent with the case-insensitivity of the
`owner` (a domain name) and with the all-lowercase convention used for `id`. No
percent-encoding normalization is required because percent-encoding is not used.
These rules are chosen to eliminate false negatives.

*Examples.*

| Deprecated `extension:` identifier              | Registered URN                                             |
| ----------------------------------------------- | ---------------------------------------------------------- |
| `extension:io.substrait:functions_arithmetic`   | `urn:substrait:extension:io.substrait:functions_arithmetic`|
| `extension:io.substrait:extension_types`        | `urn:substrait:extension:io.substrait:extension_types`     |
| `extension:com.example:my_extension`            | `urn:substrait:extension:com.example:my_extension`         |

Migration from the interim scheme is a literal replacement of the leading
`extension:` with `urn:substrait:extension:`.

**Assignment:**

Assignment is **open** and **delegated**, with no registration required with the
Substrait project:

- The `owner` segment scopes every identifier to a domain controlled by its
  publisher. Uniqueness across publishers is guaranteed by delegating to the
  existing global uniqueness of the Domain Name System: a publisher mints URNs
  only under a reverse domain name it controls.
- Within a single `owner`, the publisher is responsible for keeping `id` values
  distinct.
- The `io.substrait` owner prefix is administered by the Substrait project for
  its standard extensions.

There is no central Substrait registry of individual URNs, and the Substrait
project does not assign identifiers on behalf of third parties.

**Security and Privacy:**

Following [RFC 3552](https://datatracker.ietf.org/doc/html/rfc3552) and
[RFC 6973](https://datatracker.ietf.org/doc/html/rfc6973):

- A Substrait URN is an opaque identifier embedded in a plan. It conveys no
  authority and grants no access by itself.
- *False negatives* (two equivalent URNs not recognized as equal) would cause a
  consumer to fail to load an extension it actually supports. This is mitigated
  by the explicit, simple equivalence rules above (case-insensitive, no
  percent-encoding).
- *False positives / collisions* (two distinct extensions sharing a URN) are
  mitigated by reverse-domain-name delegation: only the controller of a domain is
  expected to publish under its prefix.
- *Resolution risk.* Resolving a URN to an extension definition is out of band
  (see **Resolution**). A consumer that fetches a definition based on a URN
  found in an untrusted plan MUST validate and sandbox that definition rather
  than trusting it implicitly; a definition can change the interpretation of a
  plan.
- *Information leakage / privacy.* Identifiers are not expected to carry personal
  data. Because `owner` is a domain name, a URN reveals the publishing
  organization; this is intended and analogous to package or XML-namespace names.
  Directory-harvesting concerns do not apply, as there is no central registry to
  enumerate.

**Interoperability:**

- The identifiers closely resemble the interim, non-URN `extension:<owner>:<id>`
  strings still present in existing plans and tooling. During the migration
  period, producers and consumers must be prepared to distinguish the two and
  should treat the `urn:substrait:` form as authoritative. See **Additional
  Information**.
- The `resource-type` token (`extension` in version 1) is a forward-compatibility
  point: consumers encountering an unrecognized resource-type token under
  `urn:substrait:` should treat the URN as unsupported rather than misinterpret
  it.
- The reverse-domain-name `owner` visually resembles Java package names and XML
  namespace conventions, but there is no formal relationship to those systems and
  no possibility of confusion at the URN level.
- No overlap or confusion with other registered URN namespaces or URI schemes is
  known.

**Resolution:**

No resolution mechanism is required, operated, or mandated by this registration.
Substrait URNs are **location-independent identifiers**. A consumer maps a URN to
a concrete extension definition out of band — for example via local
configuration, a bundled copy, an internal catalog, or an agreed convention (the
Substrait project publishes its own standard extensions in its source
repository). The Substrait project may in the future recommend a resolution
convention, but does not currently operate a resolver. The RFC 8141
`r`-component is not used and should be ignored by consumers if present.

**Documentation:**

- Extensions overview and identifier format:
  <https://substrait.io/extensions/> (source:
  <https://github.com/substrait-io/substrait/blob/main/site/docs/extensions/index.md>)
- Protobuf definition of the identifier field
  (`substrait.extensions.SimpleExtensionURN`):
  <https://github.com/substrait-io/substrait/blob/main/proto/substrait/extensions/extensions.proto>
- JSON schema requiring the `urn` field on every extension file:
  <https://github.com/substrait-io/substrait/blob/main/text/simple_extensions_schema.yaml>

**Additional Information:**

*Migration.* This namespace supersedes two earlier schemes: deprecated HTTP(S)
URIs (removed from the protobuf definitions) and the interim `extension:` scheme.
Migration to the registered form is a mechanical prefix change (`extension:` →
`urn:substrait:extension:`); the `owner` and `id` segments are unchanged, so the
mapping is one-to-one and semantics are preserved. On the wire the identifier is
stored as a plain UTF-8 string in `SimpleExtensionURN.urn`.

*Anticipated evolution.* The `resource-type` token was introduced precisely so
the namespace can grow without a second migration. The Substrait project
anticipates that a future revision may add an optional trailing segment to the
NSS (for example, an extension **version**). Any such change will be published as
a new **Version** of this registration via the RFC 8141 revision process and will
describe its differences from this version; unversioned URNs minted under
version 1 will remain valid.

*Background.* Tracking issue:
<https://github.com/substrait-io/substrait/issues/1016>. The URN form registered
here was discussed in <https://github.com/substrait-io/substrait/pull/881>.

**Revision Information:**

Version 1. Initial registration; no prior versions.

---

## Submission checklist (not part of the template)

- [ ] Fill in the **Registrant** contact (**TODO** above).
- [ ] Confirm the syntax, equivalence rules, and `io.substrait` reservation with
      the Substrait maintainers (community-consensus change).
- [ ] Send the completed template to **urn@ietf.org** for Expert Review
      ([RFC 8141 §6.2](https://datatracker.ietf.org/doc/html/rfc8141#section-6.2)).
- [ ] Iterate to address reviewer comments; re-submit as needed.
