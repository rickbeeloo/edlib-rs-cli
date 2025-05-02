/// This file provides examples and tests similar to the file edaligner.cpp in  apps directory
/// of edlib. It loads simple fasta files of one sequence in directory test_data of edlib.
///
extern crate edlib_rs;
use edlib_rs::edlib_sys::*;
use edlib_rs::*; // This imports all public items including EdlibAlignConfigRs and EdlibAlignModeRs

use ::std::os::raw::c_char;
use clap::{App, Arg};
use std::path::Path;
use std::process;

use cpu_time::ProcessTime;
use std::time::Duration;
/// example
/// edaligner --dirdata  "/Soft/edlib/test_data/Enterobacteria_Phage_1"
///           --tf "Enterobacteria_phage_1.fasta"
///           --qf "mutated_60_perc.fasta"
fn main() {
    let dirdata: String;
    let qfile: String;
    let tfile: String;
    let output_file: String;

    let matches = App::new("edaligner")
        .arg(
            Arg::with_name("dirdata")
                .long("dirdata")
                .required(true)
                .takes_value(true)
                .help("expection directory of data files"),
        )
        .arg(
            Arg::with_name("qfile")
                .long("qf")
                .takes_value(true)
                .help("expection query file for seq"),
        )
        .arg(
            Arg::with_name("tfile")
                .long("tf")
                .takes_value(true)
                .help("expection target file for seq"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .required(true)
                .help("output file to write the edit distance"),
        )
        .get_matches();

    // get data directory
    if matches.is_present("dirdata") {
        println!("dirdata");
        dirdata = matches
            .value_of("dirdata")
            .ok_or("bad value")
            .unwrap()
            .parse::<String>()
            .unwrap();
        //println!("got dirdata , {}", dirdata);
    } else {
        println!("dirdata is mandatory");
        process::exit(1);
    }

    // get query file
    if matches.is_present("qfile") {
        qfile = matches
            .value_of("qfile")
            .ok_or("bad value")
            .unwrap()
            .parse::<String>()
            .unwrap();
        //println!("got qfile , {}", qfile);
    } else {
        // println!("query file is mandatory");
        process::exit(1);
    }

    // get target file
    if matches.is_present("tfile") {
        tfile = matches
            .value_of("tfile")
            .ok_or("bad value")
            .unwrap()
            .parse::<String>()
            .unwrap();
        //  println!("got target file , {}", tfile);
    } else {
        // println!("target file is mandatory");
        process::exit(1);
    }

    // get output file
    if matches.is_present("output") {
        output_file = matches
            .value_of("output")
            .ok_or("bad value")
            .unwrap()
            .parse::<String>()
            .unwrap();
        //  println!("got output file , {}", output_file);
    } else {
        println!("output file is mandatory");
        process::exit(1);
    }

    let qfname = Path::new(&dirdata).join(qfile);
    let tfname = Path::new(&dirdata).join(tfile);
    // use logger
    let qseq: Vec<u8>;
    // get sequences
    let mut reader = needletail::parse_fastx_file(&qfname).expect("expecting valid query filename");
    if let Some(record) = reader.next() {
        let qrec = record.expect("invalid record");
        qseq = qrec.seq().into_owned();
    } else {
        std::process::exit(1);
    } // end for query seq
      //
    let tseq: Vec<u8>;
    // get sequences
    let mut reader =
        needletail::parse_fastx_file(&tfname).expect("expecting valid target filename");
    if let Some(record) = reader.next() {
        let trec = record.expect("invalid record");
        tseq = trec.seq().into_owned();
    } else {
        std::process::exit(1);
    } // end for query seq

    // Only run the EDLIB_MODE_HW alignment and save results to file
    let mut config = EdlibAlignConfigRs::default();
    config.mode = EdlibAlignModeRs::EDLIB_MODE_HW;
    let mut equalitypairs = Vec::<EdlibEqualityPairRs>::new();
    let pair = EdlibEqualityPairRs {
        first: 'A' as c_char,
        second: 'N' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 'T' as c_char,
        second: 'N' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 'G' as c_char,
        second: 'N' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 'C' as c_char,
        second: 'N' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 'N' as c_char,
        second: 'N' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 'a' as c_char,
        second: 'n' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 't' as c_char,
        second: 'n' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 'g' as c_char,
        second: 'n' as c_char,
    };
    equalitypairs.push(pair);
    let pair = EdlibEqualityPairRs {
        first: 'c' as c_char,
        second: 'n' as c_char,
    };
    equalitypairs.push(pair);
    config.additionalequalities = &equalitypairs;

    let start = ProcessTime::try_now().unwrap();
    let align_res = edlibAlignRs(&qseq, &tseq, &config);
    assert_eq!(align_res.status, EDLIB_STATUS_OK);
    let cpu_time: Duration = start.try_elapsed().unwrap();

    println!(
        "\nmode: EDLIB_MODE_HW, cpu time (ms): {}, distance: {}",
        cpu_time.as_millis(),
        align_res.editDistance
    );

    // Write edit distance to output file
    std::fs::write(&output_file, align_res.editDistance.to_string())
        .expect("Unable to write to output file");
}
