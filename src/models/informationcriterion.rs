pub enum CriterionFlavor {
    AIC,
    BIC,
}

pub struct InformationCriterion {
    criterion: CriterionFlavor,

}
impl InformationCriterion {
    pub fn new(criterion: CriterionFlavor) -> Self {
        Self { criterion }
    }

    pub fn get_criterion(&self) -> &CriterionFlavor {
        return &self.criterion;
    }

    fn log_likelihood(&self) -> f64 {

        return 0.0;
    }

    pub fn calculate_IC(&self) -> f64 {
        match self.criterion {
            CriterionFlavor::AIC => {
                return 2.0 * self.log_likelihood() - 2.0
            },
            CriterionFlavor::BIC => {
                return self.log_likelihood() - 0.5 * (self.log_likelihood().log(2.0))
            }
        }
    }
}