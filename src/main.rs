extern crate regex;
use regex::Regex;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::HashMap;

fn sec_to_dhms(s: u64) -> (u64, u64, u64, u64) {
    let mut s = s;
    let d = s / 86400;
    s %= 86400;
    let h = s / 3600;
    s %= 3600;
    let m = s / 60;
    s %= 60;
    return (d,h,m,s)
}

fn calc_point(compute_time: u64, resouce_point: f64) -> f64 {
    let point = compute_time as f64 * resouce_point / 3600.0;
    return point
}

fn parse_query(query: &str) -> (&str, u64, u64, f64) {
    let mut instances = HashMap::new();
    instances.insert("Full", 1.0);
    instances.insert("Glarge", 0.9);
    instances.insert("Gsmall",0.3);
    instances.insert("Clarge", 0.6);
    instances.insert("Csmall", 0.2);
    instances.insert("AFull", 3.0);
    instances.insert("AGsmall", 0.5);
    instances.insert("Mlarge", 0.4);
    instances.insert("Msmall", 0.2);

    let mut instance_match_score = 0;
    let mut matched_instance = "Gsmall";
    let mut compute_time = 0;
    let mut num_jobs = 1;

    let re = Regex::new(r"\d+[d,h,m,s]").unwrap();
    let re_x = Regex::new(r"x\d+").unwrap();
    let re_char = Regex::new(r"[^0-9a-zA-Z]+").unwrap();
    let fuzz_matcher = SkimMatcherV2::default();

    let query = query.trim_end().to_lowercase();
    let buf_split = query.split(" ");

    for s in buf_split {
        let s = re_char.replace_all(&s, "").to_string();
        if re.is_match(&s){
            for c in re.find_iter(&s){
                let _matched_text = c.as_str();
                let _time = &_matched_text[0.._matched_text.len()-1];
                let mut _time: u64 = _time.parse().unwrap();
                let _hms = _matched_text.chars().last().unwrap();
                match _hms {
                    'd' => _time *= 86400,
                    'h' => _time *= 3600,
                    'm' => _time *= 60,
                    _ => (),
                }
                compute_time += _time
            }
        }

        else if re_x.is_match(&s){
            for c in re_x.find_iter(&s){
                let _matched_text = c.as_str();
                let _x = &_matched_text[1.._matched_text.len()];
                let _x: u64 = _x.parse().unwrap();
                num_jobs *= _x;
            }
        }

        else {
            for (k, _) in &instances {
                let result = fuzz_matcher.fuzzy_match(&k, &s);
                if result.is_some(){
                    let score = result.unwrap();
                    if instance_match_score <= score{
                        instance_match_score = score;
                        matched_instance = k;
                    }
                }
            }
        }
        
    }

    let point = calc_point(compute_time, instances[matched_instance]);

    return (matched_instance, compute_time, num_jobs, point)
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..].join(" ");

    let (matched_instance, compute_time_single, num_jobs, point_single) = parse_query(&args);
    let compute_time_total = compute_time_single * num_jobs;

    let (ds, hs, ms, ss) = sec_to_dhms(compute_time_single);
    let (dt, ht, mt, st) = sec_to_dhms(compute_time_total);

    let point_total = point_single * num_jobs as f64;

    println!("{instance} Single: {ds}d{hs}h{ms}m{ss}s {ps:.2}pts x{nj} Total: {dt}d{ht}h{mt}m{st}s {pt:.2}pts", instance=matched_instance, ds=ds, hs=hs, ms=ms, ss=ss, ps=point_single, nj=num_jobs, dt=dt, ht=ht, mt=mt, st=st, pt=point_total);
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_0() {
        let query = "gsmall 1h";
        let (ins, ct, nj, pt) = parse_query(&query);
        assert_eq!(ins, "Gsmall");
        assert_eq!(ct, 3600);
        assert_eq!(nj, 1);
        assert_approx_eq!(pt, 0.3);
    }

    #[test]
    fn test_1() {
        let query = "glarge 1h1m1s 1s 1h x2 x3 x1x2";
        let (ins, ct, nj, pt) = parse_query(&query);
        assert_eq!(ins, "Glarge");
        assert_eq!(ct, 7262);
        assert_eq!(nj, 12);
        assert_approx_eq!(pt, 1.8155);
    }

    #[test]
    fn test_2 () {
        let query = "as 1h x2x3";
        let (ins, ct, nj, pt) = parse_query(&query);
        assert_eq!(ins, "AGsmall");
        assert_eq!(ct, 3600);
        assert_eq!(nj, 6);
        assert_approx_eq!(pt, 0.5);
    }

    #[test]
    fn test_3 () {
        let query = "gl 1h x2x3";
        let (ins, ct, nj, pt) = parse_query(&query);
        assert_eq!(ins, "Glarge");
        assert_eq!(ct, 3600);
        assert_eq!(nj, 6);
        assert_approx_eq!(pt, 0.9)
    }

    #[test]
    fn test_4(){
        let query = "1m x1x2 1h1s glarge 1s 1h x3 x2";
        let (ins, ct, nj, pt) = parse_query(&query);
        assert_eq!(ins, "Glarge");
        assert_eq!(ct, 7262);
        assert_eq!(nj, 12);
        assert_approx_eq!(pt, 1.8155);
    }

}