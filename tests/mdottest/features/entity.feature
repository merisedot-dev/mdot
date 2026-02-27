Feature: Managing individual entities

    Scenario: Empty entity
        When we build a new entity named "test"
        Then the entity is named "test"
        And the entity has 0 attributes

    Scenario Outline: Attribute type in entity
        Given a new entity named "<name>"
        And a new attribute "<attr>" of type <attrtype>
        And the entity has <nb> attributes
        When we add the attribute in the entity
        Then the entity has <nc> attributes
        And the entity has an attribute named "<attr>"
        And the attribute "<attr>" is of type <attrtype>

        Examples:
            |name |attr       |attrtype   |nb|nc|
            |test |t          |int        |0 |1 |
            |graou|tt         |bool       |1 |2 |
            |jade |pier       |varchar(42)|4 |5 |
            |tiger|eye        |varchar(2) |10|11|
            |billy|truc       |text       |3 |4 |
            |pommy|testopommes|uuid       |9 |10|
            |truc |blep       |char(6)    |20|21|

    Scenario Outline: Deleting an attribute
        Given a new entity named "<name>"
        And the entity has an attribute "<attr>" of type <attrtype>
        When the attribute "<attr>" is deleted from the entity
        Then the entity has 0 attributes
        And the entity doesn't have an attribute named "<attr>"

        Examples:
            |name   |attr     |attrtype     |
            |mew    |id       |int          |
            |saphir |caillasse|text         |
            |rubis  |graou    |uuid         |
            |diamond|truc     |bool         |
            |holl   |vide     |varchar(100) |
            |hell   |orne     |varchar(6666)|
            |truc   |muche    |char(1234)   |

    Scenario Outline: Attribute roles
        Given a new entity named "<name>"
        And the entity has <nb> attributes
        And a new attribute "<attr>" of type <attrtype>
        And the attribute has the "<role>" role
        When we add the attribute in the entity
        Then the entity has an attribute named "<attr>"
        And the attribute "<attr>" is of type <attrtype>
        And the attribute "<attr>" is of role "<role>"

        Examples:
            |name |nb|attr  |attrtype   |role|
            |test |0 |test  |uuid       |PK  |
            |grou |10|graou |varchar(42)|PK  |
            |boos |5 |tttt  |bool       |PK  |
            |rubis|4 |pierre|text       |PK  |
            |rat  |99|caille|int        |PK  |
            |jade |0 |stel  |char(5)    |PK  |
