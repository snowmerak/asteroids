#[macro_export]
macro_rules! conds {
    ( $( $o:expr )+; $( $( $p:expr )+, $a:expr; )+ ) => {
        {
            let mut objs = Vec::new();
            $(
                objs.push($o);
            )+

            let mut pats = Vec::new();
            let mut acts = Vec::new();
            $(
                pats.push(Vec::new());
                acts.push($a);
            )+
            let mut count = 0 as usize;
            $(
                $(
                    pats.get_mut(count).unwrap().push($p);
                )+
                count += 1;
            )+

            count = 0;
            let mut result = None;
            for ps in &pats {
                let mut matched = true;
                for i in 0..objs.len() {
                    if ps.get(i).unwrap() != objs.get(i).unwrap() {
                        matched = false;
                        break
                    }
                }
                if matched {
                    let rs = acts.get(count).unwrap().clone();
                    result = Some(rs);
                    break
                }
                count += 1;
            }
            result
        }
    };
}