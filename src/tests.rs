#[cfg(test)]
mod tests {
    use crate::db::models::TodoModel;
    use crate::db::Database;
    use crate::{AddOperation, Command, ListOperation, RemoveOperation, Todo, TodoOperation};

    #[test]
    fn test_add_todo() {
        let db = Database::init().unwrap();
        let todo = TodoModel {
            id: 0,
            title: "Test Todo".to_owned(),
            description: "Test Description".to_owned(),
            completed: false,
        };
        let result = db.add_todo(todo);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_todos() {
        let db = Database::init().unwrap();
        let result = db.list_todos();
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_todo() {
        let db = Database::init().unwrap();
        let result = db.remove_todo(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_todo() {
        let db = Database::init().unwrap();
        let todo = TodoModel {
            id: 1,
            title: "Updated Todo".to_owned(),
            description: "Updated Description".to_owned(),
            completed: true,
        };
        let result = db.update_todo(todo);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_operation_parse_no_args() {
        let args = vec![];
        let result = ListOperation::parse(args.into_iter());
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), TodoOperation::List(_)));
    }

    #[test]
    fn test_list_operation_parse_with_args() {
        let args = vec!["extra".to_string()];
        let result = ListOperation::parse(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_add_operation_parse_no_args() {
        let args = vec![];
        let result = AddOperation::parse(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_add_operation_parse_with_args() {
        let args = vec!["New Todo".to_string()];
        let result = AddOperation::parse(args.into_iter());
        assert!(result.is_ok());
        if let TodoOperation::Add(add_op) = result.unwrap() {
            assert_eq!(add_op.title, "New Todo");
        } else {
            panic!("Expected AddOperation");
        }
    }

    #[test]
    fn test_add_operation_parse_with_extra_args() {
        let args = vec!["New Todo".to_string(), "Extra".to_string()];
        let result = AddOperation::parse(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_operation_parse_no_args() {
        let args = vec![];
        let result = RemoveOperation::parse(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_operation_parse_with_invalid_arg() {
        let args = vec!["invalid".to_string()];
        let result = RemoveOperation::parse(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_operation_parse_with_valid_arg() {
        let args = vec!["123".to_string()];
        let result = RemoveOperation::parse(args.into_iter());
        assert!(result.is_ok());
        if let TodoOperation::Remove(remove_op) = result.unwrap() {
            assert_eq!(remove_op.id, 123);
        } else {
            panic!("Expected RemoveOperation");
        }
    }

    #[test]
    fn test_todo_build_no_args() {
        let args = vec!["program".to_string()];
        let result = Todo::build(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_todo_build_with_invalid_command() {
        let args = vec!["program".to_string(), "invalid".to_string()];
        let result = Todo::build(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_todo_build_with_valid_list_command() {
        let args = vec!["program".to_string(), "list".to_string()];
        let result = Todo::build(args.into_iter());
        assert!(result.is_ok());
    }

    #[test]
    fn test_todo_build_with_valid_add_command() {
        let args = vec!["program".to_string(), "add".to_string(), "New Todo".to_string()];
        let result = Todo::build(args.into_iter());
        assert!(result.is_ok());
    }

    #[test]
    fn test_todo_build_with_valid_remove_command() {
        let args = vec!["program".to_string(), "remove".to_string(), "123".to_string()];
        let result = Todo::build(args.into_iter());
        assert!(result.is_ok());
    }
}
