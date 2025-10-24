use std::cmp::max;
use std::cmp::min;
use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::str::FromStr;

use serde::{Deserialize, Serialize, Serializer};

use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct IntervalD {
    start: u16,
    end: u16,
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

impl IntervalD {
    pub fn new(start: u16, end: u16) -> IntervalD {
        IntervalD { start, end }
    }

    pub fn of_sparq_string(s: String) -> IntervalD {
        let v = s.split(' ').collect::<Vec<&str>>();
        let start = u16::from_str(v[1]).unwrap() - 1;
        let end = u16::from_str(v[2]).unwrap() - 2;
        IntervalD::new(start, end)
    }

    pub fn get_start(&self) -> u16 {
        self.start
    }

    pub fn get_end(&self) -> u16 {
        self.end
    }

    pub fn intersect(&self, interval: &IntervalD) -> bool {
        max(self.start, interval.start) <= min(self.end, interval.end)
    }

    pub fn intersection(&self, other: &IntervalD) -> Option<IntervalD> {
        if self.intersect(other) {
            Some(IntervalD::new(
                max(self.start, other.start),
                min(self.end, other.end),
            ))
        } else {
            None
        }
    }

    pub fn remove_intersection(&self, other: &IntervalD) -> Option<Vec<IntervalD>> {
        if self.intersect(other) {
            let r = AllenIntervalAlgebraRelation::of_intervals(self,other);
            debug!("remove_intersection: r: {:?}",r);
            match r {
                AllenIntervalAlgebraRelation::Si | AllenIntervalAlgebraRelation::Oi => {
                    let interval_d =  IntervalD::new(other.end + 1,self.end);
                    Some(vec![interval_d])
                },
                AllenIntervalAlgebraRelation::Fi | AllenIntervalAlgebraRelation::O => {
                    let interval_d =  IntervalD::new(self.start,other.start - 1);
                    Some(vec![interval_d])
                },
                AllenIntervalAlgebraRelation::Eq => {
                    None
                },
                AllenIntervalAlgebraRelation::Di =>  {
                    // need to split the interval
                    let starting_interval_d = IntervalD::new(self.start,other.start - 1);
                    let finishing_interval_d = IntervalD::new(other.end + 1,self.end);
                    Some(vec![starting_interval_d,finishing_interval_d])
                },
                _ => panic!("Unexpected interval to remove")
            }
        } else {
            panic!("Tried to remove an intersection interval portion while there is no intersection");
        }
    }

    pub fn get_duration(&self) -> u16 {
        self.end - self.start + 1
    }

    pub fn apply_offset(&self, data_offset: u16) -> IntervalD {
        IntervalD::new(self.start + data_offset, self.end + data_offset)
    }

    pub fn get_csv_column_s_v(index: u16, suffix: &str) -> Vec<String> {
        let v = vec![
            format!("i{}{}", index, suffix),
            format!("i{}{}n", index, suffix),
            format!("i{}{}s", index, suffix),
            format!("i{}{}e", index, suffix),
        ];
        v
    }

    pub fn get_csv_data_s_v(&self, meta_index: u16, index: u16) -> Vec<String> {
        let v = vec![
            format!("{}", index),
            format!("{}:{}", meta_index, index),
            format!("{}", self.start),
            format!("{}", self.end),
        ];
        v
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalC {
    #[serde(serialize_with = "ordered_map")]
    hm: HashMap<u16, IntervalD>,
}

fn ordered_map<S>(value: &HashMap<u16, IntervalD>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl FromIterator<(u16, IntervalD)> for IntervalC {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (u16, IntervalD)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl IntervalC {
    pub fn new(hm: HashMap<u16, IntervalD>) -> IntervalC {
        IntervalC { hm }
    }

    pub fn get(&self, index: &u16) -> Option<&IntervalD> {
        self.hm.get(index)
    }

    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn of_sparq_string(s: String) -> IntervalC {
        debug!("of_sparq_string: start");
        let hm: HashMap<u16, IntervalD> = s
            .split(") (")
            .map(|s| {
                debug!("of_sparq_string: s: {}", s);
                // let s_clean = s
                //     .replace('(', "")
                //     .replace('(', "")
                //     .replace(')', "")
                //     .replace(')', "")
                //     .replace("\n", "");
                let s_clean = s.replace(['(', '(', ')', ')', '\n'], "");
                debug!("of_sparq_string: s_clean: {}", s_clean);
                let v = s_clean.split(' ').collect::<Vec<&str>>();
                let index = u16::from_str(remove_first(v[0]).unwrap()).unwrap();
                (index, IntervalD::of_sparq_string(s_clean))
            })
            .collect();
        // v.sort();
        debug!("of_sparq_string: end");
        IntervalC::new(hm)
    }

    pub fn iter(&self) -> Iter<u16, IntervalD> {
        self.hm.iter()
    }

    pub fn get_last_interval_index(&self) -> u16 {
        let hm: HashMap<u16, u16> = self
            .hm
            .iter()
            .map(|(index, interval_d)| (interval_d.end, *index))
            .collect();

        let end_max = hm.keys().max().unwrap();
        *hm.get(end_max).unwrap()
    }

    pub fn get_last_data_index(&self) -> u16 {
        let v: Vec<u16> = self.hm.values().map(|interval_d| interval_d.end).collect();
        let end_max = v.iter().max().unwrap();
        *end_max
    }

    pub fn get_rightmost_ending_data_offset(&self) -> u16 {
        let v: Vec<u16> = self.hm.values().map(|interval_d| interval_d.end).collect();
        let end_max = v.iter().max().unwrap();
        *end_max
    }

    pub fn get_rightmost_starting_data_offset(&self) -> u16 {
        let v: Vec<u16> = self.hm.values().map(|interval_d| interval_d.start).collect();
        let start_max = v.iter().max().unwrap();
        *start_max
    }

    pub fn get_total_length(&self) -> u16 {
        let start = self
            .hm
            .values()
            .map(|interval_d| interval_d.start)
            .min()
            .unwrap();
        let end = self
            .hm
            .values()
            .map(|interval_d| interval_d.end)
            .max()
            .unwrap();
        end - start + 1
    }

    pub fn apply_offset(&self, index_offset: u16, data_offset: u16) -> IntervalC {
        debug!("apply_offset: start");
        let hm: HashMap<_, _> = self
            .hm
            .iter()
            .map(|(index, interval_d)| (index + index_offset, interval_d.apply_offset(data_offset)))
            .collect();
        // v.sort();
        debug!("apply_offset: data_offset: {:?}", data_offset);
        debug!("apply_offset: old: {:?}", self);
        debug!("apply_offset: new: {:?}", IntervalC::new(hm.clone()));
        debug!("apply_offset: end");
        IntervalC::new(hm)
    }

    pub fn get_csv_column_s_v(len: u32, suffix: &str) -> Vec<String> {
        // (0..len)
        //     .map(|index| IntervalD::get_csv_column_s_v(index as u16, suffix))
        //     .flatten()
        //     .collect()
        (0..len)
            .flat_map(|index| IntervalD::get_csv_column_s_v(index as u16, suffix))
            .collect()
    }

    pub fn get_csv_data_s_v(&self, meta_index: u16) -> Vec<String> {
        // self.hm
        //     .iter()
        //     .map(|(index, interval_d)| interval_d.get_csv_data_s_v(meta_index, *index))
        //     .flatten()
        //     .collect()
        self.hm
            .iter()
            .flat_map(|(index, interval_d)| interval_d.get_csv_data_s_v(meta_index, *index))
            .collect()
    }
}
