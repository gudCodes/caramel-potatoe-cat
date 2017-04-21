# caramel-potatoe-cat
[![Build Status](https://travis-ci.org/gudCodes/caramel-potatoe-cat.svg?branch=master)](https://travis-ci.org/gudCodes/caramel-potatoe-cat)

A collection of tools written in rust.

## fast fastq

Initial idea: pure rust implementation for fast parsing of FASTQ files ultimately for faster analysis

**Parsing:**

Maybe some inspiration from [fast CSV parsing in rust](https://github.com/BurntSushi/rust-csv).

Most existing FASTQ manipulation implementations are based on [htslib](https://github.com/samtools/htslib).

FASTQ files are simple text files with a four line record corresponding to a sequence, [details on wikipedia](https://en.wikipedia.org/wiki/FASTQ_format). 

[1000 genomes](http://www.internationalgenome.org/) is a good place for example data. One sample (NA12878) with multiple different FASTQ file sizes: ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/phase3/data/NA12878/sequence_read/.

Typically FASTQs gzipped and read in as pairs of files.
So `<name>_1.fastq.gz` and `<name>_2.fastq.gz` should have the same number of rows with each entry corresponding in order to a pair of reads.

**k-mer matching:**

A basic analysis is to provide as input a specific k-mer, e.g. "CGAA" and return all of the reads that contain the input k-mer. Variations could include:
- Quality thresholds for matches
- Wildcards

