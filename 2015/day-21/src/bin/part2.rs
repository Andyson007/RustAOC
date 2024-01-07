use itertools::Itertools;

#[derive(Clone, Copy)]
struct Player {
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone, Copy)]
struct Item<'a> {
    name: &'a str,
    cost: i32,
    damage: i32,
    armor: i32,
}

struct Shop<'a> {
    weapons: Vec<Item<'a>>,
    armor: Vec<Item<'a>>,
    rings: Vec<Item<'a>>,
}

fn main() {
    let shop = defineShop();
    let boss = Player {
        hitpoints: 103,
        damage: 9,
        armor: 2,
    };
    let mut maxgold = i32::MIN;
    for armoramount in 0..=1 {
        for ringamount in 0..=2 {
            for weapon in &shop.weapons {
                for ringchoice in shop.rings.iter().combinations(ringamount).map(|arr| {
                    let mut ret = Item {
                        name: "ring",
                        cost: 0,
                        damage: 0,
                        armor: 0,
                    };
                    for ring in arr {
                        ret = Item {
                            cost: ret.cost + ring.cost,
                            damage: ret.damage + ring.damage,
                            armor: ret.armor + ring.armor,
                            ..ret
                        }
                    }
                    ret
                }) {
                    for armorchoice in shop.armor.iter().combinations(armoramount).map(|arr| {
                        let mut ret = Item {
                            name: "armor",
                            cost: 0,
                            damage: 0,
                            armor: 0,
                        };
                        for armor in arr {
                            ret = Item {
                                cost: ret.cost + armor.cost,
                                damage: ret.damage + armor.damage,
                                armor: ret.armor + armor.armor,
                                ..ret
                            }
                        }
                        ret
                    }) {
                        let cost = weapon.cost + ringchoice.cost + armorchoice.cost;
                        let player = Player {
                            hitpoints: 100,
                            damage: weapon.damage + ringchoice.damage + armorchoice.damage,
                            armor: weapon.armor + ringchoice.armor + armorchoice.armor,
                        };
                        if cost > maxgold {
                            if !evalFight(player, boss.clone()) {
                                maxgold = cost;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{maxgold}");
}

fn evalFight(mut player: Player, mut boss: Player) -> bool {
    while boss.hitpoints > 0 && player.hitpoints > 0 {
        boss.hitpoints -= player.damage - boss.armor;
        player.hitpoints -= boss.damage - player.armor;
    }
    player.hitpoints > 0 || boss.hitpoints <= 0
}

fn defineShop() -> Shop<'static> {
    Shop {
        weapons: vec![
            Item {
                name: "Dagger",
                cost: 8,
                damage: 4,
                armor: 0,
            },
            Item {
                name: "Shortsword",
                cost: 10,
                damage: 5,
                armor: 0,
            },
            Item {
                name: "Warhammer",
                cost: 25,
                damage: 6,
                armor: 0,
            },
            Item {
                name: "Longsword",
                cost: 40,
                damage: 7,
                armor: 0,
            },
            Item {
                name: "Greataxe",
                cost: 74,
                damage: 8,
                armor: 0,
            },
        ],
        armor: vec![
            Item {
                name: "Leather",
                cost: 13,
                damage: 0,
                armor: 1,
            },
            Item {
                name: "Chainmail",
                cost: 31,
                damage: 0,
                armor: 2,
            },
            Item {
                name: "Splintmail",
                cost: 53,
                damage: 0,
                armor: 3,
            },
            Item {
                name: "Bandedmail",
                cost: 75,
                damage: 0,
                armor: 4,
            },
            Item {
                name: "Platemail",
                cost: 102,
                damage: 0,
                armor: 5,
            },
        ],
        rings: vec![
            Item {
                name: "Damage",
                cost: 25,
                damage: 1,
                armor: 0,
            },
            Item {
                name: "Damage",
                cost: 50,
                damage: 2,
                armor: 0,
            },
            Item {
                name: "Damage",
                cost: 100,
                damage: 3,
                armor: 0,
            },
            Item {
                name: "Defens",
                cost: 20,
                damage: 0,
                armor: 1,
            },
            Item {
                name: "Defense",
                cost: 40,
                damage: 0,
                armor: 2,
            },
            Item {
                name: "Defense",
                cost: 80,
                damage: 0,
                armor: 3,
            },
        ],
    }
}
