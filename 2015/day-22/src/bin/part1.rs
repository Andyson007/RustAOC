use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
struct Boss {
    hitpoints: u32,
    damage: u32,
    poison: u32,
}

#[derive(Clone, Copy, Debug)]
struct Player {
    hitpoints: u32,
    mana: u32,
    recharge: u32,
    armor: u32,
}

fn main() {
    let player = Player {
        hitpoints: 50,
        mana: 500,
        recharge: 0,
        armor: 0,
    };
    let boss = Boss {
        hitpoints: 51,
        damage: 9,
        poison: 0,
    };
    let ans = solve(player, boss);
    println!("{ans:?}");
}

fn solve(mut player: Player, mut boss: Boss) -> Option<u32> {
    if apply_effects(&mut player, &mut boss) {
        return Some(0);
    }
    let mut ret = None;
    if player.recharge == 0 && player.mana >= 229 {
        let mut player = player.clone();
        player.recharge = 5;
        player.mana -= 229;
        let solve = bossturn(player, boss);
        if let Some(x) = solve {
            ret = min(ret, Some(x + 229));
        }
    }

    if player.armor == 0 && player.mana >= 113 {
        let mut player = player.clone();
        player.armor = 6;
        player.mana -= 113;
        let solve = bossturn(player, boss);
        if let Some(x) = solve {
            ret = min(ret, Some(x + 113));
        }
    }

    if boss.poison == 0 && player.mana >= 173 {
        let mut boss = boss.clone();
        boss.poison = 6;
        player.mana -= 173;
        let solve = bossturn(player, boss);
        if let Some(x) = solve {
            ret = min(ret, Some(x + 173));
        }
    }

    if player.mana > 53 {
        let mut boss = boss.clone();
        if boss.hitpoints > 4 {
            boss.hitpoints -= 4;
            player.mana -= 53;
            let solve = bossturn(player, boss);
            if let Some(x) = solve {
                ret = min(ret, Some(x + 53));
            }
        } else {
            ret = min(ret, Some(53));
        }
    }

    if player.mana > 73 {
        let mut boss = boss.clone();
        let mut player = player.clone();
        if boss.hitpoints > 2 {
            boss.hitpoints -= 2;
            player.hitpoints += 2;
            player.mana -= 73;
            let solve = bossturn(player, boss);
            if let Some(x) = solve {
                ret = min(ret, Some(x + 73));
            }
        } else {
            ret = min(ret, Some(73));
        }
    }

    ret
}

fn min<T>(a: Option<T>, b: Option<T>) -> Option<T>
where
    T: PartialOrd + Copy + Debug,
{
    if let Some(x) = a {
        if let Some(y) = b {
            if x > y {
                Some(y)
            } else {
                Some(x)
            }
        } else {
            a
        }
    } else {
        b
    }
}

fn bossturn(mut player: Player, mut boss: Boss) -> Option<u32> {
    let damage = boss.damage - if player.armor > 1 { 7 } else { 0 };
    if apply_effects(&mut player, &mut boss) {
        return Some(0);
    }
    if player.hitpoints <= damage {
        None
    } else {
        player.hitpoints -= damage;
        solve(player, boss)
    }
}

fn apply_effects(player: &mut Player, boss: &mut Boss) -> bool {
    // println!("{player:?} {boss:?}");
    if player.recharge > 0 {
        player.mana += 101;
        player.recharge -= 1;
    }
    if player.armor > 0 {
        player.armor -= 1;
    }
    if boss.poison > 0 {
        if boss.hitpoints > 3 {
            boss.hitpoints -= 3;
            boss.poison -= 1;
        } else {
            // println!("{player:?} {boss:?};");
            return true;
        }
    }
    // println!("{player:?} {boss:?};");
    false
}
