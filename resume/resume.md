---
title: Dan McArdle
lang: en-US

# LaTeX variables
documentclass: article
urlcolor: OliveGreen
colorlinks: True

my-name: Dan McArdle
personal-site: https://da.nmcardle.com/resume
github-url: https://github.com/dmcardle
email: d@nmcardle.com
phone: (315) 317-6220

dan-abstract: |
    A generalist software engineer with 10+ years' experience in security and privacy, I enjoy designing and implementing new features, bug-hunting in low-level code, and contributing to open-source software.
    I currently work in Google's [Privacy Sandbox](https://privacysandbox.com), where I've been building private advertising technology in Chrome.
    Before that, I wrote bare-metal C for [OpenTitan](https://opentitan.org/) and developed novel e2e testing infrastructure.
    I've contributed to IETF specifications in the TLS and DNS spaces by implementing prototypes of draft revisions in order to evaluate feasibility.
    In the defense arena, I authored a winning Phase II SBIR proposal and created a system based on FreeBSD and LLVM that generates VMs with unique calling conventions throughout their kernel and userspace.

---

# Experience

## <employer>Google</employer> <job-role>SWE</job-role> <job-proj>Privacy Sandbox</job-proj> <job-loc>Cambridge, MA</job-loc> <job-dates>Oct. 2023--Present</job-dates>

* Contributed to the [Private Aggregation API](https://patcg-individual-drafts.github.io/private-aggregation-api)
* Designed and implemented features that improve utility without sacrificing user privacy

## <employer>zeroRISC Inc.</employer> <job-role>SWE</job-role> <job-proj>OpenTitan</job-proj> <job-loc>Cambridge, MA</job-loc> <job-dates>Apr. 2023--Sept. 2023</job-dates>

* Developed recovery mode for reprogramming flash memory after manufacturing
* Optimized CRC32 implementation, achieving a 28x speedup
* Improved debuggability by adding fixed-offset "chip info" data to the ROM

## <employer>Google</employer> <job-role>SWE</job-role> <job-proj>{OpenTitan, Chrome}</job-proj> <job-loc>Cambridge, MA</job-loc> <job-dates>Oct. 2018--Mar. 2023</job-dates>

### <job-proj>OpenTitan</job-proj>

* Saved >1 hour per build by developing tool to splice OTP images into FPGA bitstreams
* Developed glue code that enabled e2e tests to be written in terms of GDB & OpenOCD
* Implemented e2e tests that relied on splicing and testing infra
* Optimized memory functions and achieved a 1.5-5x speedup

### <job-proj>Chrome</job-proj>

* Implemented early drafts of TLS Encrypted Client Hello in BoringSSL [[draft-ietf-tls-esni/](https://datatracker.ietf.org/doc/draft-ietf-tls-esni/)]
* Created BoringSSL's first implementation of HPKE [[RFC 9180](https://datatracker.ietf.org/doc/rfc9180/)]
* Conducted an [experiment](https://chromestatus.com/feature/5948056459542528) to support development of the HTTPS RR [[RFC 9460](https://www.rfc-editor.org/rfc/rfc9460.html)]
* Analyzed solutions for "authenticated embeds" in post-third-party-cookie world

## <employer>Draper Laboratory</employer> <job-role>Security Programmer</job-role> <job-loc>Cambridge, MA</job-loc> <job-dates>Mar. 2018–Oct. 2018</job-dates>

* Technical work on DoD projects with a focus on formal methods and cybersecurity
* Specific topics include formally-verified software, static taint analysis, and fuzzing
* Audited Adam Chlipala's Spring 2018 *Formal Reasoning about Programs* at MIT

## <employer>ATCorp</employer> <job-role>R&D SWE</job-role> <job-loc>Ithaca, NY</job-loc> <job-dates>Aug. 2015--Feb. 2018</job-dates>

* Cybersecurity R&D for DoD customers and technical proposal writing
* Authored winning Phase II SBIR proposal for [SWARM](https://www.sbir.gov/awards/164047) project and managed development effort
* Proposal work led to a number of patents

## <employer>University at Buffalo</employer> <job-role>Adjunct Professor</job-role> <job-loc>Buffalo, NY</job-loc> <job-dates>June 2015--Aug. 2015</job-dates>

* Taught CSE 305: Introduction to Programming Languages
* Whirlwind tour through imperative, logic, and functional paradigms
* Special focus on Haskell and the Lambda calculus

# Education

## <span>University at Buffalo</span> <job-role>M.S.</job-role> <span>Computer Science & Eng.</span> <job-loc>Buffalo, NY</job-loc> <job-dates>2015</job-dates>

* Graduate coursework in Programming Languages and Computer Vision
* Contributed to published research on adding real-time capabilities to Standard ML

## <span>SUNY Geneseo</span> <job-role>B.A.</job-role> <span>Computer Science</span> <job-loc>Geneseo, NY</job-loc> <job-dates>2013</job-dates>

* Completed multiple Directed Studies focused on Document Image Analysis
* Developed *Stompbox*, a system that simulated analog audio effects in software
