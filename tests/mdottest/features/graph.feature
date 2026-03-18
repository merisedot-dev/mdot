Feature: Handling full graphs

    Scenario: Empty Graph
        When we build a new graph
        Then the graph has 0 entities
        And the graph has 0 links

    Scenario: Adding entities
        Given a new graph
        And a new entity named "entest"
        When we add the entity to our graph
        Then the graph has an entity named "entest"

    Scenario: Adding links
        Given a new graph
        And an entity named "ent1" in graph
        And an entity named "ent2" in graph
        When we link "ent1" and "ent2" under the name "ltest"
        Then the graph has a GraphLink named "ltest"
        And the GraphLink "ltest" knows an entity named "ent1"
        And the GraphLink "ltest" knows an entity named "ent2"

    Scenario Outline: Cardinalities
        Given a new graph
        And there are 2 entities in graph
        And the cardinalities on entity 1 will be <n1>,<m1>
        And the cardinalities on entity 2 will be <n2>,<m2>
        When we link entities 1 and 2 together
        Then the graph has a GraphLink named "ctest"
        And the GraphLink "ctest" knows an entity named "e1"
        And the cardinality for entity 1 is <n1>,<m1>
        And the GraphLink "ctest" knows an entity named "e2"
        And the cardinality for entity 2 is <n2>,<m2>

        Examples:
            |n1|m1|n2|m2|
            |0 |n |0 |n |
            |0 |n |1 |n |
            |1 |n |0 |n |
            |0 |1 |0 |n |
            |1 |1 |1 |1 |

    Scenario: Ternaries
        Given a new graph
        And an entity named "ent1" in graph
        And an entity named "ent2" in graph
        And an entity named "ent3" in graph
        When we link "ent1" and "ent2" under the name "ltest"
        And we add "ent3" to the GraphLink "ltest"
        Then the graph has a GraphLink named "ltest"
        And the GraphLink "ltest" knows an entity named "ent1"
        And the GraphLink "ltest" knows an entity named "ent2"
        And the GraphLink "ltest" knows an entity named "ent3"

    Scenario Outline: one2many coercion
        Given a new graph
        And there are 2 entities in graph
        And the cardinalities on entity 1 will be <n1>,<m1>
        And the cardinalities on entity 2 will be <n2>,<m2>
        When we link entities 1 and 2 together
        And we extract the association info from "ctest"
        Then the association is a one2many association
        And the one2many key is on <ent> and is nullable [<b>]

        Examples:
            |n1|m1|n2|m2|ent|b    |
            |0 |1 |0 |n |1  |true |
            |1 |1 |0 |n |1  |false|
            |0 |n |0 |1 |2  |true |
            |0 |n |1 |1 |2  |false|

    Scenario Outline: one2one coercion
        Given a new graph
        And there are 2 entities in graph
        And the cardinalities on entity 1 will be <n1>,<m1>
        And the cardinalities on entity 2 will be <n2>,<m2>
        When we link entities 1 and 2 together
        And we extract the association info from "ctest"
        Then the association is a one2one association
        And the key on 1 is nullable [<b1>]
        And the key on 2 is nullable [<b2>]

        Examples:
            |n1|m1|n2|m2|b1   |b2   |
            |0 |1 |0 |1 |true |true |
            |1 |1 |0 |1 |false|true |
            |0 |1 |1 |1 |true |false|
