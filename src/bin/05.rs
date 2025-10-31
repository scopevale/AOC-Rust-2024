use std::collections::{BTreeSet, HashMap, HashSet};

advent_of_code::solution!(5);

type Rule = (u32, u32);

fn parse_input(s: &str) -> anyhow::Result<(Vec<Rule>, Vec<Vec<u32>>)> {
    let (left, right) = s
        .split_once("\n\n")
        .ok_or_else(|| anyhow::anyhow!("expected a blank line separating rules and updates"))?;

    let mut rules = Vec::new();
    for line in left.lines().filter(|l| !l.trim().is_empty()) {
        let (a, b) = line
            .split_once('|')
            .ok_or_else(|| anyhow::anyhow!("bad rule line: {line}"))?;
        let a: u32 = a.trim().parse()?;
        let b: u32 = b.trim().parse()?;
        rules.push((a, b));
    }

    let mut updates = Vec::new();
    for line in right.lines().filter(|l| !l.trim().is_empty()) {
        let row = line
            .split(',')
            .map(|t| t.trim().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        // Optional: assert unique pages within an update (comment out if not needed)
        // {
        //     use std::collections::HashSet;
        //     let mut set = HashSet::new();
        //     if !row.iter().all(|&x| set.insert(x)) {
        //         anyhow::bail!("duplicate page in update: {line}");
        //     }
        // }
        updates.push(row);
    }

    Ok((rules, updates))
}

fn is_valid(update: &[u32], rules: &[Rule]) -> bool {
    // position lookup for O(1) index checks
    let pos: HashMap<u32, usize> = update.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    // For each rule A|B: if both A and B appear in this update, require pos[A] < pos[B]
    for &(a, b) in rules {
        if let (Some(&ia), Some(&ib)) = (pos.get(&a), pos.get(&b)) {
            if ia >= ib {
                return false;
            }
        }
    }
    true
}

/// Reorders a single update so that it satisfies the rules.
/// We perform a Kahn-style topological sort on the subgraph induced by the update's pages.
/// To keep results deterministic and “AoC-like”, we break ties by the **original index** in the update.
fn fix_update(update: &[u32], rules: &[Rule]) -> Option<Vec<u32>> {
    // Presence set for this update
    let present: HashSet<u32> = update.iter().copied().collect();

    // Record each page's original index to break ties deterministically.
    let orig_idx: HashMap<u32, usize> = update.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    // Build subgraph: edges only for pages present in this update
    let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut indeg: HashMap<u32, usize> = HashMap::new();

    for &p in &present {
        adj.entry(p).or_default();
        indeg.entry(p).or_insert(0);
    }
    for &(a, b) in rules {
        if present.contains(&a) && present.contains(&b) {
            adj.entry(a).or_default().push(b);
            *indeg.entry(b).or_insert(0) += 1;
        }
    }

    // Priority set of nodes with zero indegree, ordered by original index
    let mut zeros: BTreeSet<(usize, u32)> = BTreeSet::new();
    for (&node, &d) in &indeg {
        if d == 0 {
            zeros.insert((orig_idx[&node], node));
        }
    }

    let mut out = Vec::with_capacity(update.len());
    let mut indeg_mut = indeg; // take ownership

    while let Some((_, node)) = zeros.iter().next().cloned() {
        zeros.take(&(orig_idx[&node], node));
        out.push(node);
        if let Some(neis) = adj.get(&node) {
            for &v in neis {
                let e = indeg_mut.get_mut(&v).unwrap();
                *e -= 1;
                if *e == 0 {
                    zeros.insert((orig_idx[&v], v));
                }
            }
        }
    }

    if out.len() == present.len() {
        Some(out)
    } else {
        // Cycle or inconsistency detected — shouldn't happen in AoC inputs.
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(&input).ok()?;

    let mut sum: u64 = 0;
    for u in updates {
        if is_valid(&u, &rules) {
            let mid = u[u.len() / 2] as u64;
            sum += mid;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(&input).ok()?;

    let mut sum: u64 = 0;
    for u in updates {
        if !is_valid(&u, &rules) {
            let fixed = fix_update(&u, &rules).expect("rules should be acyclic for the subset");
            sum += fixed[fixed.len() / 2] as u64;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
