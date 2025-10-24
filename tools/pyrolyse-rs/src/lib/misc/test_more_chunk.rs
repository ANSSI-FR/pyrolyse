use crate::misc::interval::IntervalC;
use crate::misc::policy_evaluation::PolicyEvaluation;


// return false if a less specific policy evaluation covers the test case  
// return true otherwise 
pub fn is_test_case_original(
  policy_evaluation: &PolicyEvaluation,
  temporal_position_sl: &[u16],
  interval_c: &IntervalC,
  generate_only_unique_test_cases: bool
) -> bool {
  debug!("is_test_case_original: start");
  let rightmost_starting_v = get_rightmost_starting_v(
      temporal_position_sl,
      interval_c,
  );
  debug!("is_test_case_original: rightmost_starting_v: {:?}", rightmost_starting_v);

  let rightmost_finishing_v = get_rightmost_finishing_v(
      temporal_position_sl,
      interval_c,
  );
  debug!("is_test_case_original: rightmost_finishing_v: {:?}", rightmost_finishing_v);

  match policy_evaluation {
      PolicyEvaluation::ProgressiveOldestFinishing
      | PolicyEvaluation::OnceStartPrecedesOldestFinishing
      | PolicyEvaluation::OnceStartFollowsOldestFinishing
      | PolicyEvaluation::ProgressiveNewestFinishing
      | PolicyEvaluation::OnceStartPrecedesNewestFinishing
      | PolicyEvaluation::OnceStartFollowsNewestFinishing
      => {
          if rightmost_finishing_v.len() >= 2 || !generate_only_unique_test_cases {
              true
          } else { false }
      },
      PolicyEvaluation::ProgressiveMidFinishing
      | PolicyEvaluation::ProgressiveOldestNewestFinishing
      | PolicyEvaluation::ProgressiveOldestMidFinishing
      | PolicyEvaluation::ProgressiveMidNewestFinishing
      | PolicyEvaluation::OnceStartPrecedesMidFinishing
      | PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing
      | PolicyEvaluation::OnceStartPrecedesOldestMidFinishing
      | PolicyEvaluation::OnceStartPrecedesMidNewestFinishing
      | PolicyEvaluation::OnceStartFollowsMidFinishing
      | PolicyEvaluation::OnceStartFollowsOldestNewestFinishing
      | PolicyEvaluation::OnceStartFollowsOldestMidFinishing
      | PolicyEvaluation::OnceStartFollowsMidNewestFinishing
      => {
          if rightmost_finishing_v.len() == 3 {
              true
          } else { false }
      },
      PolicyEvaluation::ProgressiveOldestStarting
      | PolicyEvaluation::OnceStartPrecedesOldestStarting
      | PolicyEvaluation::OnceStartFollowsOldestStarting
      | PolicyEvaluation::ProgressiveNewestStarting
      | PolicyEvaluation::OnceStartPrecedesNewestStarting
      | PolicyEvaluation::OnceStartFollowsNewestStarting
      => {
          if rightmost_starting_v.len() >= 2 || !generate_only_unique_test_cases {
              true
          } else { false }
      },
      PolicyEvaluation::ProgressiveMidStarting
      | PolicyEvaluation::ProgressiveOldestNewestStarting
      | PolicyEvaluation::ProgressiveOldestMidStarting
      | PolicyEvaluation::ProgressiveMidNewestStarting
      | PolicyEvaluation::OnceStartPrecedesMidStarting
      | PolicyEvaluation::OnceStartPrecedesOldestNewestStarting
      | PolicyEvaluation::OnceStartPrecedesOldestMidStarting
      | PolicyEvaluation::OnceStartPrecedesMidNewestStarting
      | PolicyEvaluation::OnceStartFollowsMidStarting
      | PolicyEvaluation::OnceStartFollowsOldestNewestStarting
      | PolicyEvaluation::OnceStartFollowsOldestMidStarting
      | PolicyEvaluation::OnceStartFollowsMidNewestStarting
      => {
          if rightmost_starting_v.len() == 3 {
              true
          } else { false }
      },
      _ => true,
  }
}

pub fn get_rightmost_finishing_v(
  temporal_position_sl: &[u16],
  interval_c: &IntervalC
) -> Vec<u32> {
  debug!("get_rightmost_starting_v: end");
  let rightmost_ending_data_offset = interval_c.get_rightmost_ending_data_offset();

  let mut rightmost_finishing_v = interval_c
    .iter()
    .filter(|(_index, interval_d)| interval_d.get_end() == rightmost_ending_data_offset)
    .map(|(i, _)| *i as u32)
    .collect::<Vec<u32>>();
  debug!("more_chunk_v: rightmost_finishing_v: {:?}", rightmost_finishing_v);
  // We sort the vector of final interval index using their temporal position.
  rightmost_finishing_v.sort_by_key(|i| temporal_position_sl[*i as usize]);
  debug!("more_chunk_v: rightmost_finishing_v sorted: {:?}", rightmost_finishing_v);

  debug!("get_rightmost_finishing_v: end");
  rightmost_finishing_v
}

