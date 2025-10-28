use std::cmp::PartialEq;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: Implement the `PartialEq` trait for `Ticket`
// t1.eq(&t2); // here, inside eq(), `self` = t1, `other` = t2
// Self here refers to the type being implemented, which is Ticket
// So &Self means “a reference to a Ticket”
// self refers to the current instance of Ticket
// When you write self.title, you’re accessing the title field of this specific Ticket
impl PartialEq for Ticket {
    fn eq(&self, other: &Self)-> bool {
        self.title == other.title && self.description == other.description && self.status == other.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 == ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert!(ticket1 != ticket2);
    }
}
