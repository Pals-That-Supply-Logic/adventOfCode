use super::PairPartner;

pub fn is_inside(first_partner: PairPartner, second_partner: PairPartner)->bool{
    first_partner.section_start <= second_partner.section_start && first_partner.section_end >= second_partner.section_end
}

pub fn is_any_overlap(first_partner: PairPartner, second_partner: PairPartner)->bool{
    is_inside(first_partner, second_partner) 
        || first_partner.section_start == second_partner.section_end
        || first_partner.section_end == second_partner.section_start
        || first_partner.section_start < second_partner.section_end && first_partner.section_start > second_partner.section_start
        || first_partner.section_end > second_partner.section_start && first_partner.section_end < second_partner.section_end
}