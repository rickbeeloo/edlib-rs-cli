extern crate edlib_rs;
use ::std::os::raw::c_char;
use clap::Parser;
use edlib_rs::edlib_sys::*;
use edlib_rs::*;
use std::io::Write;
use std::path::Path;

fn get_config() -> EdlibAlignConfigRs<'static> {
    let mut config = EdlibAlignConfigRs::default();
    config.mode = EdlibAlignModeRs::EDLIB_MODE_HW; // Infix, semi-global
    config.additionalequalities = &EQUALITY_PAIRS; // Allow A, a, T, t, G, g, C, c to be considered equal (see bottom of file)
    config
}
/// Command-line arguments for edaligner
#[derive(clap::Parser, Debug)]
#[command(
    name = "edaligner",
    about = "Align query and target sequences using edlib"
)]
struct Cli {
    #[clap(long, help = "Directory of data files")]
    dirdata: String,

    #[clap(long, help = "Query file for sequence")]
    qf: String,

    #[clap(long, help = "Target file for sequence")]
    tf: String,

    #[clap(long, help = "Output file to write the edit distance")]
    output: String,
}

fn main() {
    // Parse command-line arguments using clap derive API
    let args = Cli::parse();

    let qfname = Path::new(&args.dirdata).join(&args.qf);
    let tfname = Path::new(&args.dirdata).join(&args.tf);

    // Read query sequences from file
    let mut reader = needletail::parse_fastx_file(&qfname).expect("expecting valid query filename");
    let q_seq = if let Some(record) = reader.next() {
        let qrec = record.expect("invalid record");
        qrec.seq().into_owned()
    } else {
        std::process::exit(1);
    };

    // Get config
    let config = get_config();

    // Buffered output writer
    let mut writer = std::io::BufWriter::new(std::fs::File::create(&args.output).unwrap());

    // Compare targets to each query
    let mut reader = needletail::parse_fastx_file(&tfname).expect("invalid ref filename");

    while let Some(record) = reader.next() {
        let trec = record.expect("invalid record");
        let tseq = trec.seq().into_owned();
        let align_res = edlibAlignRs(&q_seq, &tseq, &config);
        assert_eq!(align_res.status, EDLIB_STATUS_OK);
        writeln!(writer, "{}", align_res.editDistance).expect("Unable to write to output file");
    }
}

static EQUALITY_PAIRS: [EdlibEqualityPairRs; 17] = [
    EdlibEqualityPairRs {
        first: 'A' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'A' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'a' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'a' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'T' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'T' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 't' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 't' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'G' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'G' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'g' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'g' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'C' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'C' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'c' as c_char,
        second: 'N' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'c' as c_char,
        second: 'n' as c_char,
    },
    EdlibEqualityPairRs {
        first: 'N' as c_char,
        second: 'N' as c_char,
    },
];
