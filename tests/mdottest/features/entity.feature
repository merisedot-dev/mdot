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
            |name |attr|attrtype|
            |test |t   |int     |
            |graou|tt  |bool    |
