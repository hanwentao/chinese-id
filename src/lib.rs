use chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PersonalInfo<'a> {
    pub address: &'a str,
    pub date_of_birth: NaiveDate,
    pub order: u16,
    pub gender: Gender,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    InvalidLength,
    InvalidCharacters,
    InvalidDate,
    ChecksumError,
}

pub type Result<'a> = std::result::Result<PersonalInfo<'a>, ValidationError>;

const LENGTH: usize = 18;

pub fn validate(id: &str) -> Result<'_> {
    if id.len() != LENGTH {
        return Err(ValidationError::InvalidLength);
    }

    let mut sum = 0;
    for (i, c) in id.chars().rev().enumerate() {
        if i == 0 && c.to_ascii_uppercase() == 'X' {
            sum += 10;
        } else if let Some(d) = c.to_digit(10) {
            sum += d * ((1 << i as u32) % 11);
        } else {
            return Err(ValidationError::InvalidCharacters);
        }
    }

    if sum % 11 == 1 {
        let address = &id[0..6];
        let date_of_birth = match NaiveDate::parse_from_str(&id[6..14], "%Y%m%d") {
            Ok(date) => date,
            Err(_) => return Err(ValidationError::InvalidDate),
        };
        let order = id[14..17].parse().unwrap();
        Ok(PersonalInfo {
            address,
            date_of_birth,
            order,
            gender: if order % 2 == 0 {
                Gender::Female
            } else {
                Gender::Male
            },
        })
    } else {
        Err(ValidationError::ChecksumError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert_eq!(validate(""), Err(ValidationError::InvalidLength));
        assert_eq!(
            validate("abcdefghijklmnopqr"),
            Err(ValidationError::InvalidCharacters)
        );
        assert_eq!(
            validate("11010220000101XXXX"),
            Err(ValidationError::InvalidCharacters)
        );
        assert_eq!(
            validate("110102199902290014"),
            Err(ValidationError::InvalidDate)
        );
        assert_eq!(
            validate("11010220000101001X"),
            Err(ValidationError::ChecksumError)
        );
        assert_eq!(
            validate("110102200002290014"),
            Ok(PersonalInfo {
                address: "110102",
                date_of_birth: NaiveDate::from_ymd_opt(2000, 2, 29).unwrap(),
                order: 1,
                gender: Gender::Male
            })
        );
        assert_eq!(
            validate("110102200002291009"),
            Ok(PersonalInfo {
                address: "110102",
                date_of_birth: NaiveDate::from_ymd_opt(2000, 2, 29).unwrap(),
                order: 100,
                gender: Gender::Female
            })
        );
    }
}
