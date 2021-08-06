use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

use super::ExpectedOutput;

pub struct StatsCalculator {
    pub(crate) customPostcodeDeliveryTime: CustomPostcodeDeliveryTime,
    pub(crate) customRecipeNames: Vec<String>,
}

#[derive(Debug)]
pub struct CustomPostcodeDeliveryTime {
    pub postcode: String,
    pub(crate) from: i32,
    pub(crate) to: i32,
}

#[derive(Deserialize, Debug)]
pub struct RecipeData {
    postcode: String,
    recipe: String,
    delivery: String,
}

impl StatsCalculator {
    pub fn calculateStats(&self, rx: Receiver<RecipeData>) -> ExpectedOutput::ExpectedOutput {
        let mut countPerRecipe: HashMap<String, i32> = HashMap::new();
        let mut countPerPostcode: HashMap<String, i32> = HashMap::new();
        let mut deliveriesCountPerPostcode: HashMap<String, i32> = HashMap::new();
        let mut filteredRecipeNames: Vec<String> = vec![];

        for recipeData in rx {
            self.calculateCountPerRecipe(recipeData.recipe.as_str(), &mut countPerRecipe);
            self.calculateCountPerPostcode(recipeData.postcode.as_str(), &mut countPerPostcode);
            self.calculateDeliveriesCountPerPostcode(&recipeData, &mut deliveriesCountPerPostcode);
            self.filterRecipeName(&recipeData.recipe, &mut filteredRecipeNames);
        }

        ExpectedOutput::getExpectedOutput(
            &countPerRecipe,
            &countPerPostcode,
            &deliveriesCountPerPostcode,
            &self.customPostcodeDeliveryTime,
            &mut filteredRecipeNames,
        )
    }

    fn calculateCountPerRecipe(&self, recipe: &str, countPerRecipe: &mut HashMap<String, i32>) {
        countPerRecipe.insert(
            recipe.parse().unwrap(),
            1 + if countPerRecipe.contains_key(recipe) {
                countPerRecipe[recipe]
            } else {
                0
            },
        );
    }

    fn calculateCountPerPostcode(
        &self,
        postcode: &str,
        countPerPostcode: &mut HashMap<String, i32>,
    ) {
        countPerPostcode.insert(
            postcode.parse().unwrap(),
            1 + if countPerPostcode.contains_key(postcode) {
                countPerPostcode[postcode]
            } else {
                0
            },
        );
    }

    fn calculateDeliveriesCountPerPostcode(
        &self,
        recipeData: &RecipeData,
        deliveriesCountPerPostcode: &mut HashMap<String, i32>,
    ) {
        if recipeData.postcode == self.customPostcodeDeliveryTime.postcode
            && self.isWithinDeliveryTime(recipeData.delivery.as_str())
        {
            let postcode = &recipeData.postcode;
            deliveriesCountPerPostcode.insert(
                postcode.parse().unwrap(),
                1 + if deliveriesCountPerPostcode.contains_key(postcode.as_str()) {
                    deliveriesCountPerPostcode[postcode]
                } else {
                    0
                },
            );
        }
    }

    fn isWithinDeliveryTime(&self, delivery: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d{0,2})AM\s-\s(\d{0,2})PM").unwrap();
        }

        if RE.is_match(delivery) {
            let (from, to) = (
                &RE.captures(delivery).unwrap()[1],
                &RE.captures(delivery).unwrap()[2],
            );

            let from: i32 = from.parse().unwrap();
            let to: i32 = to.parse().unwrap();

            if from >= self.customPostcodeDeliveryTime.from
                && to <= self.customPostcodeDeliveryTime.to
            {
                return true;
            }
        }

        false
    }

    fn filterRecipeName(&self, recipe: &str, filteredRecipeNames: &mut Vec<String>) {
        for customRecipeName in &self.customRecipeNames {
            if recipe
                .to_lowercase()
                .contains(&customRecipeName.to_lowercase())
                && !filteredRecipeNames.contains(&recipe.to_string())
            {
                filteredRecipeNames.push(recipe.to_string())
            }
        }
    }
}
