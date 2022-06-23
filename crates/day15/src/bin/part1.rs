use std::{io::BufRead, collections::HashMap};

use shared::{combine::{*, parser::{char::string, repeat::take_until}}, anyhow, parse::i64};

#[derive(Debug,Clone,Copy)]
pub struct Properties{
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64
}

impl Properties{
    pub fn score(self) -> i64{
        self.capacity.max(0)
        * self.durability.max(0)
        * self.flavor.max(0)
        * self.texture.max(0)
    }
}

impl std::ops::Mul<i64> for Properties{
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Properties{
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl std::ops::AddAssign for Properties{
    fn add_assign(&mut self, rhs: Self) {
        self.capacity += rhs.capacity;
        self.durability += rhs.durability;
        self.flavor += rhs.flavor;
        self.texture += rhs.texture;
        self.calories += rhs.calories;
    }
}

pub struct Ingredient{
    name: String,
    properties: Properties,
}

pub fn parser<Input>() -> impl Parser<Input, Output = Vec<Ingredient>>
where
    Input: Stream<Token = char>,
{
    sep_end_by1(struct_parser!(
        Ingredient{
            name: take_until(string(":")),
            _: string(": capacity "),
            properties: struct_parser!(Properties{
                capacity: i64(),
                _: string(", durability "),
                durability: i64(),
                _: string(", flavor "),
                flavor: i64(),
                _: string(", texture "),
                texture: i64(),
                _: string(", calories "),
                calories: i64(),
            })
        }),shared::parse::lax_newline())
}

fn solution(mut input: impl BufRead) -> shared::Result<i64> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (ingredients, rest) = parser()
        .easy_parse(shared::combine::stream::position::Stream::new(&*buf))
        .map_err(|err| anyhow!(err.map_range(|s| s.to_string())))?;
    assert!(rest.input.len() == 0);

    let ingredient_lookup: HashMap<_,_> = ingredients.iter().map(|ingredient| (&ingredient.name,&ingredient.properties)).collect();

    let recipes = sum_combinations(ingredients.len(), 100)
        .map(|counts|{
            ingredients.iter().enumerate().map(|(i,ingredient)|{
                (&ingredient.name,counts[i])
            }).collect::<HashMap<_,_>>()
        });

    let recipe = recipes.max_by_key(|recipe|{
        score_recipe(recipe, &ingredient_lookup)
    }).expect("Should be a best recipe");

    dbg!(&recipe);

    Ok(score_recipe(&recipe, &ingredient_lookup))
}

fn sum_combinations(count: usize, total: i64) -> impl Iterator<Item=Vec<i64>>{
    if count == 1 {
        Box::new(std::iter::once(vec![total])) as Box<dyn Iterator<Item=Vec<i64>>>
    }else{
        Box::new((0..total).flat_map(move |i|{
            sum_combinations(count - 1, total - i).map(move |mut tail|{
                tail.insert(0, i);
                tail
            })
        }))
    }
}

fn score_recipe(recipe: &HashMap<&String,i64>, ingredient_lookup: &HashMap<&String,&Properties>) -> i64 {
    let mut props = Properties{ capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0 };
    for (name,&count) in recipe{
        let ingredient_props = **ingredient_lookup.get(name).unwrap_or_else(|| panic!("Missing ingredient {name}"));
        props += ingredient_props * count;
    }

    props.score()
}

shared::main!(solution);

#[cfg(test)]
#[test]
fn day15_part1_example() {
    shared::check_example(solution, "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3", 62842880)
}
