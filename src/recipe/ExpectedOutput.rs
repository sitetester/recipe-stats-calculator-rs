use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::recipe::calculator::CustomPostcodeDeliveryTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedOutput {
    #[serde(rename(serialize = "unique_recipe_count"))]
    uniqueRecipeCount: i32,
    #[serde(rename(serialize = "count_per_recipe"))]
    sortedRecipesCount: Vec<CountPerRecipe>,
    #[serde(rename(serialize = "busiest_postcode"))]
    busiestPostcode: BusiestPostcode,
    #[serde(rename(serialize = "count_per_postcode_and_time"))]
    countPerPostcodeAndTime: CountPerPostcodeAndTime,
    #[serde(rename(serialize = "match_by_name"))]
    sortedRecipeNames: Vec<String>,

    #[serde(rename(serialize = "total_json_objects"))]
    totalObjects: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct BusiestPostcode {
    #[serde(rename(serialize = "postcode"))]
    postcode: String,
    #[serde(rename(serialize = "delivery_count"))]
    deliveryCount: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountPerRecipe {
    recipe: String,
    count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountPerPostcodeAndTime {
    postcode: String,
    #[serde(rename(serialize = "from"))]
    fromAM: String,
    #[serde(rename(serialize = "to"))]
    toPM: String,
    #[serde(rename(serialize = "delivery_count"))]
    deliveryCount: i32,
}

pub fn getExpectedOutput(
    countPerRecipe: &HashMap<String, i32>,
    countPerPostcode: &HashMap<String, i32>,
    deliveriesCountPerPostcode: &HashMap<String, i32>,
    customPostcodeDeliveryTime: &CustomPostcodeDeliveryTime,
    filteredRecipeNames: &mut Vec<String>,
    totalObjects: i64,
) -> ExpectedOutput {
    filteredRecipeNames.sort();

    let deliveryCount =
        if deliveriesCountPerPostcode.contains_key(&customPostcodeDeliveryTime.postcode) {
            deliveriesCountPerPostcode[&customPostcodeDeliveryTime.postcode]
        } else {
            0
        };

    let expectedOutput = ExpectedOutput {
        uniqueRecipeCount: getUniqueRecipeCount(&countPerRecipe),
        sortedRecipesCount: getSortedRecipeCount(&countPerRecipe),
        busiestPostcode: getBusiestPostcode(countPerPostcode),
        countPerPostcodeAndTime: getDeliveriesCountForPostCode(
            customPostcodeDeliveryTime,
            deliveryCount,
        ),
        sortedRecipeNames: filteredRecipeNames.to_owned(),
        totalObjects,
    };

    expectedOutput
}

// counts the number of unique recipe names
fn getUniqueRecipeCount(countPerRecipe: &HashMap<String, i32>) -> i32 {
    let mut uniqueRecipeCount = 0;
    for (_, count) in countPerRecipe {
        if *count == 1 as i32 {
            uniqueRecipeCount += 1;
        }
    }

    uniqueRecipeCount
}

// counts the number of occurrences for each unique recipe name (alphabetically ordered by recipe name)
fn getSortedRecipeCount(countPerRecipe: &HashMap<String, i32>) -> Vec<CountPerRecipe> {
    let mut v: Vec<_> = countPerRecipe.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));

    let vCountPerRecipe: Vec<CountPerRecipe> = v
        .into_iter()
        .map(|(recipe, &count)| CountPerRecipe {
            recipe: recipe.to_string(),
            count,
        })
        .collect();

    vCountPerRecipe
}

// finds the postcode with most delivered recipes
fn getBusiestPostcode(countPerPostcode: &HashMap<String, i32>) -> BusiestPostcode {
    let mut v: Vec<_> = countPerPostcode.into_iter().collect();
    v.sort_by(|x, y| x.1.cmp(&y.1));
    let last = v.pop().unwrap();

    BusiestPostcode {
        postcode: last.0.to_string(),
        deliveryCount: *last.1,
    }
}

// counts the number of deliveries to postcode `10120` that lie within the delivery time between `10AM` and `3PM`
fn getDeliveriesCountForPostCode(
    customPostcodeDeliveryTime: &CustomPostcodeDeliveryTime,
    deliveryCount: i32,
) -> CountPerPostcodeAndTime {
    CountPerPostcodeAndTime {
        postcode: customPostcodeDeliveryTime.postcode.to_string(),
        fromAM: format!("{}{}", customPostcodeDeliveryTime.from, "AM"),
        toPM: format!("{}{}", customPostcodeDeliveryTime.to, "PM"),
        deliveryCount,
    }
}
