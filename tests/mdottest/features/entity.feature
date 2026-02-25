Feature: Managing individual entities

    Scenario: Empty entity
        When we build a new entity named "test"
        Then the entity is named "test"
        And the entity has 0 attributes

    Scenario Outline: Attribute type in entity
        Given a new entity named "<name>"
        And a new attribute "<attr>" of type <attrtype>
        When we add the attribute in the entity
        Then the entity has 1 attributes
        And the entity has an attribute named "<attr>"
        And the attribute "<attr>" is of type <attrtype>

        Examples:
            |name |attr       |attrtype   |
            |test |t          |int        |
            |graou|tt         |bool       |
            |jade |pier       |varchar(42)|
            |billy|truc       |text       |
            |pommy|testopommes|uuid       |

    Scenario Outline: Deleting an attribute
        Given a new entity named "<name>"
        And the entity has an attribute "<attr>" of type <attrtype>
        And we add the attribute in the entity
        When the attribute "<attr>" is deleted from the entity
        Then the entity has 0 attributes
        And the entity doesn't have an attribute named "<attr>"

        Examples:
            |name  |attr     |attrtype|
            |mew   |id       |int     |
            |saphir|caillasse|text    |
            |rubis |graou    |uuid    |
