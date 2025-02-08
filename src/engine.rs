use core::fmt;
use std::{collections::HashMap, fmt::{Display, Formatter}, ops::Index, vec::Vec};

#[derive(Debug)]
struct ValueMultiplicityPair {
    pub value: i64,
    pub multiplicity: i64
}

impl ValueMultiplicityPair {
    pub fn sum(&self) -> i64 {
        self.value * self.multiplicity
    }
}

#[derive(Debug)]
pub struct LuckyNumberConstruction {
    values_with_multiplicities: Vec<ValueMultiplicityPair>
}

impl LuckyNumberConstruction {
    pub fn from_factor_partials(common_factor: i64, common_factor_factors: &Vec<i64>, counts: &Vec<i64>) -> Self {
        let mut values_with_multiplicities = Vec::new();
        values_with_multiplicities.push(ValueMultiplicityPair {value: common_factor, multiplicity: counts[counts.len() - 1]});

        for (i, factor) in common_factor_factors.iter().enumerate() {
            if counts[i] == 0 {continue;}

            let deonominator_value = common_factor / factor;

            values_with_multiplicities.push(ValueMultiplicityPair {value: deonominator_value, multiplicity: counts[i]});

        }

        
        LuckyNumberConstruction {values_with_multiplicities}
    }

    pub fn calulate_value(&self) -> i64 {
        let mut output_sum = 0;
        for set in self.values_with_multiplicities.iter() {
            output_sum += set.sum();
        }
        return  output_sum;
    }
}


impl Display for LuckyNumberConstruction {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        for (i, set) in self.values_with_multiplicities.iter().enumerate() {
            if set.multiplicity > 1 {
                write!(formatter, "{}*{}", set.value, set.multiplicity)?
            } else {
                write!(formatter, "{}", set.value)?
            }
            
            if i != self.values_with_multiplicities.len() - 1 {
                write!(formatter, " + ")?
            }
        }
        Ok(())
    }
}

pub struct LuckyNumberEngine {
    min_cutoff: i64,
    max_cutoff: i64,
    largest_factor_checked: i64,
    lucky_findings: HashMap<i64, Vec<LuckyNumberConstruction>>,
    empty_reference: Vec<LuckyNumberConstruction>,
}

impl LuckyNumberEngine {
    pub fn new() -> Self {
        LuckyNumberEngine {
            min_cutoff: 0,
            max_cutoff: 0,
            largest_factor_checked: 0,
            lucky_findings: HashMap::new(),
            empty_reference: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        self.largest_factor_checked = 0;
        self.lucky_findings = HashMap::new();
    }


    pub fn luckiness(&mut self, value: i64) -> i64 {
        match self.lucky_findings.get(&value) {
            Some(x) => x.len() as i64,
            None => 0,
        }
    }

    pub fn constructions(&mut self, value: i64) -> &Vec<LuckyNumberConstruction> {
        match self.lucky_findings.get(&value) {
            Some(x) => x,
            None => &self.empty_reference,
        }
    }

    pub fn is_lucky(&mut self, value: i64) -> bool {
        match self.lucky_findings.get(&value) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn generate_range(&mut self, range_start: i64, range_end: i64) {
        self.reset();
        self.min_cutoff = range_start;
        self.max_cutoff = range_end;
        self.generate_lucky_numbers_up_to(range_end);
    }


    pub fn generate_lucky_numbers_up_to(&mut self, value: i64) {
        if value <= self.largest_factor_checked {
            return;
        }
        for i in self.largest_factor_checked..=value {
            self.generate_lucky_numbers_from_factor(i);
        }

        self.largest_factor_checked = value;
        
    }

    fn generate_lucky_numbers_from_factor(&mut self, factor: i64) {
        let factors_of_factor = factor::factor::factor(factor);

        let mut numerator_constructions = self.get_int_sum_constructors_from(factor - 1, &factors_of_factor);

        numerator_constructions.iter_mut().for_each(|v| {let i = v.len()-1; v[i] += 1});

        for numerator_construction in numerator_constructions {
            let construction = LuckyNumberConstruction::from_factor_partials(
                factor, 
                &factors_of_factor,
                &numerator_construction
            );

            let value = construction.calulate_value();
            if value <= self.max_cutoff && value >= self.min_cutoff {
                match self.lucky_findings.get_mut(&value) {
                    Some(v) => v.push(construction),
                    None => {self.lucky_findings.insert(value, vec![construction]);},
                }
            }
        }
    }


    fn max_products_till_greater(&self, than: i64, num: i64) -> i64
    {
        let mut count: i64 = 0;
        let mut total= 0;

        loop {
            if total > than {
                return count;
            }
            count += 1;
            total += num;
        }
    }

    fn get_int_sum_constructors_from(&self, sum_to: i64, with: &Vec<i64>) -> Vec<Vec<i64>> {
        if with.len() == 0 {
            return vec![vec![sum_to]];
        }

        let max_value = self.max_products_till_greater(sum_to, with[0]);

        let end_con_nums = with[1..].to_vec();


        let mut output = Vec::new();
        for i in 0..max_value {
            for end_vals in self.get_int_sum_constructors_from(sum_to - i * with[0], &end_con_nums.clone()) {
                let mut new_constructor: Vec<i64> = Vec::new();
                new_constructor.push(i);
                new_constructor.append(&mut end_vals.clone());
                output.push(new_constructor);
            }
        }

        return output;
    }
}