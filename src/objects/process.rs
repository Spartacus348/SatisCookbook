use crate::objects::building::Buildings;
use crate::objects::part::Part;
use crate::tiers::Tier;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Process<'a> {
    pub(crate) name: &'a str,
    pub(crate) time_s: usize,
    pub(crate) building: Buildings,
    pub(crate) tier: Tier,
}

impl Process<'_> {
    pub(crate) fn get_input_rate_per_min(self: &Self, search_part: &Part) -> Option<f32> {
        self.building
            .get_input()
            .iter()
            .find_map(|(find_part, amt)| {
                if find_part == search_part {
                    Some(60. * *amt/ (self.time_s as f32))
                } else {
                    None
                }
            })
    }
    pub(crate) fn get_output_rate_per_min(self: &Self, search_part: &Part) -> Option<f32> {
        self.building
            .get_output()
            .iter()
            .find_map(|(find_part, amt)| {
                if find_part == search_part {
                    Some(60. * *amt/ (self.time_s as f32))
                } else {
                    None
                }
            })
    }
}
