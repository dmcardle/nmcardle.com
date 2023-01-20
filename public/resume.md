<style>
body {
    font-family: sans-serif;
}
body h1,h2,h3 {
    font-family: monospace;
}
body {
    padding: 5em;
    width: 80em; 
}
body > h3 {
    font-weight: bold;
    color: darkred;
}
body a {
    color: green;
}
</style>

<title>Dan McArdle</title>

# Dan McArdle

## Experience

### Google

Software Engineer | Cambridge, MA | October 2018 - January 2023

* **OpenTitan**
    * Developed tooling for end-to-end chip tests.
        * Splicing OTP images into FPGA bitstreams saved hours per test run.
        * Created a new capability: JTAG-based tests defined with GDB and OpenOCD.
    * Developed a handful of e2e tests for the chip, e.g. [PR #16169](https://github.com/lowRISC/opentitan/pull/16169), [PR #16139](https://github.com/lowRISC/opentitan/pull/16139), [PR #15798](https://github.com/lowRISC/opentitan/pull/15798).
    * Optimized memory functions and achieved a 1.5-5x speedup. [PR #14243](https://github.com/lowRISC/opentitan/pull/14243).
    * Enabled *semantic* codesearch features for C/C++ sources, e.g. [dif_otbn.c](https://cs.opensource.google/opentitan/opentitan/+/master:sw/device/lib/dif/dif_otbn.c).
    * Designed and added tool for rapid bisecting. [PR #16701](https://github.com/lowRISC/opentitan/pull/16701).
* **Chrome**
    * Developed prototypes of *TLS Encrypted Client Hello* (ECH) in the BoringSSL library.
        * Server prototype in [CL 45285](https://boringssl-review.googlesource.com/c/boringssl/+/45285)
    * Developed prototypes of Hybrid Public Key Encryption (HPKE) in BoringSSL.
        * draft-irtf-cfrg-hpke-04 in [CL 41304](https://boringssl-review.googlesource.com/c/boringssl/+/41304)
        * draft-irtf-cfrg-hpke-07 in [CL 44904](https://boringssl-review.googlesource.com/c/boringssl/+/44904)
    * Contributed to specification for [SVCB/HTTPS](https://datatracker.ietf.org/doc/draft-ietf-dnsop-svcb-https/), a new DNS resource record. 
        * Ran a Chrome experiment to study the impact of new resource records on the DNS ecosystem: [design doc](https://docs.google.com/document/d/14eCqVyT_3MSj7ydqNFl1Yl0yg1fs6g24qmYUUdi5V-k/edit?usp=sharing).
    * Developed many fuzzers for Chrome.
        * Discovered and fixed tons of security bugs.
    * Hosted an intern developing Extended DNS Errors.
    
### Draper Laboratory 

Member of Technical Staff | Cambridge, MA | March 2018 - October 2018

* Technical work on DoD projects with a focus on formal methods and cybersecurity.
* Specific topics include formally-verified software, static taint analysis, and fuzzing
* Audited Adam Chlipala's Spring 2018 *Formal Reasoning about Programs* at MIT

### Architecture Technology Corporation

Software Engineer | Ithaca, NY | August 2015 - February 2018

* Cybersecurity R&D for DoD customers and technical proposal writing
* Wrote winning Phase II SBIR proposal and managed two-year development effort
* Supervised interns developing interactive security coursework
* Technical work included Linux/FreeBSD kernel hacking and modifying the LLVM compiler

### State University of New York at Buffalo

Adjunct Professor | Buffalo, NY | June 2015 - August 2015

* Taught CSE 305: Introduction to Programming Languages
* Developed lectures and coursework teaching a variety of programming paradigms
* Focused on Haskell programming language and the Lambda calculus

### Syracuse University

Graduate Teaching Assistant | Syracuse, NY | August 2013 - May 2014

* CIS 252: Introduction to Computer Science (Spring 2014)
  Graded papers, held weekly office hours, and led two lab sessions per week in Haskell language.
* CIS 275: Discrete Math (Fall 2013)
  Graded papers, held office hours, and led a weekly recitation.
  
### Metis Consulting Group

Intern & Software Engineer | Syracuse, NY | May 2011 - August 2014

* Responsible for web application development projects, specializing in travel
* Tech stack included ColdFusion, PHP, Microsoft SQL Server, and JavaScript

<!-- SUNY Geneseo -->

## Education

### Master of Science | Computer Science and Engineering

State University of New York at Buffalo | Buffalo, NY | 2015

* Published research on adding real-time capabilities to a functional programming language

### Bachelor of Arts | Computer Science

State University of New York at Geneseo | Geneseo, NY | 2013

* Directed Studies focused on Document Image Analysis
* Presented *Stompbox* framework for real-time simulation of analog audio effects at GREAT Day (Geneseo Recognizing Excellence, Achievement, and Talent)
