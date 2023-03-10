MODULE todo_list
    use stdlib_string_type, only: string_type
#define String string_type

    use stdlib_hashmap_wrappers, only: &
        get, key_type, set

    IMPLICIT NONE
    PUBLIC :: TodoList

    TYPE :: TodoItem
        TYPE(String):: title
        TYPE(String):: description
        LOGICAL     :: done
    END TYPE

    TYPE :: TodoList
        TYPE(String) :: title
        TYPE(String) :: description
        ! HashMap<String, TodoItem>
        TYPE(key_type), DIMENSION(:), ALLOCATABLE :: items

        CONTAINS
            PROCEDURE, PASS :: create => todo_list_create
            PROCEDURE, PASS :: add_item => todo_list_add_item
            PROCEDURE, PASS :: remove_item => todo_list_remove_item
            PROCEDURE, PASS :: get_item => todo_list_get_item
            PROCEDURE, PASS :: complete_item => todo_list_complete_item
            PROCEDURE, PASS :: uncomplete_item => todo_list_uncomplete_item
            PROCEDURE, PASS :: save => todo_list_save
    END TYPE

CONTAINS

    SUBROUTINE todo_list_create(self, title, description)
        CLASS(TodoList), INTENT(INOUT) :: self
        TYPE(String), INTENT(IN) :: title
        TYPE(String), INTENT(IN) :: description
        self%title = title
        self%description = description
    END SUBROUTINE

    SUBROUTINE todo_list_add_item(self, title, description)
        CLASS(TodoList), INTENT(INOUT) :: self
        TYPE(String), INTENT(IN) :: title
        TYPE(String), INTENT(IN) :: description
        TYPE(TodoItem) :: item
        item%title = title
        item%description = description
        item%done = .FALSE.
        ! self%items%set(title, item)
    END SUBROUTINE

    SUBROUTINE todo_list_remove_item(self, title)
        CLASS(TodoList), INTENT(INOUT) :: self
        TYPE(String), INTENT(IN) :: title
        ! self%items%remove(title)
    END SUBROUTINE

    SUBROUTINE todo_list_get_item(self, title, item)
        CLASS(TodoList), INTENT(INOUT) :: self
        TYPE(String), INTENT(IN) :: title
        TYPE(TodoItem), INTENT(OUT) :: item
        ! item = self%items%get(title)
    END SUBROUTINE

    SUBROUTINE todo_list_complete_item(self, title)
        CLASS(TodoList), INTENT(INOUT) :: self
        TYPE(String), INTENT(IN) :: title
        ! self%items%get(title)%done = .TRUE.
    END SUBROUTINE

    SUBROUTINE todo_list_uncomplete_item(self, title)
        CLASS(TodoList), INTENT(INOUT) :: self
        TYPE(String), INTENT(IN) :: title
        ! self%items%get(title)%done = .FALSE.
    END SUBROUTINE

    SUBROUTINE todo_list_save(self, file)
        CLASS(TodoList),INTENT(INOUT) :: self
        TYPE(String),   INTENT(IN) :: file

        ! TODO
    END SUBROUTINE

END MODULE
