---
title: Dan McArdle
lang: en-US

# LaTeX variables
documentclass: article
urlcolor: OliveGreen
colorlinks: True

my-name: Dan McArdle
personal-site: https://da.nmcardle.com/cv
github-url: https://github.com/dmcardle
email: d@nmcardle.com
phone: (315) 317-6220

dan-abstract: |
    A generalist software engineer with 8+ years' experience in security, systems software, and cryptography, I enjoy finding security vulnerabilities in low-level code by writing fuzzers, performing static analysis, and manual inspection.
    Recently, I developed infrastructure that enabled JTAG-based end-to-end hardware verification of OpenTitan, an open-source silicon root-of-trust chip.
    I've contributed to IETF specifications in the TLS and DNS spaces by developing prototypes of draft revisions, which supported the evaluation of their feasibility and correctness.
    In the defense arena, I developed a PoC that generates VMs with unique calling conventions by modifying LLVM's X86 codegen and FreeBSD's kernel and userspace.

---

# Experience

## Google
Software Engineer | Cambridge, MA | October 2023 -- Present

* Contributing to privacy-preserving advertising APIs within Chrome's [Privacy Sandbox](https://developer.chrome.com/docs/privacy-sandbox/).

## zeroRISC Inc.
Software Engineer | Cambridge, MA | April 2023 -- September 2023

### OpenTitan