pub fn get_rightmost_starting_v(
  temporal_position_sl: &[u16],
  interval_c: &IntervalC
) -> Vec<u32> {
  debug!("get_rightmost_starting_v: start");
  let rightmost_starting_data_offset = interval_c.get_rightmost_starting_data_offset();
  
  let mut rightmost_starting_v = interval_c
    .iter()
    .filter(|(_index, interval_d)| interval_d.get_start() == rightmost_starting_data_offset)
    .map(|(i, _)| *i as u32)
    .collect::<Vec<u32>>();
  debug!("get_rightmost_starting_v: rightmost_starting_v: {:?}", rightmost_starting_v);
  // We sort the vector of final interval index using their temporal position.
  rightmost_starting_v.sort_by_key(|i| temporal_position_sl[*i as usize]);
  debug!("get_rightmost_starting_v: rightmost_starting_v sorted: {:?}", rightmost_starting_v);

  debug!("get_rightmost_starting_v: end");
  rightmost_starting_v
}

pub fn get_test_case_more_chunk_v(
  policy_evaluation: &PolicyEvaluation,
  temporal_position_sl: &[u16],
  interval_c: &IntervalC
) -> Vec<bool> {

  let rightmost_chunk_v = if PolicyEvaluation::generate_starting_policy_v().iter().any(|p| p == policy_evaluation) {
    get_rightmost_starting_v(
        temporal_position_sl,
        interval_c,
    )
  } else if PolicyEvaluation::generate_finishing_policy_v().iter().any(|p| p == policy_evaluation) {
    get_rightmost_finishing_v(
        temporal_position_sl,
        interval_c,
    )
  } else {
    return vec![true; interval_c.len()]
  };

  match policy_evaluation {
      PolicyEvaluation::ProgressiveAllFinishing
      | PolicyEvaluation::OnceStartPrecedesAllFinishing
      | PolicyEvaluation::OnceStartFollowsAllFinishing 
      | PolicyEvaluation::ProgressiveAllStarting
      | PolicyEvaluation::OnceStartPrecedesAllStarting
      | PolicyEvaluation::OnceStartFollowsAllStarting 
      => {
          // More fragment (MF) is always set except...
          let mut more_chunk_v = vec![true; interval_c.len()];
          // for rightmost fragments.
          rightmost_chunk_v.iter().for_each(|i| { more_chunk_v[*i as usize] = false; });
          more_chunk_v
      },
      PolicyEvaluation::ProgressiveOldestFinishing
      | PolicyEvaluation::OnceStartPrecedesOldestFinishing
      | PolicyEvaluation::OnceStartFollowsOldestFinishing 
      | PolicyEvaluation::ProgressiveOldestStarting
      | PolicyEvaluation::OnceStartPrecedesOldestStarting
      | PolicyEvaluation::OnceStartFollowsOldestStarting 
      => {
          // The index without more fragment (MF) flag is the first one.
          let i_wo_mf = rightmost_chunk_v[0];
          debug!("more_chunk_v: i_wo_mf: {:?}", i_wo_mf);

          // More fragment (MF) is always set except...
          let mut more_chunk_v = vec![true; interval_c.len()];
          // for the previously processed index.
          more_chunk_v[i_wo_mf as usize] = false;
          debug!("more_chunk_v: more_chunk_v: {:?}", more_chunk_v);
          more_chunk_v
      },
      PolicyEvaluation::ProgressiveNewestFinishing
      | PolicyEvaluation::OnceStartPrecedesNewestFinishing
      | PolicyEvaluation::OnceStartFollowsNewestFinishing 
      | PolicyEvaluation::ProgressiveNewestStarting
      | PolicyEvaluation::OnceStartPrecedesNewestStarting
      | PolicyEvaluation::OnceStartFollowsNewestStarting 
      => {
          // The index without more fragment (MF) flag is the last one.
          let i_wo_mf = rightmost_chunk_v[rightmost_chunk_v.len() - 1];
          debug!("more_chunk_v: i_wo_mf: {:?}", i_wo_mf);

          // More fragment (MF) is always set except...
          let mut more_chunk_v = vec![true; interval_c.len()];
          // for the previously processed index.
          more_chunk_v[i_wo_mf as usize] = false;
          debug!("more_chunk_v: more_chunk_v: {:?}", more_chunk_v);
          more_chunk_v
      },
      PolicyEvaluation::ProgressiveMidFinishing
      | PolicyEvaluation::OnceStartPrecedesMidFinishing
      | PolicyEvaluation::OnceStartFollowsMidFinishing 
      | PolicyEvaluation::ProgressiveMidStarting
      | PolicyEvaluation::OnceStartPrecedesMidStarting
      | PolicyEvaluation::OnceStartFollowsMidStarting 
      => {
          // Note : it is only implemented for triplet test cases
          // The index without more fragment (MF) flag is the second one if exists.
          
          // checking that there are exactly 3 rightmost chunks   
          assert!(rightmost_chunk_v.len() == 3);

          let i_wo_mf = rightmost_chunk_v[1];
          debug!("more_chunk_v: i_wo_mf: {:?}", i_wo_mf);

          // More fragment (MF) is always set except...
          let mut more_chunk_v = vec![true; interval_c.len()];
          // for the previously processed index.
          more_chunk_v[i_wo_mf as usize] = false;
          debug!("more_chunk_v: more_chunk_v: {:?}", more_chunk_v);
          more_chunk_v
      },
      PolicyEvaluation::ProgressiveOldestNewestFinishing
      | PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing
      | PolicyEvaluation::OnceStartFollowsOldestNewestFinishing 
      | PolicyEvaluation::ProgressiveOldestNewestStarting
      | PolicyEvaluation::OnceStartPrecedesOldestNewestStarting
      | PolicyEvaluation::OnceStartFollowsOldestNewestStarting 
      => {
          // Note : it is only implemented for triplet test cases
          // Indexes without more fragment (MF) flag are the first and third ones if exist.

          // checking that there are exactly 3 rightmost chunks   
          assert!(rightmost_chunk_v.len() == 3);

          let i_w_mf = rightmost_chunk_v[1];
          debug!("more_chunk_v: i_w_mf: {:?}", i_w_mf);

          // More fragment (MF) is always unset except...
          let mut more_chunk_v = vec![false; interval_c.len()];
          // for the previously processed index.
          more_chunk_v[i_w_mf as usize] = true;
          debug!("more_chunk_v: more_chunk_v: {:?}", more_chunk_v);
          more_chunk_v
      }
      PolicyEvaluation::ProgressiveOldestMidFinishing
      | PolicyEvaluation::OnceStartPrecedesOldestMidFinishing
      | PolicyEvaluation::OnceStartFollowsOldestMidFinishing 
      | PolicyEvaluation::ProgressiveOldestMidStarting
      | PolicyEvaluation::OnceStartPrecedesOldestMidStarting
      | PolicyEvaluation::OnceStartFollowsOldestMidStarting 
      => {
          // Note : it is only implemented for triplet test cases
          // Indexes without more fragment (MF) flag are the first and second ones if exist.

          // checking that there are exactly 3 rightmost chunks   
          assert!(rightmost_chunk_v.len() == 3);

          let i_w_mf = rightmost_chunk_v[2];
          debug!("more_chunk_v: i_w_mf: {:?}", i_w_mf);

          // More fragment (MF) is always unset except...
          let mut more_chunk_v = vec![false; interval_c.len()];
          // for the previously processed index.
          more_chunk_v[i_w_mf as usize] = true;
          debug!("more_chunk_v: more_chunk_v: {:?}", more_chunk_v);
          more_chunk_v
      }
      PolicyEvaluation::ProgressiveMidNewestFinishing
      | PolicyEvaluation::OnceStartPrecedesMidNewestFinishing
      | PolicyEvaluation::OnceStartFollowsMidNewestFinishing 
      | PolicyEvaluation::ProgressiveMidNewestStarting
      | PolicyEvaluation::OnceStartPrecedesMidNewestStarting
      | PolicyEvaluation::OnceStartFollowsMidNewestStarting 
      => {
          // Note : it is only implemented for triplet test cases
          // Indexes without more fragment (MF) flag are the second and third ones if exist.

          // checking that there are exactly 3 rightmost chunks   
          assert!(rightmost_chunk_v.len() == 3);

          let i_w_mf = rightmost_chunk_v[0];
          debug!("more_chunk_v: i_w_mf: {:?}", i_w_mf);

          // More fragment (MF) is always unset except...
          let mut more_chunk_v = vec![false; interval_c.len()];
          // for the previously processed index.
          more_chunk_v[i_w_mf as usize] = true;
          debug!("more_chunk_v: more_chunk_v: {:?}", more_chunk_v);
          more_chunk_v
      },
      _ => vec![true; interval_c.len()],
  }
}