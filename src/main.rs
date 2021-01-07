use std::collections::HashSet;

fn main() {
    let inputdata = include_str!("input");

    let mut requirements: Vec<(char, char)> = inputdata
        .lines()
        .map(|l| l.as_bytes())
        .map(|l| (l[5] as char, l[36] as char))
        .collect();

    let mut resultat = String::new();
    let mut right: HashSet<char> = HashSet::new();

    while !requirements.is_empty() {
        let left: HashSet<char> = requirements.iter().map(|(x, _)| *x).collect();
        right = requirements.iter().map(|(_, y)| *y).collect();

        let mut valids = left.difference(&right).collect::<Vec<&char>>();

        valids.sort();
        let valid: String = valids.into_iter().take(1).collect();

        resultat = format!("{}{}", resultat, valid);

        requirements.retain(|(x, _y)| !valid.contains(&x.to_string()));
    }
    let restants: String = right.into_iter().collect();
    resultat = format!("{}{}", resultat, restants);
    println!("{}", resultat);
    println!("Ok");
}
