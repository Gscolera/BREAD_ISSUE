const START_AMOUNT: u32 = 10;
const MAX_AMOUNT: u32 = 30;

fn main() {
   
    println!("{:?}", calculate_purchasing_plan(60, &[(15, 200), (10, 100), (35, 500), (50, 30)])); 

}

fn calculate_purchasing_plan(total_days: u32, sellers: &[(u32, u32)]) -> Option<Vec<u32>> {
    let mut sellers = sellers.to_vec();
    let mut plan = vec![0_u32; sellers.len()];
    let mut bread = START_AMOUNT;

    // FIRST OF ALL WE NEED INPUT TO BE SURE THAT INPUT IS VALID AND SORTED
    if !validate_input_params(total_days, &mut sellers) {
        return None
    }

    // THE SECONS STEP IS TO CALCULATE HOW MUCH DO WE NEED TO BY FROM EACH SELLER
    // AND PUT THE RESULT INTO THE PLAN 
    for i in 0..sellers.len() - 1 {
        plan[i] = plan_for_current_seller(&sellers[i..], bread);
        bread += plan[i];
    }
    Some(plan)
}

fn plan_for_current_seller(sellers: &[(u32, u32)], bread: u32) -> u32 {
    //DAYS PASSED AND THE PRICE OF CURRENT SELLER
    let (passed, price) = sellers[0];
    // CURRENT BREAD AMOUNT
    let current_bread_amount = bread - passed;
    
    //WE START FROM THE NEXT SELLER AND LOOKING FOR THE BEST PRICE
    // IF IT FOUND, WE BUY BREAD TO EAT (IF WE ALLREADY NAVE NOT ENOUGH) UNTIL HE COMES TO US
    // AND IF NOT, WE BUY AS MUCH BREAD AS POSSIBLE
    for i in 1..sellers.len() {
        if sellers[i].0 - passed > MAX_AMOUNT {
            return MAX_AMOUNT - current_bread_amount
        }
        if sellers[i].1 < price {
            return if bread < sellers[i].0 { sellers[i].0 - bread } else { 0 } 
        }
    }
    0
}

fn validate_input_params(total_days: u32, sellers: &mut Vec<(u32, u32)>) -> bool {
    let sellers_amount = sellers.len();

    if sellers_amount == 0 && total_days > START_AMOUNT {
        return false;
    }

    sellers.sort_by(|(days1, _), (days2, _)| days1.partial_cmp(days2).unwrap());
    sellers.push((total_days, 0));

    for i in 0..sellers.len() - 1 {
        if sellers[i].0 >= total_days {
            sellers.truncate(i);
            return true
        }
        
        if sellers[i].0 + MAX_AMOUNT < sellers[i + 1].0 {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_answer_cases() {
        assert_eq!(None, calculate_purchasing_plan(100, &[(11, 1)]));
        assert_eq!(
            None,
            calculate_purchasing_plan(100, &[(10, 1), (40, 1), (71, 1)])
        );
        assert_eq!(
            None,
            calculate_purchasing_plan(100, &[(101, 1), (40, 1), (70, 1)])
        );

        assert_eq!(
            None,
            calculate_purchasing_plan(60, &[(9, 202), (10, 201), (11, 205), (12, 203), (13, 204), (44, 200)])
        );
    }

    #[test]
    fn regular_cases() {
        assert_eq!(
            Some(vec![5, 30, 5, 10]),
            calculate_purchasing_plan(60, &[(10, 200), (15, 100), (35, 500), (50, 30)])
        );

        assert_eq!(
            Some(vec![0]),
            calculate_purchasing_plan(10, &[(9, 200)])
        );

        assert_eq!(
            Some(vec![29, 1, 5, 5, 4, 6]),
            calculate_purchasing_plan(60, &[(9, 200), (10, 201), (15, 202), (20, 203), (25, 204), (54, 200)])
        );

        assert_eq!(
            Some(vec![5, 0, 30, 5, 5, 5]),
            calculate_purchasing_plan(60, &[(9, 200), (10, 201), (15, 199), (20, 203), (25, 204), (55, 200)])
        );

        assert_eq!(
            Some(vec![0, 30, 0, 10, 5, 5]),
            calculate_purchasing_plan(60, &[(9, 202), (10, 201), (15, 205), (20, 203), (25, 204), (55, 200)])
        );
    }

    #[test]
    fn edge_cases() {
        assert_eq!(None, calculate_purchasing_plan(100, &[]));
        assert_eq!(Some(vec![]), calculate_purchasing_plan(5, &[]));
        assert_eq!(Some(vec![]), calculate_purchasing_plan(0, &[]));
    }
}
