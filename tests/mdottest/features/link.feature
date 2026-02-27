Feature: Managing GraphLinks.

    Scenario: Empty GraphLink
        When we build a new GraphLink named "test"
        Then the GraphLink is named "test"
        And the GraphLink has 0 attributes
        And the GraphLink has 0 known entities

    Scenario Outline: Adding attribute to GraphLink
        Given a new GraphLink named "<name>"
        And the GraphLink has <nb> attributes
        And a new attribute "<attr>" of type <attrtype>
        When we add the attribute to the GraphLink
        Then the GraphLink has <nc> attributes
        And the GraphLink has an attribute named "<attr>"
        And the GraphLink attribute "<attr>" is of type <attrtype>

        Examples:
            |name       |nb|attr |attrtype     |nc|
            |jaaj       |0 |test |int          |1 |
            |mew        |5 |mew  |varchar(1000)|6 |
            |testopommes|10|graou|text         |11|
            |graou      |1 |bite |char(66)     |2 |
            |lonk       |5 |lid  |uuid         |6 |

    Scenario Outline: Deleting attributes from GraphLink
        Given a new GraphLink named "<name>"
        And an attribute named "<attr>" of type <attrtype> in GraphLink
        When we remove the attribute "<attr>" from the GraphLink
        Then the GraphLink doesn't have an attribute named "<attr>"

        Examples:
            |name|attr|attrtype|

    Scenario Outline: Known entities
        Given a new GraphLink named "<name>"

        Examples:
            |name|
