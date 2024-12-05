use regex::Regex;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("/Users/ken/src/aoc24/aoc3/src/input.txt").expect("file didn't open");
    let result = get_enabled(content.as_str());

    println!("result: {}", result);
}

fn get_enabled(content: &str) -> i32 {
    let mut result = 0;

    for line in content.lines() {
        if let Some((b, a)) = line.split_once("don't()") {
            result += perform_calc(b);
            result =
                a.split("don't()")
                    .fold(result, |mut acc, item| match item.split_once("do()") {
                        Some((_before, after)) => {
                            acc += perform_calc(after);
                            acc
                        }
                        _ => acc,
                    });
        } else {
            result += perform_calc(line);
        }
    }

    result
}

fn perform_calc(split_input: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"^([0-9]+),([0-9]+)\)").unwrap();

    for sp in split_input.split("mul(").skip(1) {
        for (_, [num1, num2]) in re.captures_iter(sp).map(|c| c.extract()) {
            let n1 = num1.parse::<i32>().unwrap();
            let n2 = num2.parse::<i32>().unwrap();
            result += n1 * n2;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::get_enabled;

    #[test]
    fn test_example() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(get_enabled(s), 48);
    }
    #[test]
    fn test_multiple() {
        let s = "mul(168,52)^>+don't()mul(453,478)^<-+when()who() *?from()mul(741,484)mul(975,310),how()*?mul(127,418)&where()mul(185,669)$select()^#mul(288,189%why()when()do()@how()+/~who()what()(mul(784,991)^<what()mul(969,971)[{what()mul(436,579)+$^],;$what()'>don't();'!&>$mul(748,589)who(){{select()mul(539,888)):)$>mul(650,904)^mul(11,905)$when())mul(3,778)(?from()-]>mul(216,342)()mul(678;don't()'%]:mul(297,302)mul(356,884)mul(915,227)]~*[!:fr";
        let result = 168 * 52 + 784 * 991 + 969 * 971 + 436 * 579;

        assert_eq!(get_enabled(s), result);
    }

    #[test]
    fn test_jarbled() {
        let s = ":mul(252,355)?select()!-don't())::what()mul(670,441))[mul(591,136),@ mul(338,232)@}mul(944,594)@what()}~>:what(596,921)mul(121,669)%}+mul(306,982)#^from()mul(530,368)#select(){!;!+!what()mul(809,843)%mul(997,156)^'where()$how()mul(444,325)}mul(14,843)from(989,984)#(<< +when()mul(330,71)*mul(277,727)who()(&$#-do()'mul(386,736)";
        let result = 252 * 355 + 386 * 736;

        assert_eq!(get_enabled(s), result);
    }

    #[test]
    fn test_simplified_cases() {
        let qs = [
            "mul(252,355)don't()mul(670,441)don't()mul(591,136)mul(338,232)do()mul(386,736)do()mul(386,736)",
            "don't()mul(252,355)don't()mul(670,441)don't()mul(591,136)mul(338,232)do()mul(386,736)do()mul(386,736)",
            "do()mul(252,355)don't()mul(670,441)don't()mul(591,136)mul(338,232)don't()do()mul(386,736)do()mul(386,736)",
            "don't()mul(252,355)do()mul(670,441)don't()mul(591,136)do()mul(338,232)don't()mul(386,736)do()mul(386,736)",
            "do()mul(670,441)don't()mul(591,136)do()mul(338,232)don't()mul(386,736)do()mul(386,736)",
            "do()don't()mul(670,441)don't()mul(591,136)do()mul(338,232)don't()mul(386,736)do()mul(386,736)",
        ];
        let results = [
            252 * 355 + 386 * 736 + 386 * 736,
            386 * 736 + 386 * 736,
            252 * 355 + 386 * 736 + 386 * 736,
            670 * 441 + 338 * 232 + 386 * 736,
            670 * 441 + 338 * 232 + 386 * 736,
            338 * 232 + 386 * 736,
        ];

        for (i, s) in qs.iter().enumerate() {
            assert_eq!(get_enabled(s), results[i]);
        }
    }
}
