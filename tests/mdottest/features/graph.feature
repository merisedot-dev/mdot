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
