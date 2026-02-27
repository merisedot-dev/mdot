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
            |bobo       |9 |khe  |bool         |10|

    Scenario Outline: Deleting attributes from GraphLink
        Given a new GraphLink named "<name>"
        And the GraphLink has an attribute named "<attr>" of type <attrtype>
        When we delete the attribute "<attr>" from GraphLink
        Then the GraphLink doesn't have an attribute named "<attr>"

        Examples:
            |name    |attr |attrtype  |
            |graphite|gr   |int       |
            |brome   |br   |bool      |
            |alcal   |alk  |uuid      |
            |book    |paper|text      |
            |light   |lt   |varchar(4)|
            |flak    |fl   |char(10)  |

    Scenario: Linking entities
        Given a new GraphLink named "ltest"
        And a new entity named "entest"
        When we add a link on "ltest" to "entest"
        Then the GraphLink does know "entest"

    Scenario: Unlinking entities
        Given a new GraphLink named "lutest"
        And a new entity named "entest"
        And the entity "entest" is known by "lutest"
        When we remove the link on "lutest" to "entest"
        Then the GraphLink does not know "entest"
