use crate::{errors::AppError, utils};

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Indexed {
    pub id: i64,
    #[sqlx(flatten)]
    pub contact: Contact,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Update {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndexedUpdate {
    pub id: i64,
    pub update: Update,
}
impl IndexedUpdate {
    fn new(id: i64) -> Self {
        Self {
            id,
            update: Update {
                first_name: None,
                last_name: None,
                display_name: None,
                email: None,
                phone_number: None,
            },
        }
    }

    pub fn first_name(mut self, first_name: &str) -> Self {
        self.update.first_name = Some(first_name.to_string());
        return self;
    }

    pub fn last_name(mut self, last_name: &str) -> Self {
        self.update.last_name = Some(last_name.to_string());
        return self;
    }

    pub fn email(mut self, email: &str) -> Self {
        self.update.email = Some(email.to_string());
        return self;
    }

    pub fn display_name(mut self, display_name: &str) -> Self {
        self.update.display_name = Some(display_name.to_string());
        return self;
    }

    pub fn phone_number(mut self, phone_number: &str) -> Self {
        self.update.phone_number = Some(phone_number.to_string());
        return self;
    }

    pub fn build(self) -> IndexedUpdate {
        IndexedUpdate {
            id: self.id,
            update: self.update,
        }
    }
}

impl Contact {
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_number: &str,
    ) -> Result<Self, AppError> {
        let display_name = format!("{first_name} {last_name}");

        if utils::is_not_valid_email(email) {
            return Err(AppError::InvalidEmail(email.to_owned()));
        }

        if utils::is_not_valid_phone_number(phone_number) {
            return Err(AppError::InvalidPhoneNumber(phone_number.to_owned()));
        }

        Ok(Self {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            display_name,
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::{Contact, IndexedUpdate};

    #[test]
    fn test_display_name() {
        let person = Contact::new("Jason", "Ribble", "john@example.com", "123-456-7890");
        let display_name = "Jason Ribble".to_string();

        assert_eq!(person.unwrap().display_name, display_name)
    }

    #[test]
    fn test_contact_update_builder() {
        let edits = IndexedUpdate::new(1)
            .display_name("Nickname")
            .phone_number("123-233-1221")
            .build();

        assert_eq!(edits.id, 1);
        assert_eq!(edits.update.display_name, Some("Nickname".to_string()));
        assert_eq!(edits.update.phone_number, Some("123-233-1221".to_string()));
        assert_eq!(edits.update.first_name, None);
        assert_eq!(edits.update.last_name, None);
        assert_eq!(edits.update.email, None);
    }

    #[test]
    fn test_contact_update_builder_2() {
        let edits = IndexedUpdate::new(2)
            .first_name("Mary")
            .last_name("Smith")
            .email("new@email.com")
            .build();

        assert_eq!(edits.id, 2);
        assert_eq!(edits.update.first_name, Some("Mary".to_string()));
        assert_eq!(edits.update.last_name, Some("Smith".to_string()));
        assert_eq!(edits.update.email, Some("new@email.com".to_string()));
        assert_eq!(edits.update.phone_number, None);
        assert_eq!(edits.update.display_name, None);
    }
}