* Optimized CRC32 implementation, achieving a 28x speedup.
  * Developed an on-device perftest to measure performance baseline, then rewrote the C implementation with inline assembly instructions from RISC-V's [bitmanip](https://github.com/riscv/riscv-bitmanip/raw/main-history/bitmanip-0.93.pdf) spec (PR [#17989](https://github.com/lowRISC/opentitan/pull/17989)).
    This yielded a 20x speedup.
  * Discovered significant overhead from function calls by inspecting disassembly.
    Achieved an overall 28x speed improvement by inlining helper functions (PR [#18068](https://github.com/lowRISC/opentitan/pull/18068)).
* Audited call sites of `sec_mmio` functions for improper usage.
  This mitigated the risk of shipping self-inflicted DoS bugs in the [M2.5.1-RC0](https://github.com/lowRISC/opentitan/releases/tag/Earlgrey-M2.5.1-RC0) release.
  * Developed syntax-level audit tooling with Bazel, Python, and libclang (PR [#18719](https://github.com/lowRISC/opentitan/pull/18719)).
* Enabled C/C++ compiler warnings for entire project in 20+ PRs ([tracker](https://github.com/lowRISC/opentitan/issues/12553#issuecomment-1542312293)).
  This work improved the toolchain's ability to detect bugs and undefined behavior.
* Added a "chip info" struct at a fixed location in the ROM (PRs [#18100](https://github.com/lowRISC/opentitan/pull/18100) and [#18254](https://github.com/lowRISC/opentitan/pull/18254)).
  This change is intended to aid debugging when the ROM crashes.
  For instance, if the ROM silently failed on a physical chip, we could dump the chip info via JTAG and determine which Git revision the software came from.
* Developed "ROM\_EXT bootstrap" feature, a recovery mode for reprogramming the flash via the SPI interface after manufacturing.
  * Refactored existing ROM bootstrap into a library to enable code reuse (PR [#19155](https://github.com/lowRISC/opentitan/pull/19155)).
  * Implemented new ROM\_EXT bootstrap with access controls that protect the flash regions that contain ROM\_EXT (PR [#18929](https://github.com/lowRISC/opentitan/pull/18929)).
  * Wrote a fuzzer that throws SPI commands at the bootstrap library (PR [#19194](https://github.com/lowRISC/opentitan/pull/19194)).

<!--
* Bazel work
  * Upgraded to Bazel 6.2.1 (PR [#17021](https://github.com/lowRISC/opentitan/pull/17021)).
  * Lychee: https://github.com/lowRISC/opentitan/pull/18257
  * Added Bazel aspect to run clang-tidy: https://github.com/lowRISC/opentitan/pull/18537
  * Clang-tidy test rules: https://github.com/lowRISC/opentitan/pull/18763
  * Clang-tidy warnings not errors: https://github.com/lowRISC/opentitan/pull/18773
* UB fixes
  * Downcast: https://github.com/lowRISC/opentitan/pull/18943
  * Bitshift: https://github.com/lowRISC/opentitan/pull/19097
  * Union access: https://github.com/lowRISC/opentitan/pull/18498
-->

## Google
Software Engineer | Cambridge, MA | October 2018 -- March 2023

### OpenTitan

* Developed Python, TCL, and Bazel tooling to splice [OTP](https://docs.opentitan.org/hw/ip/otp_ctrl/) (one-time programmable memory) images into pre-built FPGA bitstreams (PR [#15163](https://github.com/lowRISC/opentitan/pull/15163)).
  This enabled more comprehensive end-to-end tests and saved >1 hour of build time per test.
* Created infrastructure for JTAG-based end-to-end tests defined with GDB and OpenOCD.
  * Custom Bazel test rule: [`opentitan_gdb_fpga_cw310_test`](https://github.com/lowRISC/opentitan/blob/master/rules/opentitan_gdb_test.bzl#L234).
  * Python backend for rule: [`gdb_test_coordinator.py`](https://github.com/lowRISC/opentitan/blob/3f69ac5a0863acd31343914a42ee2a3bbd79b64a/rules/scripts/gdb_test_coordinator.py).
* Used these new splicing and testing capabilities to develop a number of end-to-end tests. A few examples:
  * Test that the ROM initializes watchdog timer (PR [#15798](https://github.com/lowRISC/opentitan/pull/15798)).
  * Test that JTAG debugging works in various lifecycle states (PR [#16139](https://github.com/lowRISC/opentitan/pull/16139)).
  * Test the configuration of physical memory protection (PR [#16169](https://github.com/lowRISC/opentitan/pull/16169)).
* Optimized memory functions and achieved a 1.5-5x speedup (PR [#14243](https://github.com/lowRISC/opentitan/pull/14243)).
* Enabled cross-references for C/C++ sources in [Codesearch](https://cs.opensource.google/opentitan) by developing an internal CI pipeline.
  This improves developer productivity by reducing friction while exploring the codebase.
  Try it out by clicking on a function or variable in [dif_otbn.c](https://cs.opensource.google/opentitan/opentitan/+/master:sw/device/lib/dif/dif_otbn.c).
* Designed and implemented `bitstream_bisect.py`, a tool that accelerates `git bisect` (see the design proposal in issue [#16406](https://github.com/lowRISC/opentitan/issues/16406) and implementation in PR [#16701](https://github.com/lowRISC/opentitan/pull/16701)).
  The key insight is that the time spent building bitstreams dominates the time spent running tests.
  By bisecting only on commits with cached bitstreams, we can run what would be an all-day bisect session in an hour.

### Chrome

* Developed prototypes of *TLS Encrypted Client Hello* (ECH) in BoringSSL.
  ECH enables clients to encrypt sensitive fields such as the desired server name, which are sent in cleartext by default.
    * Completed C and Go server prototypes for draft 09 in [CL 45285](https://boringssl-review.googlesource.com/c/boringssl/+/45285).
    * Contributed to ECH's specification in [eight PRs](https://github.com/tlswg/draft-ietf-tls-esni/pulls?q=is%3Apr+is%3Aclosed+author%3Admcardle).
    * Added GREASE support for drafts 08 and 09 in [CL 40204](https://boringssl-review.googlesource.com/c/boringssl/+/40204) and [CL 44784](https://boringssl-review.googlesource.com/c/boringssl/+/44784).
      First defined in [RFC 8701](https://datatracker.ietf.org/doc/rfc8701/), GREASE staves off ecosystem ossification by enabling clients to send fake ECH data to servers that do not support it; passive middleboxes cannot tell the difference.
      Thus, passive adversaries cannot selectively block ECH traffic without blocking GREASEd non-ECH traffic.
    * Implemented backend server for draft 09 in [CL 43924](https://boringssl-review.googlesource.com/c/boringssl/+/43924).

* Developed prototypes of [RFC 9180: Hybrid Public Key Encryption](https://www.rfc-editor.org/rfc/rfc9180.html) (HPKE) in BoringSSL.
    * Contributed C implementation of draft-irtf-cfrg-hpke-04 in [CL 41304](https://boringssl-review.googlesource.com/c/boringssl/+/41304).
    * Contributed Go implementation of draft-irtf-cfrg-hpke-05 in [CL 42124](https://boringssl-review.googlesource.com/c/boringssl/+/42124).
    * Updated C implementation to draft-irtf-cfrg-hpke-05 in [CL 42444](https://boringssl-review.googlesource.com/c/boringssl/+/42444).
    * Added PSK variants of HPKE in [CL 42664](https://boringssl-review.googlesource.com/c/boringssl/+/42664).
    * Updated C and Go implementations to draft-irtf-cfrg-hpke-07 [CL 44904](https://boringssl-review.googlesource.com/c/boringssl/+/44904).

* Contributed to specification for [SVCB/HTTPS](https://datatracker.ietf.org/doc/draft-ietf-dnsop-svcb-https/), a new DNS resource record required for practical deployment of TLS ECH.
    * While HTTPS record specification was in flux, designed and ran a Chrome experiment to study the impact of new resource records on the DNS ecosystem [[design doc]](https://docs.google.com/document/d/14eCqVyT_3MSj7ydqNFl1Yl0yg1fs6g24qmYUUdi5V-k/edit?usp=sharing).
* Added a number of fuzzers, such as [robots_rules_parser_fuzzer](https://chromium-review.googlesource.com/c/chromium/src/+/2625993),
    [content_settings_pattern_parser_fuzzer](https://chromium-review.googlesource.com/c/chromium/src/+/2308232),
    and [vr_omnibox_formatting_fuzzer](https://chromium-review.googlesource.com/c/chromium/src/+/1847793).
    * Discovered a bug in Chrome's URL parser that made it non-idempotent.
      Filed [crbug 1128999](https://bugs.chromium.org/p/chromium/issues/detail?id=1128999) and added an idempotency check to gurl_fuzzer in [CL 2414615](https://chromium-review.googlesource.com/c/chromium/src/+/2414615).

* Hosted an intern who implemented [RFC 8914: Extended DNS Errors](https://www.rfc-editor.org/rfc/rfc8914.html) in Chrome's net stack.

## Draper Laboratory

Software Engineer / Member of Technical Staff | Cambridge, MA | March 2018 -- October 2018

* Technical work on DoD projects with a focus on formal methods and cybersecurity.
* Specific topics include formally-verified software, static taint analysis, and fuzzing.
* Audited Adam Chlipala's Spring 2018 *Formal Reasoning about Programs* at MIT.

## Architecture Technology Corporation

Software Engineer | Ithaca, NY | August 2015 -- February 2018

* Cybersecurity R&D for DoD customers and technical proposal writing.
* Authored winning Phase II SBIR proposal and managed two-year development effort.
* Proposal work led to a number of patents.
* Supervised interns developing interactive security coursework.
* Technical work included Linux/FreeBSD kernel hacking and modifying the LLVM compiler.

## State University of New York at Buffalo

Adjunct Professor | Buffalo, NY | June 2015 -- August 2015

* Taught CSE 305: Introduction to Programming Languages.
    * Developed lectures and coursework teaching a variety of programming paradigms.
    * Focused on Haskell programming language and the Lambda calculus.

## Syracuse University

Graduate Teaching Assistant | Syracuse, NY | August 2013 -- May 2014

* CIS 252: Introduction to Computer Science (Spring 2014).
    * Graded papers, held office hours, and led two lab sessions per week in Haskell language.
* CIS 275: Discrete Math (Fall 2013).
    * Graded papers, held office hours, and led a weekly recitation.

## Metis Consulting Group

Intern & Software Engineer | Syracuse, NY | May 2011 -- August 2014

* Responsible for web application development projects, specializing in travel.
* Tech stack included ColdFusion, PHP, Microsoft SQL Server, and JavaScript.

<!-- SUNY Geneseo -->

# Education

## Master of Science | Computer Science and Engineering

State University of New York at Buffalo | Buffalo, NY | 2015

* Contributed to published research on adding real-time capabilities to Standard ML, a functional programming language.

## Bachelor of Arts | Computer Science

State University of New York at Geneseo | Geneseo, NY | 2013

* Multiple semesters of Directed Studies focused on Document Image Analysis.
* Presented *Stompbox* framework for real-time simulation of analog audio effects at GREAT Day (Geneseo Recognizing Excellence, Achievement, and Talent).


# Skills

* Languages: C, C++, Rust, Python, Go, Bash.
  Some experience with RISC-V and X86 assembly.
  Approximate knowledge of many other languages.
* Version control: Git. Some experience with Mercurial and Perforce.
* Build systems: Bazel, GN, Make. Some experience with CMake.
* Debuggers: GDB and RR.
* Technical writing: DoD proposals and software documentation. Contributed to some IETF specifications.

# Patents & Publications

* Daniel McArdle, Judson Powers, Robert A. Joyce (2022-12-06). *Self-healing architecture for resilient computing services* (US-11522904-B2).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11522904>

* Paul Nicotera, Robert Joyce, Judson Powers, Daniel McArdle (2022-03-15). *Systems and methods for used learned representations to determine terrain type* (US-11275940-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11275940>

* Daniel McArdle, Judson Powers (2021-05-18). *Systems and methods for runtime enforcement of data flow integrity* (US-11010495-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11010495>

* Judson Powers, Robert A. Joyce, Daniel McArdle (2019-09-10). *Mechanism for concealing application and operation system identity* (US-10412116-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10412116>

* Judson Powers, Robert A. Joyce, Daniel McArdle (2019-09-10). *Application randomization mechanism* (US-10412114-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10412114>

* Judson Powers, Robert A. Joyce, Daniel McArdle (2019-05-07). *Application randomization mechanism* (US-10284592-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10284592>

* Judson Powers, Robert A. Joyce, Daniel McArdle (2019-02-05). *Evaluating results of multiple virtual machines that use application randomization mechanism* (US-10200401-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10200401>

* Judson Powers, Robert A. Joyce, Daniel McArdle (2019-02-05). *Configuration of application randomization mechanism* (US-10200406-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10200406>

* Judson Powers, Daniel McArdle, Robert A. Joyce (2018-09-18). *Late-stage software feature reduction tool for security and performance* (US-10078510-B1).\
  <https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/10078510>

* Li, Muyuan, Daniel E. McArdle, Jeffrey C. Murphy, Bhargav Shivkumar, and Lukasz Ziarek. "Adding real-time capabilities to a SML compiler." ACM SIGBED Review 13, no. 2 (2016): 8-13.
