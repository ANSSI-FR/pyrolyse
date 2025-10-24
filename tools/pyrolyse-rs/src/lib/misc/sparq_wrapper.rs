// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fmt::Debug;
// use std::fmt::Display;
// use std::hash::Hash;
// use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

// use crate::interval::IntervalC;

pub fn get_sparq_consistency(sparq_path: &Path, constraint_s: String) -> bool {
    debug!("get_sparq_consistency: start");
    debug!("get_sparq_consistency: constraint_s: {}", constraint_s);

    if !sparq_path.exists() {
        panic!("Cannot find SparQ at location: {:?}", sparq_path)
    };

    let arg_v = vec![
        "constraint-reasoning",
        "allen",
        "check-consistency",
        &constraint_s,
    ];

    let p = Command::new(sparq_path)
        .args(&arg_v)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = p.wait_with_output().unwrap();

    if !output.status.success() {
        panic!("SparQ failed with: {:?}",output)
    };
    debug!("output:\n{:?}", output);

    let output_s = output.stdout.iter().map(|&c| c as char).collect::<String>();

    let r = output_s == "Consistent.\n";

    debug!("get_sparq_consistency: r: {}", r);
    debug!("get_sparq_consistency: end");

    r
}

//
// def get_sparq_quantification(constraint_s):
//   stdout_s = subprocess.check_output(["../tools/SparQ/sparq",
//                                       "quantify",
//                                       "allen",
//                                       constraint_s
//                                      ])
//   return stdout_s
//

pub fn get_sparq_quantification(sparq_path: &Path, constraint_s: String) -> String {
    debug!("get_sparq_quantification: start");

    if !sparq_path.exists() {
        panic!("Cannot find SparQ at location: {:?}", sparq_path)
    };

    let arg_v = vec!["quantify", "allen", &constraint_s];

    let p = Command::new(sparq_path)
        .args(&arg_v)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = p.wait_with_output().unwrap();

    debug!("output:\n{:?}", output);

    let output_s = output.stdout.iter().map(|&c| c as char).collect::<String>();

    debug!("get_sparq_quantification: output_s: {:?}", output_s);
    debug!("get_sparq_quantification: end");

    output_s
}
