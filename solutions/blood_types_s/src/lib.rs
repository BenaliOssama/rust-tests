#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}
use Antigen::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor != other.rh_factor && self.rh_factor == RhFactor::Negative {
            return false;
        }

        if other.antigen == Antigen::O {
            return true;
        }

        self.antigen == Antigen::AB || other.antigen == self.antigen
    }

    // who are the donors of self
    pub fn donors(&self) -> Vec<Self> {
        // all blood types A, B, AB, O
        let mut blood_types = Vec::new();
        let mut antigens = if self.antigen == Antigen::O {
            vec![Antigen::O]
        } else {
            vec![Antigen::O, self.antigen.clone()]
        };

        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Negative]
        } else {
            vec![RhFactor::Positive, RhFactor::Negative]
        };

        if self.antigen == Antigen::AB {
            antigens.push(Antigen::A);
            antigens.push(Antigen::B);
        }

        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                });
            }
        }

        blood_types
    }

    pub fn recipients(&self) -> Vec<Self> {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_ab_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_ab_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_neg_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_pos_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_compatible_ab_neg_with_o_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_antigen_ab_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }

    #[test]
    fn test_antigen_a_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);
    }

    #[test]
    fn test_donors() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut givers = blood_type.donors();
        // println!("Before sorting {:?}", &givers);
        givers.sort();
        // println!("{:?}", &givers);
        let mut expected = vec![
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            }
        ];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_a_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            }
        ];

        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_o_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };

        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_ab_pos_recipients() {
        let blood = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_a_neg_recipients() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };

        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![
            blood.clone(),
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            }
        ];
        expected.sort();
        assert_eq!(recipients, expected);
    }
}
